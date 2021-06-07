//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use glib::Sender;
use std::error::Error;
use std::path::PathBuf;
use std::result::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use monitaringu_rei_core::supervisor::Supervisor as SupervisorInner;

#[derive(Debug, Clone)]
pub enum Message {
    FileCreated(PathBuf),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub command: String,
    pub directory: String,
    pub pattern: String,
}

pub struct Supervisor {
    worker: Option<thread::JoinHandle<()>>,
    should_stop: Arc<AtomicBool>,
}

impl Supervisor {
    pub fn spawn(
        settings: Settings,
        sender: Sender<Message>,
    ) -> Result<Supervisor, Box<dyn Error>> {
        log::info!("Spawning {:?}", settings);
        let should_stop = Arc::new(AtomicBool::new(false));
        let s = should_stop.clone();
        // FIXME: handle quoted arguments
        let args: Vec<String> = settings.command.split(' ').map(String::from).collect();
        let mut inner = SupervisorInner::new(&args[0], &args[1..], settings.directory);
        if !settings.pattern.is_empty() {
            inner.pattern(&settings.pattern)?;
        }
        let worker = thread::spawn(move || {
            if let Err(e) = inner.run(s, |path| {
                sender
                    .send(Message::FileCreated(path.to_path_buf()))
                    .expect("Failed to send result");
            }) {
                sender
                    .send(Message::Error(e.to_string()))
                    .expect("Failed to send error")
            }
        });
        Ok(Supervisor {
            worker: Some(worker),
            should_stop,
        })
    }
    pub fn kill(self) -> Result<(), Box<dyn Error>> {
        if let Some(worker) = self.worker {
            log::info!("Killing");
            self.should_stop.store(true, Ordering::SeqCst);
            worker.join().expect("Failed to kill worker");
        }
        Ok(())
    }
}
