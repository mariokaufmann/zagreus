use crate::error::ZagreusError;
use notify::{raw_watcher, RawEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn listen() -> Result<FileWatcherHandle, ZagreusError> {
    // Set up file watcher handle.
    let is_terminated = Arc::new(AtomicBool::new(false));
    let output_tx: OptionalSender<RawEvent> = Arc::new(Mutex::new(None));
    let handle = FileWatcherHandle::new(Arc::clone(&is_terminated), Arc::clone(&output_tx));

    // Spawn a notify file watcher.
    let (notify_tx, notify_rx) = mpsc::channel();
    let mut watcher = raw_watcher(notify_tx)?;
    watcher.watch(Path::new(""), RecursiveMode::Recursive)?;

    // Spawn the relay thread.
    thread::spawn(move || {
        // Make sure the watcher is moved into the thread, so it won't get dropped.
        let _ = watcher;
        while !is_terminated.load(Acquire) {
            let event = notify_rx.recv().unwrap();
            let sender = output_tx.lock().expect("Unable to lock output sender");
            match &*sender {
                Some(tx) => tx.send(event).unwrap(),
                None => {}
            }
        }
        info!("File watcher terminated.")
    });

    Ok(handle)
}

/// Block until (1) at least one file system event (create, delete,
/// write, etc.) has occurred in the working directory, but (2) no events have occurred for `delay`
/// amount of time. As soon as conditions (1) and (2) are met, return.
pub fn wait_for_update(handle: &FileWatcherHandle, debounce_delay: Duration) {
    let tx_handle = handle.tx();
    let (tx, rx) = mpsc::channel();
    {
        let mut tx_option = tx_handle.lock().expect("Unable to lock handle");
        *tx_option = Some(tx);
    }
    let _ = rx.recv();
    while rx.recv_timeout(debounce_delay).is_ok() {}
    {
        let mut tx_option = tx_handle.lock().expect("Unable to lock handle");
        *tx_option = None;
    }
}

type OptionalSender<T> = Arc<Mutex<Option<Sender<T>>>>;

pub struct FileWatcherHandle {
    is_terminated: Arc<AtomicBool>,
    tx: OptionalSender<RawEvent>,
}

impl FileWatcherHandle {
    fn new(is_terminated: Arc<AtomicBool>, tx: OptionalSender<RawEvent>) -> Self {
        FileWatcherHandle { is_terminated, tx }
    }

    pub(super) fn tx(&self) -> OptionalSender<RawEvent> {
        Arc::clone(&self.tx)
    }

    #[allow(dead_code)]
    pub fn terminate(&self) {
        self.is_terminated.store(true, Release);
    }
}
