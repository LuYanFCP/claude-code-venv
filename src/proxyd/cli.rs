use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ccv-d")]
#[command(about = "Claude Code Virtual Environment Daemon - OpenAI to Anthropic Proxy")]
pub struct DaemonCli {
    #[arg(short, long, default_value = "config.toml")]
    pub config: String,

    #[command(subcommand)]
    pub command: Option<DaemonCommands>,
}

#[derive(Subcommand)]
pub enum DaemonCommands {
    /// Start the proxy server
    Start {
        /// Bind address (e.g., 127.0.0.1:3000)
        #[arg(short, long)]
        bind: Option<String>,
    },
    
    /// Test connection to Anthropic API
    Test,
    
    /// Show configuration
    Config,
}