use crate::build::{ASSETS_FOLDER_NAME, BUILD_FOLDER_NAME};
use crate::error::{simple_error, ZagreusError};
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, TrySendError};
use std::thread;
use std::time::{Duration, Instant};

pub fn spawn(
    watch_path: PathBuf,
    recursive: bool,
    debounce_delay: Duration,
) -> Result<Receiver<()>, ZagreusError> {
    // Check whether watch path is absolute, required for path ignore checks.
    if !watch_path.is_absolute() {
        return simple_error("Watch path must be absolute");
    }

    // Set up file event channel.
    let (event_tx, event_rx) = mpsc::sync_channel(1);

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
            // Wait for first relevant event, break if an error is received.
            if let Err(error) = wait_for_relevant_event(&notify_rx, &watch_path) {
                error!("Terminating file watcher thread due to error: {:?}", error);
                break;
            }

            // Wait until no further relevant events have occurred for at least debounce_delay
            // amount of time, break if an error is received.
            if let Err(error) = debounce(&notify_rx, &watch_path, debounce_delay) {
                error!("Terminating file watcher thread due to error: {:?}", error);
                break;
            }

            // Try notifying receiver. Drop event if buffer is full, break if receiver has
            // disconnected.
            match event_tx.try_send(()) {
                Ok(_) | Err(TrySendError::Full(_)) => {}
                Err(TrySendError::Disconnected(_)) => {
                    // Terminate thread if rx has disconnected.
                    trace!("File watcher receiver disconnected, terminating watcher thread");
                    break;
                }
            }
        }
        trace!("File watcher thread terminated.")
    });

    Ok(event_rx)
}

fn wait_for_relevant_event(rx: &Receiver<RawEvent>, watch_path: &Path) -> Result<(), ZagreusError> {
    loop {
        let event = match rx.recv() {
            Ok(event) => event,
            Err(error) => return Err(error.into()),
        };
        match categorize_event(&event, watch_path) {
            EventCategory::Relevant => return Ok(()),
            EventCategory::Ignored => {}
            EventCategory::Err(error) => return Err(error),
        }
    }
}

fn debounce(
    rx: &Receiver<RawEvent>,
    watch_path: &Path,
    delay: Duration,
) -> Result<(), ZagreusError> {
    // Deadline for receiving further relevant events is (now + debounce delay). If no relevant
    // events are received past the deadline, debouncing is complete.
    let mut deadline = Instant::now().add(delay);

    // Time remaining is the difference between now and deadline, i.e. how much time there is left
    // until the deadline. Break if deadline is already reached or exceeded (i.e. now is later
    // than deadline).
    while let Some(time_remaining) = deadline.checked_duration_since(Instant::now()) {
        // Wait for an event. If timeout is reached, break. If sender disconnected, return Err.
        let event = match rx.recv_timeout(time_remaining) {
            Ok(event) => event,
            Err(RecvTimeoutError::Timeout) => break,
            Err(RecvTimeoutError::Disconnected) => {
                let msg = String::from("Notify sender has disconnected");
                return Err(ZagreusError::from(msg));
            }
        };

        // Event received, no timeout. Check if event is relevant. If it should be ignored, continue
        // loop without affecting the deadline. If it is relevant, reset the deadline (i.e. keep
        // debouncing). If error is received, return error.
        match categorize_event(&event, watch_path) {
            EventCategory::Ignored => {}
            EventCategory::Relevant => deadline = Instant::now().add(delay),
            EventCategory::Err(error) => return Err(error),
        }
    }

    // No relevant events seen for at least delay amount of time, return Ok.
    Ok(())
}

fn categorize_event(event: &RawEvent, watch_path: &Path) -> EventCategory {
    // Check whether the event represents an error, return Err if true.
    if let Err(error) = &event.op {
        return EventCategory::Err(error.into());
    }

    // Try extracting event path, skip if no path is available.
    let event_path = match event.path.as_ref() {
        Some(path) => path,
        None => {
            // Should not reach here - non-error events should always have a path.
            debug!("Event has no path, marking as ignored: {:?}", event);
            return EventCategory::Ignored;
        }
    };

    // Check whether event should be ignored. Skip if true, break on error.
    match should_ignore_event_path(watch_path, event_path) {
        Ok(true) => EventCategory::Ignored,
        Ok(false) => EventCategory::Relevant,
        Err(error) => EventCategory::Err(error),
    }
}

enum EventCategory {
    Relevant,
    Ignored,
    Err(ZagreusError),
}

fn should_ignore_event_path(root_path: &Path, event_path: &Path) -> Result<bool, ZagreusError> {
    // Make event path relative to root directory, return Err if event path is not below root
    // directory.
    let event_path = event_path.strip_prefix(root_path)?;

    // Never ignore assets directory.
    if event_path.starts_with(ASSETS_FOLDER_NAME) {
        // Assets directory, don't ignore.
        return Ok(false);
    }

    // Always ignore build directory.
    if event_path.starts_with(BUILD_FOLDER_NAME) {
        // Assets directory, don't ignore.
        return Ok(true);
    }

    // Event is not in build or assets dir: decide based on extension. Get event path's file
    // extension. If None, mark event as ignored (i.e. is a dir).
    let extension = match event_path.extension() {
        Some(extension) => extension,
        None => return Ok(true),
    };

    // Keep yaml and svg files, ignore everything else.
    if (extension == "yaml") || (extension == "yml") || (extension == "svg") {
        Ok(false)
    } else {
        Ok(true)
    }
}
