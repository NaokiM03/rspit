use std::fs;

use anyhow::Result;

use crate::core::cache_dir;

pub(crate) fn clean_cache_dir() -> Result<()> {
    for entry in cache_dir().read_dir()? {
        if let Ok(entry) = entry {
            fs::remove_dir_all(entry.path())?;
        }
    }

    Ok(())
}
