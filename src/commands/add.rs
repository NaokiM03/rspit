use std::path::Path;

use anyhow::Result;

use crate::core::add;

pub(crate) fn add_package<P: AsRef<Path>>(file_path: P) -> Result<()> {
    add(file_path)?;

    Ok(())
}
