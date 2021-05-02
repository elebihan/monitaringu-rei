//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

mod error;
mod logging;

pub mod cli;
pub use crate::error::*;
pub mod supervisor;
