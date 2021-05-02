//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use anyhow::Result;
use monitaringu_rei_core::cli::run;

pub fn main() -> Result<()> {
    run()?;
    Ok(())
}
