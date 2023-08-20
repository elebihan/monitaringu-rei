//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use std::env;
use std::path::PathBuf;

use crate::{error::Result, pkginfo};

pub fn init() -> Result<()> {
    let mut path = if env::var("MESON_DEVENV").is_ok() {
        let mut path = env::current_exe()?;
        path.pop();
        path.push("data");
        path
    } else {
        let mut path = PathBuf::new();
        path.push(pkginfo::DATADIR);
        path
    };
    path.push("monitaringu-rei-gtk.gresource");
    let resources = gio::Resource::load(path)?;
    gio::resources_register(&resources);
    Ok(())
}
