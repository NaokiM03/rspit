use anyhow::Result;

mod add;
mod build;
mod cache;
mod clean;
mod cli;
mod list;
mod package;
mod run;
mod release;

fn main() -> Result<()> {
    cli::main()?;

    Ok(())
}
