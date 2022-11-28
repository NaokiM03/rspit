use std::path::Path;

use anyhow::Result;

use crate::core::{extract, packages_from_path};

pub(crate) fn extract_package<P: AsRef<Path>, Q: AsRef<Path>>(
    file_path: P,
    package: &str,
    out_dir: Q,
) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .find(|x| x.name == package)
        .iter()
        .for_each(|package| extract(&package, &out_dir).expect("Failed to extract."));

    Ok(())
}
