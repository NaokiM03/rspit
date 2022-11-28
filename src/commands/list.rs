use std::{fs, path::Path};

use anyhow::Result;

use crate::core::Package;

pub(crate) fn list_packages<P>(file_path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;
    let names = src
        .split("//# ---")
        .map(|x| Package::from(x))
        .map(|x| x.name)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{}", names);

    Ok(())
}
