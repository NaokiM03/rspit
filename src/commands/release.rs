use std::{fs, path::Path};

use anyhow::Result;

use crate::core::{release, Package};

pub(crate) fn release_specified_package<P: AsRef<Path>, Q: AsRef<Path>>(
    file_path: P,
    package: &str,
    out_dir: Q,
    quiet: bool,
    let src = fs::read_to_string(file_path)?;

    let package = src
        .split("//# ---")
        .map(|x| Package::from(x))
) -> Result<()> {
        .filter(|x| x.name == package)
        .next()
        .expect("Package not found in file.");
    release(&package, out_dir, quiet)?;

    Ok(())
}

    let src = fs::read_to_string(file_path)?;

    for package in src.split("//# ---") {
        let package = Package::from(package);
        release(&package, &out_dir, quiet)?;
    }
pub(crate) fn release_all<P: AsRef<Path>, Q: AsRef<Path>>(
    file_path: P,
    out_dir: Q,
    quiet: bool,
) -> Result<()> {

    Ok(())
}
