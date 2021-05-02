//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use pretty_env_logger;
use std::env;

pub(crate) fn init() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "monitaringi-rei-core=error");
    }
    pretty_env_logger::init();
}
