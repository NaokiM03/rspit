use std::{fs, path::Path};

use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng};

pub(crate) fn init_snippet<P: AsRef<Path>>(file_name: &str, out_dir: P) -> Result<()> {
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
"###,
        name
    );
    let content = content.trim_start();

    let file_path = out_dir.as_ref().join(file_name);
    fs::write(file_path, content)?;

    Ok(())
}
