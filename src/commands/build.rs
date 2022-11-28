use std::path::Path;

use anyhow::Result;

use crate::core::{build, packages_from_path};

pub(crate) fn build_specified_package<P: AsRef<Path>>(
    file_path: P,
    package: &str,
    quiet: bool,
) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .filter(|x| x.name == package)
        .next()
        .iter()
        .for_each(|package| build(&package, false, quiet).expect("Failed to build."));

    Ok(())
}

pub(crate) fn build_all<P: AsRef<Path>>(file_path: P, quiet: bool) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .for_each(|package| build(&package, false, quiet).expect("Failed to build."));

    Ok(())
}
