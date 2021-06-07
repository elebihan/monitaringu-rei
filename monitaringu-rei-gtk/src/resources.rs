//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use std::path::PathBuf;

use crate::{error::Result, pkginfo};

pub fn init() -> Result<()> {
    let mut path = PathBuf::new();
    path.push(pkginfo::DATADIR);
    path.push("monitaringu-rei-gtk.gresource");
    let resource = gio::Resource::load(path)?;
    gio::resources_register(&resource);
    Ok(())
}
