use std::{fs, path::Path};

use sha2::{Digest, Sha256};

#[derive(Debug)]
pub(crate) struct Package {
    pub(crate) name: String,
    pub(crate) toml: String,
    pub(crate) src: String,
}

impl From<&str> for Package {
    fn from(src: &str) -> Self {
        let toml = src
            .lines()
            .skip_while(|line| line.is_empty())
            .take_while(|line| line.starts_with("//#"))
            .map(|line| line[3..].trim())
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

impl Package {
    pub(crate) fn identity_hash(&self) -> String {
        let hash = Sha256::new()
            .chain_update(&self.toml)
            .chain_update(&self.src)
            .finalize();
        return format!("{:x}", hash);
    }
}

pub(crate) fn packages_from_path<P: AsRef<Path>>(file_path: P) -> Vec<Package> {
    fs::read_to_string(file_path)
        .expect("Failed to read string from file.")
        .split("//# ---")
        .map(|x| Package::from(x))
        .collect()
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
