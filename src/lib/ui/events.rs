use anyhow::{Error, Result};
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;
use std::thread::{sleep, spawn as spawn_thread, JoinHandle};
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;

use crate::anyhowize;

const TICK_DEFAULT_DURATION: u64 = 250;

pub enum Event<T> {
    Input(T),
    Tick,
}

/// An `EventHandler` for keys pressed during consumption
/// When creating an instance of this `EventHandler` two
/// threads will spawn.
///
/// The first thread will be the `input_handle` which is in
/// charge of reading keys pressed while this `EventHandler` is
/// running
///
/// The second thread is the `tick_handle` which takes care of
/// polling the events channel
pub struct EventHandler {
    ignore_exit_key: Arc<AtomicBool>,
    #[allow(dead_code)]
    input_handle: JoinHandle<()>,
    rx: Receiver<Event<Key>>,
    #[allow(dead_code)]
    tick_handle: JoinHandle<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (event_tx, event_rx) = channel();
        let ignore_exit_key = Arc::new(AtomicBool::new(false));
        let input_handle = {
            let tx = event_tx.clone();
            let ignore_exit_key = ignore_exit_key.clone();

            spawn_thread(move || {
                let stdin = io::stdin();

                for event in stdin.keys() {
                    if let Ok(key) = event {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }

                        if !ignore_exit_key.load(Ordering::Relaxed) && key == Key::Char('q') {
                            return;
                        }
                    }
                }
            })
        };

        let tick_handle = spawn_thread(move || loop {
            if event_tx.send(Event::Tick).is_err() {
                break;
            }

            sleep(Duration::from_millis(TICK_DEFAULT_DURATION));
        });

        Self {
            ignore_exit_key,
            input_handle,
            rx: event_rx,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key>> {
        self.rx.recv().map_err(|e| anyhowize!(e))
    }

    pub fn disable_exit_key(&mut self) {
        self.ignore_exit_key.store(true, Ordering::Relaxed);
    }

    pub fn enable_exit_key(&mut self) {
        self.ignore_exit_key.store(false, Ordering::Relaxed);
    }
}
