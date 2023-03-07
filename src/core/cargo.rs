use std::{path::Path, process};

use anyhow::{bail, Result};

pub(super) fn check<P: AsRef<Path>>(package_dir: P, quiet: bool) -> Result<()> {
    let mut command = process::Command::new("cargo");
    command.arg("check");
    if quiet {
        command.arg("--quiet");
    }
    let exit_status = command.current_dir(&package_dir).spawn()?.wait()?;

    if !exit_status.success() {
        bail!("Failed to check.");
    }

    Ok(())
}

pub(super) fn build<P: AsRef<Path>>(package_dir: P, release: bool, quiet: bool) -> Result<()> {
    let mut command = process::Command::new("cargo");
    command.arg("build");
    if release {
        command.arg("--release");
    }
    if quiet {
        command.arg("--quiet");
    }
    let exit_status = command.current_dir(&package_dir).spawn()?.wait()?;

    if !exit_status.success() {
        bail!("Failed to build.");
    }

    Ok(())
}
