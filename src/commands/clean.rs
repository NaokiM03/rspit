use anyhow::Result;

use crate::core::clean;

pub(crate) fn clean_cache_dir() -> Result<()> {
    clean()?;

    Ok(())
}
