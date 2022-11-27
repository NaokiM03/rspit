use std::{env, fs, path::Path, process};

use anyhow::{bail, Result};
use clap::{CommandFactory, Parser, Subcommand};
use rand::{seq::SliceRandom, thread_rng};
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tempdir::TempDir;
use tiny_ansi::TinyAnsi;

#[derive(Debug)]
struct Package {
    name: String,
    toml: String,
    src: String,
}

impl From<&str> for Package {
    fn from(src: &str) -> Self {
        let toml = src
            .lines()
            .skip_while(|line| line.is_empty())
            .take_while(|line| line.starts_with("//#"))
            .map(|line| line[3..].trim())
            // .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>()
            .join("\n");

        let src = src
            .lines()
            .skip_while(|line| line.is_empty() || line.starts_with("//#"))
            .collect::<Vec<&str>>()
            .join("\n");

        let name = {
            let value = toml
                .parse::<toml::Value>()
                .expect("Failed to parse string into toml.");
            value["package"]["name"]
                .as_str()
                .expect("Failed to extract name from toml.")
                .to_owned()
        };

        Package { name, toml, src }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Identity {
    name: String,
    hash: String,
}

impl Package {
    fn gen_identity(&self) -> Identity {
        let hash = Sha256::new()
            .chain_update(&self.toml)
            .chain_update(&self.src)
            .finalize();
        Identity {
            name: self.name.to_owned(),
            hash: format!("{:x}", hash),
        }
    }
}

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

fn build_package(package_dir: &Path, quiet: bool) -> Result<()> {
    let mut command = process::Command::new("cargo");
    command.arg("build");
    if quiet {
        command.arg("--quiet");
    }
    let exit_status = command.current_dir(&package_dir).spawn()?.wait()?;

    if !exit_status.success() {
        bail!("Failed to build.");
    }

    Ok(())
}

fn execute(package_dir: &Path, name: &str) -> Result<()> {
    let execute_path = package_dir.join("target").join("debug").join(name);
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

    let temp_dir = TempDir::new("pit")?;

    let package_dir = temp_dir.path().join(&package.name);
    fs::create_dir(&package_dir)?;

    create_toml(&package_dir, &package.toml)?;
    create_src(&package_dir, &package.src)?;

    let package_target_dir = package_dir.join("target");
    let cache_target_dir = cache_dir.join("target");
    fs::create_dir_all(&cache_target_dir)?;
    // Restore target directory from cache.
    fs::rename(&cache_target_dir, &package_target_dir)?;

    build_package(&package_dir, quiet)?;
    execute(&package_dir, &package.name)?;

    if cache_target_dir.exists() {
        bail!("Failed to handle cache.")
    }
    // Store target directory in cache.
    fs::rename(package_target_dir, cache_target_dir)?;

    // Store the hash generated from src and toml.
    let identity = toml::to_string(&Identity {
        name: identity.name,
        hash: identity.hash,
    })?;
    fs::write(cache_identity_path, identity)?;

    Ok(())
}

fn run_specified_package<P>(file_path: P, package: &str, quiet: bool) -> Result<()>
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

fn run_all<P>(file_path: P, quiet: bool) -> Result<()>
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

fn list_packages<P>(file_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;
    let names = src
        .split("//# ---")
        .map(|x| Package::from(x))
        .map(|x| x.name)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", names);

    Ok(())
}

fn add_package<P>(file_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let src = fs::read_to_string(&file_path)?;

    let random_str: String = "abcdefghijklmnopqrstuvwxyz0123456789"
        .as_bytes()
        .choose_multiple(&mut thread_rng(), 7)
        .cloned()
        .map(char::from)
        .collect();
    let name = format!("tmp-{}", random_str);

    let content = format!(
        r###"
//# [package]
//# name = "{}"
//# version = "0.1.0"
//# edition = "2021"
//#
//# [dependencies]
//#

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

fn clean_cache_dir() -> Result<()> {
    let cache_dir = env::temp_dir().join("pit");
    for entry in cache_dir.read_dir()? {
        if let Ok(entry) = entry {
            dbg!(&entry);
            fs::remove_dir_all(entry.path())?;
        }
    }

    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "pit", author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Run all package in file
    Run {
        file_path: String,
        /// Run only the specified package
        package: Option<String>,
        /// Do not print cargo log messages
        #[arg(short, long)]
        quiet: bool,
    },
    /// List all packages in the given file
    List { file_path: String },
    /// Add an empty package on top in the given file
    Add { file_path: String },
    /// Remove everything in the cache directory
    Clean,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            SubCommands::Run {
                file_path,
                package,
                quiet,
            } => {
                if let Some(package) = package {
                    run_specified_package(file_path, &package, quiet)?;
                } else {
                    run_all(file_path, quiet)?;
                }
            }
            SubCommands::List { file_path } => {
                list_packages(file_path)?;
            }
            SubCommands::Add { file_path } => {
                add_package(file_path)?;
            }
            SubCommands::Clean => {
                clean_cache_dir()?;
            }
        }
    } else {
        let mut cmd = Args::command();
        cmd.print_help()?;
    }

    Ok(())
}

#[test]
fn package_from() {
    const INPUT: &str = r#"
//# [package]
//# name = "test"
//# version = "0.1.0"
//# edition = "2021"
//#
//# [dependencies]
//# rand = "*"

use rand::prelude::*;

fn main() {
    let num: u64 = random();
    println!("num: {}", num);
}
"#;

    const NAME: &str = "test";

    const TOML: &str = r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "*"
"#;

    const SRC: &str = r#"
use rand::prelude::*;

fn main() {
    let num: u64 = random();
    println!("num: {}", num);
}
"#;

    let package = Package::from(INPUT);
    assert_eq!(package.name, NAME);
    assert_eq!(package.toml, TOML.trim());
    assert_eq!(package.src, SRC.trim());
}
