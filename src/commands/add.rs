use std::{fs, path::Path};

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};

pub(crate) fn add_package<P: AsRef<Path>>(file_path: P) -> Result<()> {
    let src = fs::read_to_string(&file_path)?;

    let random_str: String = "abcdefghijklmnopqrstuvwxyz0123456789"
        .as_bytes()
        .choose_multiple(&mut thread_rng(), 7)
        .cloned()
        .map(char::from)
        .collect();
    let name = format!("tmp-{}", random_str);

    let content = format!(
        r###"
//# [package]
//# name = "{}"
//# version = "0.1.0"
//# edition = "2021"
//#
//# [dependencies]
//#
//# [profile.release]
//# lto = true

fn main() {{
}}

//# ---

{}"###,
        name, src
    );
    let content = content.trim_start();

    fs::write(file_path, content)?;

    Ok(())
}
