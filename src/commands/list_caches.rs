use std::path::Path;

use anyhow::Result;

use crate::core::{list_caches, packages_from_path};

pub(crate) fn list_cached_packages<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let file_path = file_path.as_ref();
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();

    packages_from_path(file_path).iter().for_each(|package| {
        list_caches(file_name, package).expect("Failed to list cached packages.")
    });

    Ok(())
}
