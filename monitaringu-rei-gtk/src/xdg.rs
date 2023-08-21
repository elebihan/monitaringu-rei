//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use std::env;
use std::io;

use crate::error::Result;

pub fn fixup() -> Result<()> {
    let exe = std::env::current_exe()?;
    let root = exe
        .ancestors().nth(2)
        .ok_or(io::Error::new(io::ErrorKind::Other, "Invalid exe path"))?;
    let path = root.join("share");
    match env::var_os("XDG_DATA_DIRS") {
        Some(value) => {
            let mut paths = env::split_paths(&value).collect::<Vec<_>>();
            paths.push(path);
            let path = env::join_paths(paths)?;
            env::set_var("XDG_DATA_DIRS", path);
        }
        None => {
            env::set_var("XDG_DATA_DIRS", &path);
        }
    }
    Ok(())
}
