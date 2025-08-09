use anyhow::Result;
use colored::*;
use dialoguer::{Confirm, Input};
use std::collections::HashMap;

use crate::config::Config;

pub struct EnvironmentManager {
    config: Config,
}

impl EnvironmentManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn list_environments(&self) -> Result<()> {
        let environments = self.config.list_environments();
        let current = self.config.get_current_env_name();

        if environments.is_empty() {
            println!("{} No environments configured", "!".yellow());
            println!("Use {} to create one", "ccv create".cyan());
            return Ok(());
        }

        println!("{} Available environments:", "*".blue());

        for env in environments {
            let is_current = current.as_ref() == Some(&env.name);
            let indicator = if is_current {
                "→".green().to_string()
            } else {
                "  ".to_string()
            };

            print!("{} {} ", indicator, env.name.bold());

            if let Some(desc) = &env.description {
                print!("- {}", desc);
            }

            if is_current {
                print!(" {}", "(current)".green().italic());
            }

            println!();

            if !env.variables.is_empty() {
                for (key, value) in &env.variables {
                    println!("    {}={}", key.dimmed(), value.dimmed());
                }
            }
        }

        if let Some(global) = &self.config.global_env {
            println!("\n{} Global environment: {}", "*".blue(), global.bold());
        }

        Ok(())
    }

    pub fn create_interactive(&mut self, name: Option<String>) -> Result<()> {
        let name = match name {
            Some(n) => n,
            None => Input::new()
                .with_prompt("Environment name")
                .interact_text()?,
        };

        if self.config.get_environment(&name).is_some() {
            anyhow::bail!("Environment '{}' already exists", name);
        }

        let description = Some("Anthropic Claude Code configuration".to_string());

        let mut variables = HashMap::new();

        println!(
            "\n{} Configuring Anthropic environment variables:",
            "*".blue()
        );

        let anthropic_url: String = Input::new()
            .with_prompt("ANTHROPIC_BASE_URL")
            .default("https://api.anthropic.com".to_string())
            .interact_text()?;
        variables.insert("ANTHROPIC_BASE_URL".to_string(), anthropic_url);

        let auth_token: String = Input::new()
            .with_prompt("ANTHROPIC_AUTH_TOKEN")
            .interact_text()?;
        variables.insert("ANTHROPIC_AUTH_TOKEN".to_string(), auth_token);

        let model: String = Input::new()
            .with_prompt("ANTHROPIC_MODEL")
            .default("claude-3-5-sonnet-20241022".to_string())
            .interact_text()?;
        variables.insert("ANTHROPIC_MODEL".to_string(), model);

        let small_fast_model: String = Input::new()
            .with_prompt("ANTHROPIC_SMALL_FAST_MODEL")
            .allow_empty(true)
            .default("claude-3-haiku-20240307".to_string())
            .interact_text()?;

        if !small_fast_model.is_empty() {
            variables.insert("ANTHROPIC_SMALL_FAST_MODEL".to_string(), small_fast_model);
        }

        self.config
            .add_environment(name.clone(), variables, description)?;

        println!(
            "{} Environment '{}' created successfully",
            "✓".green(),
            name.bold()
        );

        let set_global = Confirm::new()
            .with_prompt("Set as global default environment?")
            .default(true)
            .interact()?;

        if set_global {
            self.config.set_global(&name)?;
            println!("{} Set {} as global environment", "✓".green(), name.bold());
        }

        Ok(())
    }

    pub fn set_global(&mut self, name: &str) -> Result<()> {
        if self.config.get_environment(name).is_none() {
            anyhow::bail!("Environment '{}' does not exist", name);
        }

        self.config.set_global(name)?;
        println!("{} Set global environment to {}", "✓".green(), name.bold());

        Ok(())
    }

    pub fn show_current(&self) -> Result<()> {
        match self.config.get_current_env_name() {
            Some(name) => {
                println!("{} Current environment: {}", "*".blue(), name.bold());

                if let Some(env) = self.config.get_environment(&name) {
                    if let Some(desc) = &env.description {
                        println!("  Description: {}", desc);
                    }

                    if !env.variables.is_empty() {
                        println!("  Variables:");
                        for (key, value) in &env.variables {
                            println!("    {}={}", key.green(), value);
                        }
                    }
                }
            }
            None => {
                println!("{} No active environment", "!".yellow());
                println!("Use {} to set one", "ccv global <env>".cyan());
            }
        }

        Ok(())
    }

    pub fn remove(&mut self, name: &str) -> Result<()> {
        if self.config.get_environment(name).is_none() {
            anyhow::bail!("Environment '{}' does not exist", name);
        }

        let confirm = Confirm::new()
            .with_prompt(format!("Remove environment '{}'?", name))
            .default(false)
            .interact()?;

        if !confirm {
            println!("{} Cancelled", "!".yellow());
            return Ok(());
        }

        self.config.remove_environment(name)?;
        println!("{} Environment '{}' removed", "✓".green(), name.bold());

        Ok(())
    }
}
