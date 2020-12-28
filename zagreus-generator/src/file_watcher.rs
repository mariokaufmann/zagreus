use crate::error::{error_with_message, simple_error, ZagreusError};
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{RecvTimeoutError, Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use globset::{GlobSetBuilder, Glob};
use std::env;

/*
TODO:
  - Implement a simple filter API to avoid forwarding events for e.g. the build dir
  - See if there are any raw event types we need to filter out. Maybe take list of watched events
    as an input to the wait_for_update() function?
  - Doc comments
  - Is there anything we could test?
 */

pub fn spawn(watch_path: &Path, recursive: bool) -> Result<FileWatcherHandle, ZagreusError> {
    // TODO: Consider building from args.
    let mut globset_builder = GlobSetBuilder::new();
    globset_builder.add(Glob::new("build/**").unwrap());
    globset_builder.add(Glob::new("*.afdesign").unwrap());
    let globset = globset_builder.build().unwrap();

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
    watcher.watch(watch_path, recursive_mode)?;

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

            // TODO: Refactor, avoid using unwrap.
            let pwd = env::current_dir().unwrap();
            let event_path = event.path.as_ref().unwrap();
            let rel_event_path = event_path.strip_prefix(pwd).unwrap();
            if globset.is_match(rel_event_path) {
                continue;
            }

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
