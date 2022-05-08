//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

#![windows_subsystem = "windows"]

use std::error;
use std::result::Result;

use gio::prelude::*;

use monitaringu_rei_gtk::{app::Application, i18n, resources, xdg};

fn main() -> Result<(), Box<dyn error::Error>> {
    xdg::fixup()?;
    resources::init()?;
    i18n::init();

    let app = Application::new();
    app.run();

    Ok(())
}
