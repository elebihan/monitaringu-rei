//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "monitaringu-rei")]
pub struct Opts {
    #[structopt(
        long = "directory",
        short = "D",
        name = "PATH",
        help = "Directory to monitor",
        parse(from_os_str)
    )]
    pub directory: Option<PathBuf>,
    #[structopt(
        long = "pattern",
        short = "E",
        name = "EXPRESSION",
        help = "File name pattern"
    )]
    pub pattern: Option<String>,
    #[structopt(name = "PROGRAM", help = "Program to execute and monitor")]
    pub program_exe: String,
    #[structopt(name = "ARGUMENT", help = "Program argument(s)")]
    pub program_args: Vec<String>,
}
