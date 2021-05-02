//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use std::path::Path;
use structopt::clap::Shell;

include!("src/cli/opts.rs");

fn main() {
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let comp_dir = Path::new(&manifest_dir).join("..").join("shell-completion");
    let mut app = Opts::clap();
    for (shell, folder) in &[(Shell::Bash, "bash"), (Shell::Zsh, "zsh")] {
        app.gen_completions("monitaringu-rei", *shell, &comp_dir.join(folder));
    }
}
