//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),
    #[error("GLib error: {0}")]
    GLib(#[from] glib::Error),
    #[error("Env error: {0}")]
    Env(#[from] std::env::JoinPathsError),
}

pub type Result<T> = std::result::Result<T, Error>;
