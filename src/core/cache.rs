use std::{
    env, fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};

pub(crate) struct Cache {
    root: PathBuf,
    file_name: String,
    package_name: String,
    identity_hash: String,
}

impl Cache {
    pub(crate) fn new(file_name: &str, package_name: &str, identity_hash: &str) -> Cache {
        let root = Self::root_dir();
        let file_name = file_name.to_owned();
        let package_name = package_name.to_owned();
        let identity_hash = identity_hash.to_owned();
        Cache {
            root,
            file_name,
            package_name,
            identity_hash,
        }
    }

    pub(crate) fn package_name(&self) -> String {
        self.package_name.to_owned()
    }

    pub(crate) fn root_dir() -> PathBuf {
        env::temp_dir().join("pit")
    }

    fn package_dir(&self) -> PathBuf {
        self.root.join(&self.file_name).join(&self.package_name)
    }

    fn target_dir(&self) -> PathBuf {
        self.package_dir().join("target")
    }

    pub(crate) fn debug_exe(&self) -> PathBuf {
        let exe_name = if cfg!(windows) {
            format!("{}.exe", &self.package_name)
        } else {
            self.package_name.to_owned()
        };
        self.target_dir().join("debug").join(exe_name)
    }

    pub(crate) fn release_exe(&self) -> PathBuf {
        let exe_name = if cfg!(windows) {
            format!("{}.exe", &self.package_name)
        } else {
            self.package_name.to_owned()
        };
        self.target_dir().join("release").join(exe_name)
    }

    pub(crate) fn restore<P: AsRef<Path>>(&self, package_target_dir: P) -> Result<()> {
        fs::create_dir_all(self.target_dir())?;
        // Restore target directory from cache.
        fs::rename(self.target_dir(), &package_target_dir)?;

        Ok(())
    }

    pub(crate) fn store<P: AsRef<Path>>(&self, package_target_dir: P) -> Result<()> {
        if self.target_dir().exists() {
            bail!("Failed to handle cache.")
        }
        // Store target directory in cache.
        fs::rename(package_target_dir, self.target_dir())?;

        Ok(())
    }

    pub(crate) fn write_identity_hash(&self) -> Result<()> {
        let path = self.package_dir().join("identity_hash");
        let contents = self.identity_hash.to_owned();
        fs::write(path, contents)?;

        Ok(())
    }

    pub(crate) fn check_identity_hash(&self) -> Option<()> {
        let path = self.package_dir().join("identity_hash");

        if let Ok(identity_hash) = fs::read_to_string(&path) {
            if identity_hash == self.identity_hash {
                return Some(());
            }
        }
        None
    }
}
