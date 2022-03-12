use std::ops::Add;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError, TrySendError};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::anyhow;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};

use crate::build::{ASSETS_FOLDER_NAME, BUILD_FOLDER_NAME};
use crate::error::simple_error;

/// Starts recursively watching the given path for file changes. Returns a `ZagreusError` if the
/// `watch_path` is not absolute, or if the file watcher can't be initialized or started.
/// Otherwise, returns a `Receiver<()>` which gets notified about filtered and debounced file
/// events.
///
/// # Arguments
/// * `watch_path` - The path to recursively watch for file changes, must be absolute.
/// * `debounce_delay` - Amount of time to wait after the last relevant file event, before notifying
///                      the receiver.
///
/// # Bounded Channel
/// The receiver's queue is guaranteed to always contain at most one item. If a new event occurs
/// while the queue already contains an item, the new event is dropped.
///
/// # Relay Thread
/// This function creates a new `notify` file watcher and spawns a relay thread which is responsible
/// for continuously polling the `notify` watcher for new file events. If an event is received, the
/// relay thread first decides whether the event is relevant (filtering), and then waits until no
/// further relevant events are received for at least `debounce_delay` amount of time (debouncing),
/// before notifying the receiver. Note that ignored events to not reset the debounce timer.
///
/// Whether or not an event should be ignored (i.e. is irrelevant) is decided as follows:
/// * If an event occurs in the `build` directory, it is ignored.
/// * Else, if it occurs in the `assets` directory, it is **not** ignored.
/// * Else, if it occurs on a `yml`, `yaml` or `svg` file, it is **not** ignored
/// * Else, it is ignored
///
/// # Termination
/// The relay thread and its underlying `notify` file watcher are terminated if:
/// * The relay thread receives an `Error` event from the `notify` file watcher
/// * The relay thread tries to notify a disconnected (i.e. dropped) receiver
/// * An unexpected error occurs in the relay thread while processing an event
pub fn spawn(watch_path: PathBuf, debounce_delay: Duration) -> anyhow::Result<Receiver<()>> {
    // Check whether watch path is absolute, required for path ignore checks.
    if !watch_path.is_absolute() {
        return simple_error("Watch path must be absolute");
    }

    // Set up file event channel.
    let (event_tx, event_rx) = mpsc::sync_channel(1);

    // Spawn a notify file watcher.
    let (notify_tx, notify_rx) = mpsc::channel();
    let mut watcher = raw_watcher(notify_tx)?;
    watcher.watch(&watch_path, RecursiveMode::Recursive)?;

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

/// Blocks until an event is received and categorized as `EventCategory::Relevant`. Immediately
/// returns `Err` if an event gets categorized as `Err`.
fn wait_for_relevant_event(rx: &Receiver<RawEvent>, watch_path: &Path) -> anyhow::Result<()> {
    loop {
        let event = match rx.recv() {
            Ok(event) => event,
            Err(error) => return Err(error.into()),
        };
        match categorize_event(&event, watch_path) {
            EventCategory::Relevant => return Ok(()),
            EventCategory::Ignored => {}
            EventCategory::Err(error) => {
                return Err(anyhow!(
                    "Error occurred when watching for file changes: {}",
                    error
                ))
            }
        }
    }
}

/// Blocks until no event has been received and categorized as `EventCategory::Relevant` for at
/// least `debounce_delay` amount of time. Returns `Ok` as soon as this timeout is reached.
/// Immediately returns `Err` if an event gets categorized as `Err`.
fn debounce(rx: &Receiver<RawEvent>, watch_path: &Path, delay: Duration) -> anyhow::Result<()> {
    // Deadline for receiving further relevant events is (now + debounce delay). If no relevant
    // events are received past the deadline, debouncing is complete.
    let mut deadline = Instant::now().add(delay);

    // Time remaining is the difference between now and the deadline, i.e. how much time is left
    // until the deadline. Break if deadline is already reached or exceeded (i.e. now is later
    // than deadline).
    while let Some(time_remaining) = deadline.checked_duration_since(Instant::now()) {
        // Wait for an event. If timeout is reached, break. If sender disconnected, return Err.
        let event = match rx.recv_timeout(time_remaining) {
            Ok(event) => event,
            Err(RecvTimeoutError::Timeout) => break,
            Err(RecvTimeoutError::Disconnected) => {
                let msg = String::from("Notify sender has disconnected");
                return Err(anyhow!(msg));
            }
        };

        // Event received, no timeout occurred. Check if event is relevant. If it should be ignored,
        // continue loop without affecting the deadline. If it is relevant, reset the deadline (i.e.
        // keep debouncing). If error is received, return error.
        match categorize_event(&event, watch_path) {
            EventCategory::Ignored => {}
            EventCategory::Relevant => deadline = Instant::now().add(delay),
            EventCategory::Err(error) => {
                return Err(anyhow!(
                    "Error occurred when watching for file change event: {}",
                    error
                ))
            }
        }
    }

    // No relevant events seen for at least delay amount of time, return Ok.
    Ok(())
}

/// Assigns an `EventCategory` variant to a given event.
fn categorize_event(event: &RawEvent, watch_path: &Path) -> EventCategory {
    // Check whether the event represents an error, return Err if true.
    if let Err(error) = &event.op {
        return EventCategory::Err(format!("Got file change error event: {}.", error));
    }

    // Try extracting event path, skip if no path is available.
    let event_path = match event.path.as_ref() {
        Some(path) => path,
        None => {
            // Should not reach here - non-error events should always have a path.
            warn!("Event has no path, marking as ignored: {:?}", event);
            return EventCategory::Ignored;
        }
    };

    // Check whether event should be ignored. Skip if true, break on error.
    match should_ignore_event_path(watch_path, event_path) {
        Ok(true) => EventCategory::Ignored,
        Ok(false) => EventCategory::Relevant,
        Err(error) => EventCategory::Err(format!(
            "Could not determine whether to ignore event path: {}",
            error
        )),
    }
}

/// Represents whether a given event is relevant, irrelevant (ignored), or represents an error.
enum EventCategory {
    /// This event is relevant, the receiver should notified.
    Relevant,

    /// This event can be ignored, the receiver should **not** be notified.
    Ignored,

    /// This event represents a `notify` error or otherwise can't be processed.
    Err(String),
}

/// Determines whether or not a given event path should be ignored, based on a predefined ruleset.
/// Returns `true` if an event at that path should be ignored, `false` if it should not be ignored,
/// and `Err` if the given `event_path` is not below the `root_path`.
fn should_ignore_event_path(root_path: &Path, event_path: &Path) -> anyhow::Result<bool> {
    // Make event path relative to root directory, return Err if event path is not below root
    // directory.
    let event_path = event_path.strip_prefix(root_path)?;

    // Never ignore assets directory.
    if event_path.starts_with(ASSETS_FOLDER_NAME) {
        return Ok(false);
    }

    // Always ignore build directory.
    if event_path.starts_with(BUILD_FOLDER_NAME) {
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
