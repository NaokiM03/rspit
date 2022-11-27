use std::{env, fs, path::Path, process};

use anyhow::{bail, Result};
use tiny_ansi::TinyAnsi;

use crate::{build, cache::Identity, package::Package};

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
    println!(
        "{}",
        &format!("Start {} package", &package.name)
            .bright_green()
            .bold()
    );

    let cache_dir = env::temp_dir().join("pit").join(&package.name);

    let identity = package.gen_identity();
    let cache_identity_path = cache_dir.join("identity_hash.toml");
    // If there is no change in iether src or toml, use the executable file on the cache.
    if let Ok(cache_identity) = fs::read(&cache_identity_path) {
        let cache_identity: Identity = toml::from_slice(&cache_identity)?;
        if identity.hash == cache_identity.hash {
            let cache_execute_path = cache_dir.join("target").join("debug").join(&package.name);
            let exit_status = process::Command::new(cache_execute_path).spawn()?.wait()?;

            if exit_status.success() {
                return Ok(());
            }
        }
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
