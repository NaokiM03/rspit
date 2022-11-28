use anyhow::Result;

mod cli;
mod commands;
mod core;

fn main() -> Result<()> {
    cli::main()?;

    Ok(())
}
