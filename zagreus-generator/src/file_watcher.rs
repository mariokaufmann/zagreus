use crate::build::{ASSETS_FOLDER_NAME, BUILD_FOLDER_NAME};
use crate::error::{error_with_message, simple_error, ZagreusError};
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{RecvTimeoutError, Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

/*
TODO:
  - Doc comments
  - Is there anything we could test?
 */

pub fn spawn(watch_path: PathBuf, recursive: bool) -> Result<FileWatcherHandle, ZagreusError> {
    if !watch_path.is_absolute() {
        // Watch path must be absolute, required for path ignore checks.
        return simple_error("Watch path must be absolute");
    }

    // Set up file watcher handle.
    let (terminate_watcher_tx, terminate_watcher_rx) = mpsc::channel();
    let file_watcher_tx = OptionalSender::from_none();
    let file_watcher_handle = FileWatcherHandle::new(
        terminate_watcher_tx,
        OptionalSender::clone(&file_watcher_tx),
    );

    // Spawn a notify file watcher.
    let (notify_tx, notify_rx) = mpsc::channel();
    let mut watcher = raw_watcher(notify_tx)?;
    let recursive_mode = match recursive {
        true => RecursiveMode::Recursive,
        false => RecursiveMode::NonRecursive,
    };
    watcher.watch(&watch_path, recursive_mode)?;

    // Spawn the file event relay thread.
    thread::spawn(move || {
        // Make sure the watcher is moved into the thread, so it won't get dropped.
        let _ = watcher;

        loop {
            // If a message was received or the sender has disconnected on the terminate_watcher
            // channel, break the loop and terminate the thread.
            match terminate_watcher_rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => {}
            }

            // Wait for the next file event from notify.
            let event = match notify_rx.recv() {
                Ok(event) => event,
                Err(error) => {
                    // Terminate thread if there is a RecvError.
                    error!("Terminating file watcher thread due to error: {:?}", error);
                    break;
                }
            };

            // Check whether the event represents an error, break if true.
            if let Err(error) = &event.op {
                error!(
                    "Terminating file watcher thread due to notify error: {:?}",
                    error
                );
                break;
            }

            // Try extracting event path, skip if no path is available.
            let event_path = match event.path.as_ref() {
                Some(path) => path,
                None => {
                    // Should not reach here - non-error events should always have a path.
                    debug!("Ignoring event without path: {:?}", event);
                    continue;
                }
            };

            // Check whether event should be ignored. Skip if true, break on error.
            match should_ignore_event_path(&watch_path, event_path) {
                Ok(true) => continue,
                Ok(false) => {}
                Err(error) => {
                    // Error occurs if event path is not at or below watch path.
                    error!("Terminating file watcher thread due to error: {:?}", error);
                    break;
                }
            };

            if let Err(error) = file_watcher_tx.try_send_or_reset(event) {
                // Terminate thread if there is an error.
                error!("Terminating file watcher thread due to error: {:?}", error);
                break;
            }
        }
        trace!("Notify watcher thread terminated.")
    });

    Ok(file_watcher_handle)
}

fn should_ignore_event_path(root_path: &Path, event_path: &Path) -> Result<bool, ZagreusError> {
    // Make event path relative to root directory.
    let event_path = event_path.strip_prefix(root_path)?;

    // Never ignore assets directory.
    if event_path.starts_with(ASSETS_FOLDER_NAME) {
        // Assets directory, don't ignore.
        return Ok(false);
    }

    // Always ignore build dir.
    if event_path.starts_with(BUILD_FOLDER_NAME) {
        // Assets directory, don't ignore.
        return Ok(true);
    }

    // Get event path file extension. If no extension is available, ignore event (i.e. is a dir).
    let extension = match event_path.extension() {
        Some(extension) => extension,
        None => return Ok(true),
    };

    // Not in build or assets dir. Keep yaml and svg files, ignore everything else.
    if (extension == "yaml") || (extension == "yml") || (extension == "svg") {
        Ok(false)
    } else {
        Ok(true)
    }
}

struct OptionalSender<T>(Arc<Mutex<Option<Sender<T>>>>);

impl<T> OptionalSender<T> {
    fn new(tx_option: Option<Sender<T>>) -> Self {
        OptionalSender(Arc::new(Mutex::new(tx_option)))
    }

    fn from_none() -> Self {
        OptionalSender::new(None)
    }

    fn lock(&self) -> Result<MutexGuard<Option<Sender<T>>>, ZagreusError> {
        match self.0.lock() {
            Err(error) => error_with_message("Failed to lock optional sender", error),
            Ok(inner) => Ok(inner),
        }
    }

    fn set(&self, value: Option<Sender<T>>) -> Result<(), ZagreusError> {
        let mut inner = self.lock()?;
        *inner = value;
        Ok(())
    }

    fn try_send_or_reset(&self, value: T) -> Result<(), ZagreusError> {
        let mut sender_guard = self.lock()?;

        // Check whether sender is Some, return Ok if not.
        let tx = match &*sender_guard {
            Some(tx) => tx,
            None => return Ok(()),
        };

        // Try sending value, return Ok if Ok.
        if tx.send(value).is_ok() {
            return Ok(());
        }

        // Sending failed, reset sender to None.
        *sender_guard = None;
        Ok(())
    }
}

impl<T> Clone for OptionalSender<T> {
    fn clone(&self) -> Self {
        OptionalSender(Arc::clone(&self.0))
    }

    fn clone_from(&mut self, source: &Self) {
        self.0 = Arc::clone(&source.0)
    }
}

pub struct FileWatcherHandle {
    _terminate_watcher: Sender<()>,
    file_watcher_tx: OptionalSender<RawEvent>,
}

impl FileWatcherHandle {
    fn new(terminate_tx: Sender<()>, file_watcher_tx: OptionalSender<RawEvent>) -> Self {
        FileWatcherHandle {
            _terminate_watcher: terminate_tx,
            file_watcher_tx,
        }
    }
}

/// Block until (1) at least one file system event (create, delete,
/// write, etc.) has occurred in the working directory, but (2) no events have occurred for `delay`
/// amount of time. As soon as conditions (1) and (2) are met, return.
pub fn await_file_event(
    watcher_handle: &FileWatcherHandle,
    debounce_delay: Duration,
) -> Result<(), ZagreusError> {
    let (tx, rx) = mpsc::channel();
    watcher_handle.file_watcher_tx.set(Some(tx))?;

    // Wait for a first event, or return error.
    rx.recv()?;

    // Wait until recv_timeout no longer returns Ok(). Then, return Ok() if a timeout occurred,
    // or Err() if the file watcher has disconnected.
    loop {
        match rx.recv_timeout(debounce_delay) {
            Ok(_) => {}
            Err(RecvTimeoutError::Disconnected) => {
                return simple_error("File watcher disconnected unexpectedly")
            }
            Err(RecvTimeoutError::Timeout) => return Ok(()),
        }
    }
}
