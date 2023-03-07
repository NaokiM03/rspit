use std::{fs, path::Path};

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};

pub(super) fn random_name() -> String {
    let random_str: String = "abcdefghijklmnopqrstuvwxyz0123456789"
        .as_bytes()
        .choose_multiple(&mut thread_rng(), 7)
        .cloned()
        .map(char::from)
        .collect();
    format!("tmp-{}", random_str)
}

pub(super) fn create_toml<P: AsRef<Path>>(package_dir: P, toml: &str) -> Result<()> {
    let toml_file = package_dir.as_ref().join("Cargo.toml");
    fs::write(toml_file, toml.as_bytes())?;

    Ok(())
}

pub(super) fn create_src<P: AsRef<Path>>(package_dir: P, src: &str) -> Result<()> {
    let src_dir = package_dir.as_ref().join("src");
    fs::create_dir(&src_dir)?;
    let src_file = src_dir.join("main.rs");
    fs::write(src_file, src.as_bytes())?;

    Ok(())
}
