use std::{fs, path::Path};

use anyhow::Result;

use crate::core::{build, Package};

pub(crate) fn build_specified_package<P: AsRef<Path>>(
    file_path: P,
    package: &str,
    quiet: bool,
) -> Result<()> {
    let src = fs::read_to_string(file_path)?;

    let package = src
        .split("//# ---")
        .map(|x| Package::from(x))
        .filter(|x| x.name == package)
        .next()
        .expect("Package not found in file.");
    build(&package, false, quiet)?;

    Ok(())
}

pub(crate) fn build_all<P: AsRef<Path>>(file_path: P, quiet: bool) -> Result<()> {
    let src = fs::read_to_string(file_path)?;

    for package in src.split("//# ---") {
        let package = Package::from(package);
        build(&package, false, quiet)?;
    }

    Ok(())
}
