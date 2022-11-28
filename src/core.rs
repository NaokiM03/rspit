use std::{env, fs, path::Path, process};

use anyhow::{bail, Result};
use tempdir::TempDir;
use tiny_ansi::TinyAnsi;

mod cache;
mod package;

pub(crate) use package::Package;

// Build

fn create_toml(package_dir: &Path, toml: &str) -> Result<()> {
    let toml_file = package_dir.join("Cargo.toml");
    fs::write(toml_file, toml.as_bytes())?;

    Ok(())
}

fn create_src(package_dir: &Path, src: &str) -> Result<()> {
    let src_dir = package_dir.join("src");
    fs::create_dir(&src_dir)?;
    let src_file = src_dir.join("main.rs");
    fs::write(src_file, src.as_bytes())?;

    Ok(())
}

fn cargo_build(package_dir: &Path, release: bool, quiet: bool) -> Result<()> {
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

pub(crate) fn build(package: &Package, release: bool, quiet: bool) -> Result<()> {
    if !quiet {
        println!(
            "{}",
            &format!("Build {} package", &package.name)
                .bright_green()
                .bold()
        );
    }

    let package_dir = TempDir::new("pit")?.path().join(&package.name);
    fs::create_dir_all(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;

    let package_target_dir = package_dir.join("target");
    let cache_target_dir = cache::cache_dir(&package.name).join("target");

    cache::restore(&cache_target_dir, &package_target_dir)?;
    cargo_build(&package_dir, release, quiet)?;
    cache::store(&package_target_dir, &cache_target_dir)?;

    // The hash is not stored at release build time.
    // This is because release build is only used with the `release` command.
    if release {
        return Ok(());
    }

    cache::write_identity_hash(&package)?;

    Ok(())
}

// Run

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

pub(crate) fn run(package: &Package, quiet: bool) -> Result<()> {
    if !quiet {
        println!(
            "{}",
            &format!("Run {} package", &package.name)
                .bright_green()
                .bold()
        );
    }

    // If there is no change in either src or toml, use the executable file on the cache.
    if cache::check_identity_hash(&package).is_some() {
        return execute(&package.name);
    }

    build(&package, false, quiet)?;
    execute(&package.name)?;

    Ok(())
}

// Release

fn distribute<P: AsRef<Path>>(package_name: &str, out_dir: P) -> Result<()> {
    let file_name = if cfg!(windows) {
        format!("{}.exe", package_name)
    } else {
        package_name.to_owned()
    };
    let execute_path = env::temp_dir()
        .join("pit")
        .join(&package_name)
        .join("target")
        .join("release")
        .join(&file_name);
    let target_path = out_dir.as_ref().join(&file_name);
    fs::copy(&execute_path, target_path)?;

    Ok(())
}

pub(crate) fn release<P: AsRef<Path>>(package: &Package, out_dir: P, quiet: bool) -> Result<()> {
    if !quiet {
        println!(
            "{}",
            &format!("Release {} package", &package.name)
                .bright_green()
                .bold()
        );
    }

    build(&package, true, quiet)?;
    distribute(&package.name, out_dir)?;

    Ok(())
}
