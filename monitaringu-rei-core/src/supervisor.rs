//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use log;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::mpsc::channel;
use std::sync::{atomic::AtomicBool, Arc};
use std::{convert::AsRef, time::Duration};
use std::{ffi::OsStr, sync::atomic::Ordering};

use crate::error::Result;

pub struct Supervisor {
    program: Command,
    directory: PathBuf,
    pattern: Option<Regex>,
}

impl Supervisor {
    pub fn new<S, P>(exe: &S, args: &[S], directory: P) -> Self
    where
        P: AsRef<Path>,
        S: AsRef<OsStr>,
    {
        let mut program = Command::new(exe);
        program.args(args);
        Supervisor {
            program,
            directory: PathBuf::from(directory.as_ref()),
            pattern: None,
        }
    }

    pub fn pattern(&mut self, pattern: &str) -> Result<()> {
        let regex = Regex::new(pattern)?;
        self.pattern = Some(regex);
        Ok(())
    }

    pub fn run<F>(&mut self, should_quit: Arc<AtomicBool>, func: F) -> Result<()>
    where
        F: Fn(&Path),
    {
        let relax_delay = Duration::from_secs(1);
        let (tx, rx) = channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;
        watcher.watch(&self.directory, RecursiveMode::NonRecursive)?;
        log::info!("Watching directory {}", self.directory.display());
        let mut child = self.program.spawn()?;
        log::info!("Started child process {}", child.id());
        loop {
            if should_quit.load(Ordering::SeqCst) {
                break;
            }

            if let Ok(Some(status)) = child.try_wait() {
                log::info!("Child process has exited with status {}", status);
                break;
            }

            if let Ok(DebouncedEvent::Create(path)) = rx.try_recv() {
                if let Some(file_name) = path.file_name() {
                    match file_name.to_str() {
                        Some(file_name) => {
                            let is_valid = match &self.pattern {
                                Some(pattern) => pattern.is_match(file_name),
                                None => true,
                            };
                            if is_valid {
                                func(&path);
                            }
                        }
                        None => log::warn!("Skipping invalid file name"),
                    }
                }
            }
            std::thread::sleep(relax_delay);
        }

        match child.kill() {
            Ok(_) => match child.wait() {
                Ok(status) => {
                    log::info!("Child process exited with status {}", status);
                }
                Err(_) => {
                    log::error!("Failed to get child process status");
                }
            },
            Err(err) => {
                if err.kind() == io::ErrorKind::InvalidInput {
                    log::info!("Child process has already exited");
                } else {
                    log::error!("Failed to kill child process");
                }
            }
        };
        Ok(())
    }
}
