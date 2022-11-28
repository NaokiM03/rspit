use std::path::Path;

use anyhow::Result;

use crate::core::{packages_from_path, release};

pub(crate) fn release_specified_package<P: AsRef<Path>, Q: AsRef<Path>>(
    file_path: P,
    package: &str,
    out_dir: Q,
    quiet: bool,
) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .filter(|x| x.name == package)
        .next()
        .iter()
        .for_each(|package| release(&package, &out_dir, quiet).expect("Failed to release."));

    Ok(())
}

pub(crate) fn release_all<P: AsRef<Path>, Q: AsRef<Path>>(
    file_path: P,
    out_dir: Q,
    quiet: bool,
) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .for_each(|package| release(&package, &out_dir, quiet).expect("Failed to release."));

    Ok(())
}
