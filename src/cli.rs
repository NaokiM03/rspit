use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

use crate::add::add_package;
use crate::build::{build_all, build_specified_package};
use crate::clean::clean_cache_dir;
use crate::list::list_packages;
use crate::run::{run_all, run_specified_package};

#[derive(Debug, Parser)]
#[command(name = "pit", author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Run all package in file
    Run {
        file_path: String,
        /// Run only the specified package
        #[arg(short, long)]
        package: Option<String>,
        /// Do not print cargo log messages
        #[arg(short, long)]
        quiet: bool,
    },
    /// Build all package in file
    Build {
        file_path: String,
        /// Build only the specified package
        #[arg(short, long)]
        package: Option<String>,
        /// Build in release mode with optimizations
        #[arg(short, long)]
        release: bool,
        /// Do not print cargo log messages
        #[arg(short, long)]
        quiet: bool,
    },
    /// List all packages in the given file
    List { file_path: String },
    /// Add an empty package on top in the given file
    Add { file_path: String },
    /// Remove everything in the cache directory
    Clean,
}

pub(crate) fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            SubCommands::Run {
                file_path,
                package,
                quiet,
            } => {
                if let Some(package) = package {
                    run_specified_package(file_path, &package, quiet)?;
                } else {
                    run_all(file_path, quiet)?;
                }
            }
            SubCommands::Build {
                file_path,
                package,
                release,
                quiet,
            } => {
                if let Some(package) = package {
                    build_specified_package(file_path, &package, release, quiet)?;
                } else {
                    build_all(file_path, release, quiet)?;
                }
            }
            SubCommands::List { file_path } => {
                list_packages(file_path)?;
            }
            SubCommands::Add { file_path } => {
                add_package(file_path)?;
            }
            SubCommands::Clean => {
                clean_cache_dir()?;
            }
        }
    } else {
        let mut cmd = Args::command();
        cmd.print_help()?;
    }

    Ok(())
}
