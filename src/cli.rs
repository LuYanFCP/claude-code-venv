use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::config::Config;
use crate::env::EnvironmentManager;
use crate::shell;

#[derive(Parser)]
#[command(name = "ccv")]
#[command(
    about = "Claude Code Environment Version Manager, default config file in ~/.claude-code-env."
)]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Optional config file path
    #[arg(short, long, global = true)]
    pub config_file: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
    // TODO: Add shell completion generation command for bash/zsh/fish
    // TODO: Add export/import functionality for environment configurations
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all available environments
    Envs,

    /// Create a new environment interactively
    Create {
        /// Optional environment name
        name: Option<String>,
    },

    /// Set global default environment
    Global {
        /// Environment name to set as global
        version: String,
    },

    /// Activate environment in shell
    Shell {
        /// Environment name to activate
        env_name: Option<String>,
    },

    /// Show current active environment
    Current,

    /// Remove an environment
    Remove {
        /// Environment name to remove
        name: String,
    },
}

impl Cli {
    pub fn run(self) -> Result<()> {
        let config = Config::load(self.config_file)?;
        let mut manager = EnvironmentManager::new(config.clone());

        match self.command {
            Commands::Envs => {
                manager.list_environments()?;
            }
            Commands::Create { name } => {
                manager.create_interactive(name)?;
            }
            Commands::Global { version } => {
                manager.set_global(&version)?;
            }
            Commands::Shell { env_name } => {
                shell::activate(env_name, &config)?;
            }
            Commands::Current => {
                manager.show_current()?;
            }
            Commands::Remove { name } => {
                manager.remove(&name)?;
            }
        }

        Ok(())
    }
}
