use std::{env, fs, path::Path, process};

use anyhow::{bail, Result};
use serde_derive::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tempdir::TempDir;

#[derive(Debug)]
struct Project {
    name: String,
    toml: String,
    src: String,
}

impl From<&str> for Project {
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

        Project { name, toml, src }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Identity {
    name: String,
    hash: String,
}

impl Project {
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

fn create_toml(project_dir: &Path, toml: &str) -> Result<()> {
    let toml_file = project_dir.join("Cargo.toml");
    fs::write(toml_file, toml.as_bytes())?;

    Ok(())
}

fn create_src(project_dir: &Path, src: &str) -> Result<()> {
    let src_dir = project_dir.join("src");
    fs::create_dir(&src_dir)?;
    let src_file = src_dir.join("main.rs");
    fs::write(src_file, src.as_bytes())?;

    Ok(())
}

fn build_package(project_dir: &Path) -> Result<()> {
    let mut command = process::Command::new("cargo");
    command.arg("build");
    let exit_status = command.current_dir(&project_dir).spawn()?.wait()?;

    if !exit_status.success() {
        bail!("Failed to build.");
    }

    Ok(())
}

fn execute(project_dir: &Path, name: &str) -> Result<()> {
    let execute_path = project_dir.join("target").join("debug").join(name);
    let exit_status = process::Command::new(execute_path).spawn()?.wait()?;

    if !exit_status.success() {
        bail!("Failed to execute.");
    }

    Ok(())
}

fn main() -> Result<()> {
    let src = fs::read_to_string("./sample/snippet.rs")?;

    let temp_dir = TempDir::new("pit")?;
    for package in src.split("//# ---") {
        let project = Project::from(package);

        let cache_dir = env::temp_dir().join("pit").join(&project.name);

        let identity = project.gen_identity();
        let cache_identity_path = cache_dir.join("identity_hash.toml");
        // If there is no change in iether src or toml, use the executable file on the cache.
        if let Ok(cache_identity) = fs::read(&cache_identity_path) {
            let cache_identity: Identity = toml::from_slice(&cache_identity)?;
            if identity.hash == cache_identity.hash {
                let cache_execute_path = cache_dir.join("target").join("debug").join(&project.name);
                let exit_status = process::Command::new(cache_execute_path).spawn()?.wait()?;

                if exit_status.success() {
                    continue;
                }
            }
        }

        let project_dir = temp_dir.path().join(&project.name);
        fs::create_dir(&project_dir)?;

        create_toml(&project_dir, &project.toml)?;
        create_src(&project_dir, &project.src)?;

        let project_target_dir = project_dir.join("target");
        let cache_target_dir = cache_dir.join("target");
        fs::create_dir_all(&cache_target_dir)?;
        // Restore target directory from cache.
        fs::rename(&cache_target_dir, &project_target_dir)?;

        build_package(&project_dir)?;
        execute(&project_dir, &project.name)?;

        if cache_target_dir.exists() {
            bail!("Failed to handle cache.")
        }
        // Store target directory in cache.
        fs::rename(project_target_dir, cache_target_dir)?;

        // Store the hash generated from src and toml.
        let identity = toml::to_string(&Identity {
            name: identity.name,
            hash: identity.hash,
        })?;
        fs::write(cache_identity_path, identity)?;
    }

    Ok(())
}

#[test]
fn project_from() {
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

    let project = Project::from(INPUT);
    assert_eq!(project.name, NAME);
    assert_eq!(project.toml, TOML.trim());
    assert_eq!(project.src, SRC.trim());
}
