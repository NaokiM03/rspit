use std::path::Path;

use anyhow::Result;
use rayon::prelude::*;

use crate::core::{packages_from_path, release};

pub(crate) fn release_specified_package<P: AsRef<Path>, Q: AsRef<Path>>(
    file_path: P,
    package: &str,
    out_dir: Q,
    quiet: bool,
) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .find(|x| x.name == package)
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

pub(crate) fn release_all_parallel<P: AsRef<Path>, Q: AsRef<Path> + std::marker::Sync>(
    file_path: P,
    out_dir: Q,
) -> Result<()> {
    packages_from_path(file_path)
        .par_iter()
        .for_each(|package| release(&package, &out_dir, true).expect("Failed to release."));

    Ok(())
}
