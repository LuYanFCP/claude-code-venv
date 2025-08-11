use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DaemonConfig {
    pub server: ServerConfig,
    pub providers: Vec<OpenAICompatible>,
    pub proxy: ProxyConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub cors_origins: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAICompatible {
    pub base_url: String,
    pub models: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxyConfig {
    pub timeout_seconds: Option<u64>,
    pub max_request_size: Option<u64>,
    pub rate_limit: Option<u32>,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                bind_addr: "127.0.0.1:3000".to_string(),
                cors_origins: Some(vec!["*".to_string()]),
            },
            providers: vec![],
            proxy: ProxyConfig {
                timeout_seconds: Some(30),
                max_request_size: Some(1024 * 1024), // 1MB
                rate_limit: Some(100),
            },
        }
    }
}

pub fn load_config(path: &str) -> Result<DaemonConfig> {
    let path = Path::new(path);
    
    if !path.exists() {
        let default_config = DaemonConfig::default();
        let toml = toml::to_string_pretty(&default_config)?;
        fs::write(path, toml)?;
        println!("Created default config file at {}", path.display());
        return Ok(default_config);
    }
    
    let contents = fs::read_to_string(path)?;
    let config: DaemonConfig = toml::from_str(&contents)?;
    
    Ok(config)
}