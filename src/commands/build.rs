use std::path::Path;

use anyhow::Result;
use rayon::prelude::*;

use crate::core::{build, packages_from_path};

pub(crate) fn build_specified_package<P: AsRef<Path>>(
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
        .for_each(|package| build(file_name, package, false, quiet).expect("Failed to build."));

    Ok(())
}

pub(crate) fn build_all<P: AsRef<Path>>(file_path: P, quiet: bool) -> Result<()> {
    let file_path = file_path.as_ref();
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();

    packages_from_path(file_path)
        .iter()
        .for_each(|package| build(file_name, package, false, quiet).expect("Failed to build."));

    Ok(())
}

pub(crate) fn build_all_parallel<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let file_path = file_path.as_ref();
    let file_name = file_path.file_stem().unwrap().to_str().unwrap();

    packages_from_path(file_path)
        .par_iter()
        .for_each(|package| build(file_name, package, false, true).expect("Failed to build."));

    Ok(())
}
