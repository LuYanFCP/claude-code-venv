use anyhow::Result;
use clap::Parser;

mod cli;
mod config;
mod env;
mod shell;

use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run()
}
