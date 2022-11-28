use std::{env, fs};

use anyhow::Result;

pub(crate) fn clean_cache_dir() -> Result<()> {
    let cache_dir = env::temp_dir().join("pit");
    for entry in cache_dir.read_dir()? {
        if let Ok(entry) = entry {
            fs::remove_dir_all(entry.path())?;
        }
    }

    Ok(())
}
