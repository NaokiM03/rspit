use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};

use crate::commands;

#[derive(Debug, Parser)]
#[command(name = "pit", author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// Check all package in file
    Check {
        file_path: String,
        /// Check only the specified package
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
        /// Do not print cargo log messages
        #[arg(short, long)]
        quiet: bool,
        /// Build in parallel without cargo log messages
        #[arg(long)]
        parallel: bool,
    },
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
    /// Build all package in file in release mode
    /// and copy the artifacts to the target directory.
    Release {
        file_path: String,
        /// Build only the specified package
        #[arg(short, long)]
        package: Option<String>,
        /// Copy final artifacts to this directory
        #[arg(short, long, default_value = "./")]
        out_dir: String,
        /// Do not print cargo log messages
        #[arg(short, long)]
        quiet: bool,
        /// Build in parallel without cargo log messages
        #[arg(long)]
        parallel: bool,
    },
    /// Create a new file
    Init {
        #[arg(default_value = "rspit.rs")]
        file_name: String,
        /// Create a new file in the specified directory
        #[arg(short, long, default_value = "./")]
        out_dir: String,
    },
    /// List all packages in the given file
    List { file_path: String },
    /// Add an empty package on top in the given file
    Add { file_path: String },
    /// Extract the package from file
    Extract {
        file_path: String,
        /// Extract this package
        #[arg(short, long)]
        package: String,
        /// Extract the package to the specified directory
        #[arg(short, long, default_value = "./")]
        out_dir: String,
    },
    /// Remove everything in the cache directory
    Clean,
    #[doc(hidden)]
    #[clap(hide = true)]
    ListCaches { file_path: String },
}

pub(crate) fn main() -> Result<()> {
    let args = Args::parse();

    if let Some(command) = args.command {
        match command {
            SubCommands::Check {
                file_path,
                package,
                quiet,
            } => {
                if let Some(package) = package {
                    commands::check_specified_package(file_path, &package, quiet)?;
                } else {
                    commands::check_all(file_path, quiet)?;
                }
            }
            SubCommands::Build {
                file_path,
                package,
                quiet,
                parallel,
            } => {
                if let Some(package) = package {
                    commands::build_specified_package(file_path, &package, quiet)?;
                } else if parallel {
                    commands::build_all_parallel(file_path)?;
                } else {
                    commands::build_all(file_path, quiet)?;
                }
            }
            SubCommands::Run {
                file_path,
                package,
                quiet,
            } => {
                if let Some(package) = package {
                    commands::run_specified_package(file_path, &package, quiet)?;
                } else {
                    commands::run_all(file_path, quiet)?;
                }
            }
            SubCommands::Release {
                file_path,
                package,
                out_dir,
                quiet,
                parallel,
            } => {
                if let Some(package) = package {
                    commands::release_specified_package(file_path, &package, out_dir, quiet)?;
                } else if parallel {
                    commands::release_all_parallel(file_path, out_dir)?;
                } else {
                    commands::release_all(file_path, out_dir, quiet)?;
                }
            }
            SubCommands::Init { file_name, out_dir } => {
                commands::init_snippet(&file_name, out_dir)?;
            }
            SubCommands::List { file_path } => {
                commands::list_packages(file_path)?;
            }
            SubCommands::Add { file_path } => {
                commands::add_package(file_path)?;
            }
            SubCommands::Extract {
                file_path,
                package,
                out_dir,
            } => {
                commands::extract_package(file_path, &package, out_dir)?;
            }
            SubCommands::Clean => {
                commands::clean_cache_dir()?;
            }
            SubCommands::ListCaches { file_path } => {
                commands::list_cached_packages(file_path)?;
            }
        }
    } else {
        let mut cmd = Args::command();
        cmd.print_help()?;
    }

    Ok(())
}
