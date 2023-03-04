use std::path::Path;

use anyhow::Result;

use crate::core::{packages_from_path, run};

pub(crate) fn run_specified_package<P: AsRef<Path>>(
    file_path: P,
    package: &str,
    quiet: bool,
) -> Result<()> {
    let file_path = file_path.as_ref();
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();

    packages_from_path(file_path)
        .iter()
        .find(|x| x.name == package)
        .iter()
        .for_each(|package| run(file_name, package, quiet).expect("Failed to run"));

    Ok(())
}

pub(crate) fn run_all<P: AsRef<Path>>(file_path: P, quiet: bool) -> Result<()> {
    let file_path = file_path.as_ref();
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();

    packages_from_path(file_path)
        .iter()
        .for_each(|package| run(file_name, package, quiet).expect("Failed to run"));

    Ok(())
}
