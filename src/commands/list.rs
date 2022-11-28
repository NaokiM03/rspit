use std::path::Path;

use anyhow::Result;

use crate::core::packages_from_path;

pub(crate) fn list_packages<P: AsRef<Path>>(file_path: P) -> Result<()> {
    packages_from_path(file_path)
        .iter()
        .map(|x| &x.name)
        .for_each(|name| println!("{}", name));

    Ok(())
}
