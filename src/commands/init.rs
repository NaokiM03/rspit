use std::path::Path;

use anyhow::Result;

use crate::core::init;

pub(crate) fn init_snippet<P: AsRef<Path>>(file_name: &str, out_dir: P) -> Result<()> {
    let file_path = out_dir.as_ref().join(file_name);
    init(file_path)?;

    Ok(())
}
