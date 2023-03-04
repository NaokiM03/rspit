use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

use anyhow::{bail, Result};
use rand::{distributions::Alphanumeric, seq::SliceRandom, thread_rng, Rng};
use tiny_ansi::TinyAnsi;

mod cache;
mod package;

pub(crate) use cache::cache_dir;
pub(crate) use package::Package;

pub(crate) fn packages_from_path<P: AsRef<Path>>(file_path: P) -> Vec<Package> {
    fs::read_to_string(file_path)
        .expect("Failed to read string from file.")
        .split("//# ---")
        .map(|x| Package::from(x))
        .collect()
}

fn random_name() -> String {
    let random_str: String = "abcdefghijklmnopqrstuvwxyz0123456789"
        .as_bytes()
        .choose_multiple(&mut thread_rng(), 7)
        .cloned()
        .map(char::from)
        .collect();
    format!("tmp-{}", random_str)
}

fn temp_dir() -> PathBuf {
    let suffix: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    env::temp_dir().join(format!("pit-{}", suffix))
}

fn create_toml<P: AsRef<Path>>(package_dir: P, toml: &str) -> Result<()> {
    let toml_file = package_dir.as_ref().join("Cargo.toml");
    fs::write(toml_file, toml.as_bytes())?;

    Ok(())
}

fn create_src<P: AsRef<Path>>(package_dir: P, src: &str) -> Result<()> {
    let src_dir = package_dir.as_ref().join("src");
    fs::create_dir(&src_dir)?;
    let src_file = src_dir.join("main.rs");
    fs::write(src_file, src.as_bytes())?;

    Ok(())
}

// Check

fn cargo_check<P: AsRef<Path>>(package_dir: P, quiet: bool) -> Result<()> {
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

pub(crate) fn check(file_name: &str, package: &Package, quiet: bool) -> Result<()> {
    let temp_dir = temp_dir();
    let package_dir = temp_dir.join(file_name).join(&package.name);
    fs::create_dir_all(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;

    let package_target_dir = package_dir.join("target");
    let cache_target_dir = cache::cache_dir()
        .join(file_name)
        .join(&package.name)
        .join("target");

    cache::restore(&cache_target_dir, &package_target_dir)?;
    cargo_check(&package_dir, quiet)?;
    cache::store(&package_target_dir, &cache_target_dir)?;

    let _ = fs::remove_dir_all(temp_dir);

    Ok(())
}

// Build

fn cargo_build<P: AsRef<Path>>(package_dir: P, release: bool, quiet: bool) -> Result<()> {
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

pub(crate) fn build(file_name: &str, package: &Package, release: bool, quiet: bool) -> Result<()> {
    let temp_dir = temp_dir();
    let package_dir = temp_dir.join(file_name).join(&package.name);
    fs::create_dir_all(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;

    let package_target_dir = package_dir.join("target");
    let cache_target_dir = cache::cache_dir()
        .join(file_name)
        .join(&package.name)
        .join("target");

    cache::restore(&cache_target_dir, &package_target_dir)?;
    cargo_build(&package_dir, release, quiet)?;
    cache::store(&package_target_dir, &cache_target_dir)?;

    let _ = fs::remove_dir_all(temp_dir);

    // The hash is not stored at release build time.
    // This is because release build is only used with the `release` command.
    if release {
        return Ok(());
    }

    cache::write_identity_hash(file_name, &package)?;

    Ok(())
}

// Run

fn execute(file_name: &str, package_name: &str) -> Result<()> {
    let execute_path = cache_dir()
        .join(file_name)
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

pub(crate) fn run(file_name: &str, package: &Package, quiet: bool) -> Result<()> {
    let output_text = format!("Run {} package", &package.name)
        .bright_green()
        .bold();

    // If there is no change in either src or toml, use the executable file on the cache.
    if cache::check_identity_hash(file_name, package).is_some() {
        println!("{output_text}");
        return execute(file_name, &package.name);
    }

    build(file_name, package, false, quiet)?;

    println!("{output_text}");
    execute(file_name, &package.name)?;

    Ok(())
}

// Release

fn distribute<P: AsRef<Path>>(file_name: &str, package_name: &str, out_dir: P) -> Result<()> {
    let exe_name = if cfg!(windows) {
        format!("{}.exe", package_name)
    } else {
        package_name.to_owned()
    };
    let execute_path = cache_dir()
        .join(file_name)
        .join(&package_name)
        .join("target")
        .join("release")
        .join(&exe_name);

    let out_dir = out_dir.as_ref();
    fs::create_dir_all(out_dir)?;

    let target_path = out_dir.join(&exe_name);
    fs::copy(&execute_path, target_path)?;

    Ok(())
}

pub(crate) fn release<P: AsRef<Path>>(
    file_name: &str,
    package: &Package,
    out_dir: P,
    quiet: bool,
) -> Result<()> {
    build(file_name, package, true, quiet)?;
    distribute(file_name, &package.name, out_dir)?;

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

pub(crate) fn list<P: AsRef<Path>>(file_path: P) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .map(|x| &x.name)
        .for_each(|name| println!("{}", name));

    Ok(())
}

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

fn create_gitignore<P: AsRef<Path>>(package_dir: P) -> Result<()> {
    let gitignore = package_dir.as_ref().join(".gitignore");
    let contents = r#"
/target
"#
    .trim_start();
    fs::write(gitignore, contents)?;

    Ok(())
}

pub(crate) fn extract<P: AsRef<Path>>(
    package: &Package,
    out_dir: P,
) -> Result<()> {
    let package_dir = out_dir.as_ref().join(&package.name);
    fs::create_dir_all(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;
    create_gitignore(&package_dir)?;

    Ok(())
}

// Clean

pub(crate) fn clean() -> Result<()> {
    for entry in cache_dir().read_dir()? {
        let Ok(entry) = entry else { continue };
        fs::remove_dir_all(entry.path())?;
    }

    Ok(())
}
