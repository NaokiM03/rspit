use std::{env, fs, path::Path, process};

use anyhow::{bail, Result};
use tiny_ansi::TinyAnsi;

use crate::{build, cache, package::Package};

fn execute(package_name: &str) -> Result<()> {
    let execute_path = env::temp_dir()
        .join("pit")
        .join(&package_name)
        .join("target")
        .join("debug")
        .join(package_name);
    let exit_status = process::Command::new(execute_path).spawn()?.wait()?;

    if !exit_status.success() {
        bail!("Failed to execute.");
    }

    Ok(())
}

fn run(package: &Package, quiet: bool) -> Result<()> {
    if !quiet {
        println!(
            "{}",
            &format!("Run {} package", &package.name)
                .bright_green()
                .bold()
        );
    }

    // If there is no change in iether src or toml, use the executable file on the cache.
    if cache::check_identity_hash(&package).is_some() {
        return execute(&package.name);
    }

    build::build(&package, false, quiet)?;
    execute(&package.name)?;

    Ok(())
}

pub(crate) fn run_specified_package<P>(file_path: P, package: &str, quiet: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;

    let package = src
        .split("//# ---")
        .map(|x| Package::from(x))
        .filter(|x| x.name == package)
        .next()
        .expect("Package not found in file.");
    run(&package, quiet)?;

    Ok(())
}

pub(crate) fn run_all<P>(file_path: P, quiet: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;

    for package in src.split("//# ---") {
        let package = Package::from(package);
        run(&package, quiet)?;
    }

    Ok(())
}
