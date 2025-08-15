use anyhow::Result;
use colored::*;
use std::env;
#[cfg(unix)]
use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::config::Config;

pub fn activate(env_name: Option<String>, config: &Config) -> Result<()> {
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
    
    #[cfg(windows)]
    let shell_cmd = match shell.as_str() {
        "cmd" => "cmd",
        "powershell" => "powershell",
        "pwsh" => "pwsh", // PowerShell Core
        _ => "powershell",
    };
    
    #[cfg(not(windows))]
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

    #[cfg(windows)]
    {
        // On Windows, spawn a new shell with appropriate arguments
        let mut command = Command::new(shell_cmd);
        
        match shell_cmd {
            "cmd" => {
                command.arg("/k").arg("echo Environment activated!");
            }
            "powershell" | "pwsh" => {
                command.arg("-NoExit").arg("-Command").arg("Write-Host 'Environment activated!'");
            }
            _ => {
                command.arg("-NoExit");
            }
        }
        
        let status = command
            .status()
            .expect("Failed to execute shell");
        std::process::exit(status.code().unwrap_or(0));
    }
    
    #[cfg(not(any(unix, windows)))]
    {
        // On other non-Unix systems, spawn a new shell
        let status = Command::new(shell_cmd)
            .status()
            .expect("Failed to execute shell");
        std::process::exit(status.code().unwrap_or(0));
    }
}

fn detect_shell() -> String {
    #[cfg(windows)]
    {
        // On Windows, check COMSPEC or use PowerShell as default
        std::env::var("COMSPEC")
            .or_else(|_| Ok("powershell".to_string()))
            .unwrap_or_else(|_: String| "powershell".to_string())
            .split('\\')
            .next_back()
            .unwrap_or("powershell")
            .to_string()
            .to_lowercase()
    }
    
    #[cfg(not(windows))]
    {
        // On Unix-like systems, check SHELL environment variable
        std::env::var("SHELL")
            .unwrap_or_else(|_| "unknown".to_string())
            .split('/')
            .next_back()
            .unwrap_or("unknown")
            .to_string()
    }
}
