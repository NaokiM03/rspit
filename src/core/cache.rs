use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use serde_derive::{Deserialize, Serialize};

use super::package::Package;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Identity {
    pub(crate) name: String,
    pub(crate) hash: String,
}

pub(crate) fn restore(cache_target_dir: &Path, package_target_dir: &Path) -> Result<()> {
    fs::create_dir_all(&cache_target_dir)?;
    // Restore target directory from cache.
    fs::rename(&cache_target_dir, &package_target_dir)?;

    Ok(())
}

pub(crate) fn store(package_target_dir: &Path, cache_target_dir: &Path) -> Result<()> {
    if cache_target_dir.exists() {
        bail!("Failed to handle cache.")
    }
    // Store target directory in cache.
    fs::rename(package_target_dir, cache_target_dir)?;

    Ok(())
}

pub(crate) fn cache_dir() -> PathBuf {
    env::temp_dir().join("pit")
}

pub(crate) fn write_identity_hash(package: &Package) -> Result<()> {
    let cache_identity_path = cache_dir().join(&package.name).join("identity_hash.toml");
    let identity = package.gen_identity();
    // Store the hash generated from src and toml.
    let identity = toml::to_string(&Identity {
        name: identity.name,
        hash: identity.hash,
    })?;
    fs::write(cache_identity_path, identity)?;

    Ok(())
}

pub(crate) fn check_identity_hash(package: &Package) -> Option<()> {
    let identity = package.gen_identity();
    let cache_identity_path = cache_dir().join(&package.name).join("identity_hash.toml");
    if let Ok(cache_identity) = fs::read(&cache_identity_path) {
        let cache_identity: Identity = toml::from_slice(&cache_identity).unwrap();
        if identity.hash == cache_identity.hash {
            return Some(());
        }
    }
    None
}
