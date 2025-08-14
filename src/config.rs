use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environment {
    pub name: String,
    pub variables: HashMap<String, String>,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub environments: HashMap<String, Environment>,
    pub global_env: Option<String>,
    pub config_path: PathBuf,
}

impl Config {
    pub fn load(config_file: Option<PathBuf>) -> Result<Self> {
        let config_path = config_file.unwrap_or_else(Self::get_default_config_path);

        if !config_path.exists() {
            let mut config = Self::default();
            config.config_path = config_path;
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&config_path).context("Failed to read config file")?;

        let mut config: Config = toml::from_str(&content).context("Failed to parse config file")?;

        config.config_path = config_path;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let dir = self.config_path.parent().context("Invalid config path")?;

        if !dir.exists() {
            fs::create_dir_all(dir).context("Failed to create config directory")?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&self.config_path, content).context("Failed to write config file")?;

        Ok(())
    }

    pub fn get_default_config_path() -> PathBuf {
        let home = dirs::home_dir().expect("Failed to get home directory");
        home.join(".claude-code-env.toml")
    }

    pub fn add_environment(
        &mut self,
        name: String,
        variables: HashMap<String, String>,
        description: Option<String>,
    ) -> Result<()> {
        let now = chrono::Utc::now();

        let env = Environment {
            name: name.clone(),
            variables,
            description,
            created_at: now,
            updated_at: now,
        };

        self.environments.insert(name, env);
        self.save()?;
        Ok(())
    }

    pub fn remove_environment(&mut self, name: &str) -> Result<()> {
        self.environments.remove(name);

        if self.global_env.as_deref() == Some(name) {
            self.global_env = None;
        }

        self.save()?;
        Ok(())
    }

    pub fn set_global(&mut self, name: &str) -> Result<()> {
        if !self.environments.contains_key(name) {
            anyhow::bail!("Environment '{}' does not exist", name);
        }

        self.global_env = Some(name.to_string());
        self.save()?;
        Ok(())
    }

    pub fn get_environment(&self, name: &str) -> Option<&Environment> {
        self.environments.get(name)
    }

    pub fn list_environments(&self) -> Vec<&Environment> {
        self.environments.values().collect()
    }

    pub fn get_current_env_name(&self) -> Option<String> {
        if let Ok(env) = std::env::var("CLAUDE_CODE_ENV") {
            return Some(env);
        }
        self.global_env.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            environments: HashMap::new(),
            global_env: None,
            config_path: Self::get_default_config_path(),
        }
    }
}
