use std::{fs, path::Path};

use anyhow::Result;

use crate::core::Package;

    let src = fs::read_to_string(file_path)?;
    let names = src
        .split("//# ---")
        .map(|x| Package::from(x))
        .map(|x| x.name)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", names);
pub(crate) fn list_packages<P: AsRef<Path>>(file_path: P) -> Result<()> {

    Ok(())
}
