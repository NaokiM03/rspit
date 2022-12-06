use std::path::Path;

use anyhow::Result;

use crate::core::{check, packages_from_path};

pub(crate) fn check_specified_package<P: AsRef<Path>>(
    file_path: P,
    package: &str,
    quiet: bool,
) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .find(|x| x.name == package)
        .iter()
        .for_each(|package| check(&package, quiet).expect("Failed to check."));

    Ok(())
}

pub(crate) fn check_all<P: AsRef<Path>>(file_path: P, quiet: bool) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .for_each(|package| check(&package, quiet).expect("Failed to check."));

    Ok(())
}
