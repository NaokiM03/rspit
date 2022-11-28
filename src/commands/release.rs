use std::{fs, path::Path};

use anyhow::Result;

use crate::core::{release, Package};

pub(crate) fn release_specified_package<P, Q>(
    file_path: P,
    package: &str,
    out_dir: Q,
    quiet: bool,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;

    let package = src
        .split("//# ---")
        .map(|x| Package::from(x))
        .filter(|x| x.name == package)
        .next()
        .expect("Package not found in file.");
    release(&package, out_dir, quiet)?;

    Ok(())
}

pub(crate) fn release_all<P, Q>(file_path: P, out_dir: Q, quiet: bool) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;

    for package in src.split("//# ---") {
        let package = Package::from(package);
        release(&package, &out_dir, quiet)?;
    }

    Ok(())
}
