use std::{fs, path::Path, process};

use anyhow::{bail, Result};
use tiny_ansi::TinyAnsi;

mod cache;
mod cargo;
mod package;
mod temp_dir;
mod utils;

pub(crate) use package::packages_from_path;

use cache::Cache;
use package::Package;
use temp_dir::TempDir;
use utils::{create_gitignore, create_src, create_toml, random_name};

// Check

pub(crate) fn check(file_name: &str, package: &Package, quiet: bool) -> Result<()> {
    let temp_dir = TempDir::new(&package);
    let cache = Cache::new(file_name, &package.name);

    cache.restore(&temp_dir.target_dir)?;
    cargo::check(&temp_dir.package_dir, quiet)?;
    cache.store(&temp_dir.target_dir)?;

    let _ = temp_dir.remove();

    Ok(())
}

// Build

pub(crate) fn build(
    file_name: &str,
    package: &Package,
    release: bool,
    quiet: bool,
) -> Result<Cache> {
    let cache = Cache::new(file_name, &package.name);

    let new_identity_hash = package.identity_hash();
    if !release && cache.is_same_identity_hash(&new_identity_hash) {
        let output_text = format!(
            "Skip building the {} package because it is cached.",
            &package.name
        )
        .bright_green()
        .bold();
        println!("{output_text}");

        return Ok(cache);
    }

    let output_text = format!("Build {} package", &package.name)
        .bright_green()
        .bold();
    println!("{output_text}");

    let temp_dir = TempDir::new(&package);

    cache.restore(&temp_dir.target_dir)?;
    if let Err(e) = cargo::build(&temp_dir.package_dir, release, quiet) {
        cache.store(&temp_dir.target_dir)?;
        let _ = cache.delete_identity_hash();
        return Err(e);
    }
    cache.store(&temp_dir.target_dir)?;

    // Cache is also used for release build,
    // but compiled each time.
    if !release {
        cache.write_identity_hash(&new_identity_hash)?;
    }

    let _ = temp_dir.remove();

    Ok(cache)
}

// Run

pub(crate) fn run(file_name: &str, package: &Package, quiet: bool) -> Result<()> {
    fn execute<P: AsRef<Path>>(exe: P) -> Result<()> {
        let exit_status = process::Command::new(exe.as_ref()).spawn()?.wait()?;
        if !exit_status.success() {
            bail!("Failed to execute.");
        }

        Ok(())
    }

    let output_text = format!("Run {} package", &package.name)
        .bright_green()
        .bold();
    println!("{output_text}");

    let cache = build(file_name, package, false, quiet)?;
    execute(cache.debug_exe)?;

    Ok(())
}

// Release

pub(crate) fn release<P: AsRef<Path>>(
    file_name: &str,
    package: &Package,
    out_dir: P,
    quiet: bool,
) -> Result<()> {
    fn distribute<P: AsRef<Path>, Q: AsRef<Path>>(
        exe: P,
        out_dir: Q,
        exe_name: &str,
    ) -> Result<()> {
        let from = exe.as_ref();

        let to = {
            let out_dir = out_dir.as_ref();
            fs::create_dir_all(out_dir)?;
            out_dir.join(exe_name)
        };

        fs::copy(from, to)?;

        Ok(())
    }

    let cache = build(file_name, package, true, quiet)?;
    distribute(&cache.release_exe, &out_dir, &cache.exe_name)?;

    Ok(())
}

// Init

pub(crate) fn init<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let name = random_name();
    let content = format!(
        r###"
//# [package]
//# name = "{}"
//# version = "0.1.0"
//# edition = "2021"
//#
//# [dependencies]
//#
//# [profile.release]
//# lto = true

fn main() {{
}}
"###,
        name
    );
    let content = content.trim_start();

    fs::write(file_path, content)?;

    Ok(())
}

// List
// Removed because too simple.
// For exaple, if want an option to show

// Add

pub(crate) fn add<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let src = fs::read_to_string(&file_path)?;

    let name = random_name();
    let content = format!(
        r###"
//# [package]
//# name = "{}"
//# version = "0.1.0"
//# edition = "2021"
//#
//# [dependencies]
//#
//# [profile.release]
//# lto = true

fn main() {{
}}

//# ---

{}"###,
        name, src
    );
    let content = content.trim_start();

    fs::write(file_path, content)?;

    Ok(())
}

// Extract

pub(crate) fn extract<P: AsRef<Path>>(package: &Package, out_dir: P) -> Result<()> {
    let package_dir = out_dir.as_ref().join(&package.name);
    fs::create_dir_all(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;
    create_gitignore(&package_dir)?;

    Ok(())
}

// Clean

pub(crate) fn clean() -> Result<()> {
    for entry in cache::root_dir().read_dir()? {
        let Ok(entry) = entry else { continue };
        fs::remove_dir_all(entry.path())?;
    }

    Ok(())
}

// ListCaches

pub(crate) fn list_caches(file_name: &str, package: &Package) -> Result<()> {
    let cache = Cache::new(file_name, &package.name);

    let new_identity_hash = package.identity_hash();
    if cache.is_same_identity_hash(&new_identity_hash) {
        println!("{}", package.name);
    }

    Ok(())
}
