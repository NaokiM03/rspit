use std::path::Path;

use anyhow::Result;

use crate::core::list;

pub(crate) fn list_packages<P: AsRef<Path>>(file_path: P) -> Result<()> {
    list(file_path)?;

    Ok(())
}
