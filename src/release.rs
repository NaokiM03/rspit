use std::{env, fs, path::Path};

use anyhow::Result;
use tiny_ansi::TinyAnsi;

use crate::{build, package::Package};

fn distribute<P: AsRef<Path>>(package_name: &str, out_dir: P) -> Result<()> {
    let file_name = if cfg!(windows) {
        format!("{}.exe", package_name)
    } else {
        package_name.to_owned()
    };
    let execute_path = env::temp_dir()
        .join("pit")
        .join(&package_name)
        .join("target")
        .join("release")
        .join(&file_name);
    let target_path = out_dir.as_ref().join(&file_name);
    fs::copy(&execute_path, target_path)?;

    Ok(())
}

fn release<P: AsRef<Path>>(package: &Package, out_dir: P, quiet: bool) -> Result<()> {
    if !quiet {
        println!(
            "{}",
            &format!("Release {} package", &package.name)
                .bright_green()
                .bold()
        );
    }

    build::build(&package, true, quiet)?;
    distribute(&package.name, out_dir)?;

    Ok(())
}

pub(crate) fn release_specified_package<P, Q>(
    file_path: P,
    package: &str,
    out_dir: Q,
    quiet: bool,
) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;

    let package = src
        .split("//# ---")
        .map(|x| Package::from(x))
        .filter(|x| x.name == package)
        .next()
        .expect("Package not found in file.");
    release(&package, out_dir, quiet)?;

    Ok(())
}

pub(crate) fn release_all<P, Q>(file_path: P, out_dir: Q, quiet: bool) -> Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = fs::read_to_string(file_path)?;

    for package in src.split("//# ---") {
        let package = Package::from(package);
        release(&package, &out_dir, quiet)?;
    }

    Ok(())
}
