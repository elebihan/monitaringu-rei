//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

mod opts;

use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use crate::error::Result;
use crate::logging;
use crate::supervisor::Supervisor;
use ctrlc;
use structopt::StructOpt;

use crate::cli::opts::Opts;

pub fn build_opts() -> Opts {
    Opts::from_args()
}

fn run_with_opts(opts: Opts) -> Result<()> {
    let directory = opts.directory.unwrap_or(env::current_dir()?);
    let mut supervisor = Supervisor::new(&opts.program_exe, &opts.program_args, directory);
    if let Some(pattern) = &opts.pattern {
        supervisor.pattern(pattern)?;
    }
    let should_quit = Arc::new(AtomicBool::new(false));
    let q = should_quit.clone();
    ctrlc::set_handler(move || {
        q.store(true, Ordering::Relaxed);
    })?;
    supervisor.run(should_quit, |path| println!("New file {}", path.display()))
}

pub fn run() -> Result<()> {
    logging::init();
    let opts = build_opts();
    run_with_opts(opts)
}
