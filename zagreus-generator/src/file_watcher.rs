use crate::error::ZagreusError;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, raw_watcher};
use std::sync::mpsc;
use std::path::Path;

/// Create a new file watcher. Block until (1) at least one file system event (create, delete,
/// write, etc.) has occurred in the working directory, but (2) no events have occurred for `delay`
/// amount of time. As soon as conditions (1) and (2) are met, return.
pub fn wait_for_update(delay: Duration) -> Result<(), ZagreusError> {
    // TODO:
    //  - Proper error handling: make sure to either return or expect on receive and notify errors.
    //  - Consider trying to filter the receiver to ignore events in the build directory.
    let (tx, rx) = mpsc::channel();
    let mut watcher = raw_watcher(tx)?;
    watcher.watch(Path::new(""), RecursiveMode::Recursive)?;
    let _ = rx.recv().unwrap();
    while rx.recv_timeout(delay).is_ok() {
        // Do nothing.
    }
    // Error (hopefully not) or timeout (hopefully) has occurred, done.
    Ok(())
}