use std::ops::Add;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::{Duration, Instant};

use anyhow::{anyhow, Context};
use notify::{recommended_watcher, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

use crate::build::{ASSETS_FOLDER_NAME, BUILD_FOLDER_NAME};
use crate::error::simple_error;

const FILE_WATCHER_DEBOUNCE_DELAY: Duration = Duration::from_millis(200);

pub struct FileWatcher {
    // watcher must not be dropped
    #[allow(dead_code)]
    watcher: RecommendedWatcher,
    receiver: Receiver<Event>,
    watch_path: PathBuf,
}

impl FileWatcher {
    /// Creates a file watcher that starts recursively watching the given path for file changes. Returns an Error if the
    /// `watch_path` is not absolute, or if the file watcher can't be initialized or started.
    /// Otherwise, returns a `FileWatcher`.
    ///
    /// # Arguments
    /// * `watch_path` - The path to recursively watch for file changes, must be absolute.
    ///
    /// # Bounded Channel
    /// The receiver's queue is guaranteed to always contain at most one item. If a new event occurs
    /// while the queue already contains an item, the new event is dropped.
    ///
    pub fn new(watch_path: PathBuf) -> anyhow::Result<Self> {
        // Check whether watch path is absolute, required for path ignore checks.
        if !watch_path.is_absolute() {
            return simple_error("Watch path must be absolute");
        }

        let (notify_tx, notify_rx) = mpsc::channel::<Event>();
        // Spawn a notify file watcher.
        let mut watcher = recommended_watcher(move |res| match res {
            Ok(evt) => {
                if let Err(err) = notify_tx.send(evt) {
                    error!("Could not send file change event: {}.", err);
                }
            }
            Err(err) => {
                error!(
                    "Error occurred when receiving file change watch event: {}.",
                    err
                );
            }
        })?;
        watcher.watch(&watch_path, RecursiveMode::Recursive)?;

        Ok(FileWatcher {
            receiver: notify_rx,
            watch_path,
            watcher,
        })
    }

    /// Wait for a relevant file change
    /// If an event is received, the file watcher first decides whether the event is relevant (filtering),
    /// and then waits until no further relevant events are received for at least `FILE_WATCHER_DEBOUNCE_DELAY`
    /// amount of time (debouncing) before returning. Note that ignored events to not reset the debounce timer.
    ///
    /// Whether or not an event should be ignored (i.e. is irrelevant) is decided as follows:
    /// * If an event occurs in the `build` directory, it is ignored.
    /// * Else, if it occurs in the `assets` directory, it is **not** ignored.
    /// * Else, if it occurs on a `yml`, `yaml` or `svg` file, it is **not** ignored
    /// * Else, it is ignored
    pub fn wait_for_file_change(&self) -> anyhow::Result<()> {
        // Wait for first relevant event, return error is received.
        self.wait_for_relevant_event()
            .context("Could not wait for relevant event.")?;

        // Wait until no further relevant events have occurred for at least debounce_delay
        // amount of time, return error if one is received.
        self.debounce().context("Could not debounce file event.")?;

        Ok(())
    }

    /// Blocks until an event is received and categorized as `EventCategory::Relevant`. Immediately
    /// returns `Err` if an event gets categorized as `Err`.
    fn wait_for_relevant_event(&self) -> anyhow::Result<()> {
        loop {
            let event = match self.receiver.recv() {
                Ok(event) => event,
                Err(error) => return Err(error.into()),
            };
            match self.categorize_event(&event) {
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
    fn debounce(&self) -> anyhow::Result<()> {
        // Deadline for receiving further relevant events is (now + debounce delay). If no relevant
        // events are received past the deadline, debouncing is complete.
        let mut deadline = Instant::now().add(FILE_WATCHER_DEBOUNCE_DELAY);

        // Time remaining is the difference between now and the deadline, i.e. how much time is left
        // until the deadline. Break if deadline is already reached or exceeded (i.e. now is later
        // than deadline).
        while let Some(time_remaining) = deadline.checked_duration_since(Instant::now()) {
            // Wait for an event. If timeout is reached, break. If sender disconnected, return Err.
            let event = match self.receiver.recv_timeout(time_remaining) {
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
            match self.categorize_event(&event) {
                EventCategory::Ignored => {}
                EventCategory::Relevant => {
                    deadline = Instant::now().add(FILE_WATCHER_DEBOUNCE_DELAY)
                }
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
    fn categorize_event(&self, event: &Event) -> EventCategory {
        if should_ignore_event_kind(&event.kind) {
            return EventCategory::Ignored;
        }

        // Try extracting event path, skip if no path is available.
        let event_path = match event.paths.first() {
            Some(path) => path,
            None => {
                // Should not reach here - non-error events should always have a path.
                warn!("Event has no path, marking as ignored: {:?}", event);
                return EventCategory::Ignored;
            }
        };

        // Check whether event should be ignored. Skip if true, break on error.
        match should_ignore_event_path(&self.watch_path, event_path) {
            Ok(true) => EventCategory::Ignored,
            Ok(false) => EventCategory::Relevant,
            Err(error) => EventCategory::Err(format!(
                "Could not determine whether to ignore event path: {error}"
            )),
        }
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

/// Determines whether or not a given event should be ignored based on its kind.
/// Returns `true` if the event kind should be ignored, `false`otherwise.
fn should_ignore_event_kind(event_kind: &EventKind) -> bool {
    !matches!(
        event_kind,
        EventKind::Any | EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
    )
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
