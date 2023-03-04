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

pub(crate) use cache::Cache;
pub(crate) use package::{packages_from_path, Package};

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
    let package_dir = temp_dir.join(&package.name);
    fs::create_dir_all(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;

    let package_target_dir = package_dir.join("target");
    let cache = Cache::new(file_name, &package.name, &package.identity_hash());

    cache.restore(&package_target_dir)?;
    cargo_check(&package_dir, quiet)?;
    cache.store(&package_target_dir)?;

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
    let package_dir = temp_dir.join(&package.name);
    fs::create_dir_all(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;

    let package_target_dir = package_dir.join("target");
    let cache = Cache::new(file_name, &package.name, &package.identity_hash());

    cache.restore(&package_target_dir)?;
    cargo_build(&package_dir, release, quiet)?;
    cache.store(&package_target_dir)?;

    let _ = fs::remove_dir_all(temp_dir);

    cache.write_identity_hash()?;

    Ok(())
}

// Run

fn execute(cache: Cache) -> Result<()> {
    let exe = cache.debug_exe();
    let exit_status = process::Command::new(exe).spawn()?.wait()?;

    if !exit_status.success() {
        bail!("Failed to execute.");
    }

    Ok(())
}

pub(crate) fn run(file_name: &str, package: &Package, quiet: bool) -> Result<()> {
    let output_text = format!("Run {} package", &package.name)
        .bright_green()
        .bold();

    let cache = Cache::new(file_name, &package.name, &package.identity_hash());

    if cache.is_same_identity_hash() {
        println!("{output_text}");
        return execute(cache);
    }

    build(file_name, package, false, quiet)?;

    println!("{output_text}");
    execute(cache)?;

    Ok(())
}

// Release

fn distribute<P: AsRef<Path>>(cache: &Cache, out_dir: P) -> Result<()> {
    let exe_name = if cfg!(windows) {
        format!("{}.exe", cache.package_name())
    } else {
        cache.package_name()
    };
    let from = cache.release_exe();

    let out_dir = out_dir.as_ref();
    fs::create_dir_all(out_dir)?;

    let to = out_dir.join(&exe_name);
    fs::copy(from, to)?;

    Ok(())
}

pub(crate) fn release<P: AsRef<Path>>(
    file_name: &str,
    package: &Package,
    out_dir: P,
    quiet: bool,
) -> Result<()> {
    let cache = Cache::new(file_name, &package.name, &package.identity_hash());

    if cache.is_same_identity_hash() {
        return distribute(&cache, &out_dir);
    }

    build(file_name, package, true, quiet)?;

    distribute(&cache, &out_dir)?;

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

fn create_gitignore<P: AsRef<Path>>(package_dir: P) -> Result<()> {
    let gitignore = package_dir.as_ref().join(".gitignore");
    let contents = r#"
/target
"#
    .trim_start();
    fs::write(gitignore, contents)?;

    Ok(())
}

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
    for entry in Cache::root_dir().read_dir()? {
        let Ok(entry) = entry else { continue };
        fs::remove_dir_all(entry.path())?;
    }

    Ok(())
}
