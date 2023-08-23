//
// This file is part of monitaringu-rei
//
// Copyright (C) 2021 Eric Le Bihan <eric.le.bihan.dev@free.fr>
//
// SPDX-License-Identifier: MIT
//

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

fn build_res(input: &Path) -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let windres = env::var("WINDRES").unwrap_or("windres".to_string());
    let objpath = PathBuf::from(&out_dir).join("winresource.o");
    let status = Command::new(windres)
        .arg(format!("{}", input.display()))
        .arg(format!("{}", objpath.display()))
        .status()?;
    if !status.success() {
        return Err(String::from("Failed to compile resource file").into());
    }
    let ar = env::var("AR").unwrap_or("ar".to_string());
    let libpath = PathBuf::from(&out_dir).join("libwinresource.a");
    let status = Command::new(ar)
        .arg("crus")
        .arg(format!("{}", libpath.display()))
        .arg(format!("{}", objpath.display()))
        .status()?;
    if !status.success() {
        return Err(String::from("Failed to create static library for resource").into());
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-lik-lib=static=winresource");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    if let Ok("windows") = target_os.as_ref().map(|x| &**x) {
        let input = PathBuf::from("src").join("app.rc");
        build_res(&input)?;
    }
    Ok(())
}
