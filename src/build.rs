use std::{env, fs, path::Path, process};

use anyhow::{bail, Result};
use tempdir::TempDir;
use tiny_ansi::TinyAnsi;

use crate::{cache, package::Package};

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
    println!(
        "{}",
        &format!("Build {} package", &package.name)
            .bright_green()
            .bold()
    );

    let temp_dir = TempDir::new("pit")?;
    let cache_dir = env::temp_dir().join("pit").join(&package.name);

    let package_dir = temp_dir.path().join(&package.name);
    fs::create_dir_all(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;

    let package_target_dir = package_dir.join("target");
    let cache_target_dir = cache_dir.join("target");

    cache::restore(&cache_target_dir, &package_target_dir)?;
    cargo_build(&package_dir, release, quiet)?;
    cache::store(&package_target_dir, &cache_target_dir)?;

    // The hash is not stored at release build time.
    // This is because the `run` command always uses the debug build.
    if release {
        return Ok(());
    }

    cache::write_identity_hash(&package, &cache_dir)?;

    Ok(())
}

pub(crate) fn build_specified_package<P>(
    file_path: P,
    package: &str,
    release: bool,
    quiet: bool,
) -> Result<()>
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
    build(&package, release, quiet)?;

    Ok(())
}

pub(crate) fn build_all<P>(file_path: P, release: bool, quiet: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;

    for package in src.split("//# ---") {
        let package = Package::from(package);
        build(&package, release, quiet)?;
    }

    Ok(())
}
