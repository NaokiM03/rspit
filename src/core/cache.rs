use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};

pub(super) fn root_dir() -> PathBuf {
    env::temp_dir().join("pit")
}

pub(crate) struct Cache {
    target_dir: PathBuf,

    identity_hash: PathBuf,
    current_identity_hash: String,

    pub(super) exe_name: String,
    pub(super) debug_exe: PathBuf,
    pub(super) release_exe: PathBuf,
}

impl Cache {
    pub(super) fn new(file_name: &str, package_name: &str) -> Cache {
        let root = root_dir();
        let file_name = file_name.to_owned();
        let package_dir = root.join(file_name).join(package_name);

        let target_dir = package_dir.join("target");

        let identity_hash = package_dir.join("identity_hash");
        let current_identity_hash = fs::read_to_string(&identity_hash).unwrap_or_default();

        let exe_name = if cfg!(windows) {
            format!("{package_name}.exe")
        } else {
            package_name.to_owned()
        };
        let debug_exe = target_dir.join("debug").join(&exe_name);
        let release_exe = target_dir.join("release").join(&exe_name);

        Cache {
            target_dir,

            identity_hash,
            current_identity_hash,

            exe_name,
            debug_exe,
            release_exe,
        }
    }

    pub(super) fn restore<P: AsRef<Path>>(&self, target_dir: P) -> Result<()> {
        fs::create_dir_all(&self.target_dir)?;
        // Restore target directory from cache.
        fs::rename(&self.target_dir, &target_dir)?;

        Ok(())
    }

    pub(super) fn store<P: AsRef<Path>>(&self, target_dir: P) -> Result<()> {
        if self.target_dir.exists() {
            bail!("Failed to handle cache.")
        }
        // Store target directory in cache.
        fs::rename(target_dir, &self.target_dir)?;

        Ok(())
    }

    pub(super) fn write_identity_hash(&self, new_identity_hash: &str) -> Result<()> {
        fs::write(&self.identity_hash, new_identity_hash)?;

        Ok(())
    }

    pub(super) fn is_same_identity_hash(&self, new_identity_hash: &str) -> bool {
        new_identity_hash == self.current_identity_hash
    }

    pub(super) fn delete_identity_hash(&self) -> Result<()> {
        fs::remove_file(&self.identity_hash)?;

        Ok(())
    }
}
