use anyhow::Result;
use colored::*;
use std::env;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::config::Config;

pub fn activate(env_name: Option<String>, config: &Config) -> Result<()> {
    // TODO: Add dry-run mode to preview environment changes without activation
    // TODO: Add environment diff/compare functionality for troubleshooting
    
    let env_name = env_name
        .or_else(|| config.get_current_env_name())
        .ok_or_else(|| anyhow::anyhow!("No environment specified and no global environment set"))?;

    let Some(env) = config.get_environment(&env_name) else {
        anyhow::bail!("Environment '{}' does not exist", env_name);
    };

    // Set environment variables in current process
    unsafe {
        env::set_var("CLAUDE_CODE_ENV", &env_name);
        for (key, value) in &env.variables {
            env::set_var(key, value);
        }
    }

    // Start a new shell with the environment variables set
    let shell = detect_shell();
    let shell_cmd = match shell.as_str() {
        "bash" => "bash",
        "zsh" => "zsh",
        "fish" => "fish",
        _ => "sh",
    };

    println!(
        "{} Environment '{}' activated",
        "✓".green(),
        env_name.bold()
    );
    println!(
        "{} Starting new shell with environment variables set...",
        "*".blue()
    );

    #[cfg(unix)]
    {
        // On Unix-like systems, use exec to replace current process
        let err = Command::new(shell_cmd).exec();
        anyhow::bail!("Failed to start shell: {}", err)
    }

    #[cfg(not(unix))]
    {
        // On non-Unix systems, spawn a new shell
        let status = Command::new(shell_cmd)
            .status()
            .expect("Failed to execute shell");
        std::process::exit(status.code().unwrap_or(0));
    }
}

fn detect_shell() -> String {
    std::env::var("SHELL")
        .unwrap_or_else(|_| "unknown".to_string())
        .split('/')
        .next_back()
        .unwrap_or("unknown")
        .to_string()
}
