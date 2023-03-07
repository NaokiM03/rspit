use std::{env, fs, path::PathBuf};

use anyhow::Result;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

use super::{
    package::Package,
    utils::{create_src, create_toml},
};

pub(super) struct TempDir {
    root: PathBuf,
    pub(super) package_dir: PathBuf,
    pub(super) package_target_dir: PathBuf,
}

impl TempDir {
    pub(super) fn new(package: &Package) -> TempDir {
        let root = {
            let suffix: String = thread_rng()
                .sample_iter(Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();
            env::temp_dir().join(format!("pit-{}", suffix))
        };

        let Package { name, toml, src } = package.to_owned();

        let package_dir = root.join(name);
        fs::create_dir_all(&package_dir).expect("Failed to create temporary directory.");

        create_toml(&package_dir, toml).expect("Failed to create Cargo.toml");
        create_src(&package_dir, src).expect("Failed to create main.rs");

        // The target directory is not created
        // because it is renamed from the cache.
        let package_target_dir = package_dir.join("target");

        TempDir {
            root,
            package_dir,
            package_target_dir,
        }
    }

    pub(super) fn remove(&self) -> Result<()> {
        fs::remove_dir_all(&self.root)?;

        Ok(())
    }
}
