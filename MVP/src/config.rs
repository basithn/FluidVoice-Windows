use serde::{Deserialize, Serialize};
use std::fs;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkey: String,
    pub record_duration_ms: u64,
    pub audio_device_index: Option<usize>,
    pub openai_api_key: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hotkey: "Ctrl+Shift+V".to_string(),
            record_duration_ms: 5000,
            audio_device_index: None,
            openai_api_key: None,
        }
    }
}

pub fn load_config() -> Result<AppConfig> {
    let path = "config.toml";
    if !std::path::Path::new(path).exists() {
        let config = AppConfig::default();
        let toml = toml::to_string_pretty(&config)?;
        fs::write(path, toml)?;
        println!("Created default config: {}", path);
        return Ok(config);
    }
    
    let content = fs::read_to_string(path)?;
    let config: AppConfig = toml::from_str(&content)?;
    Ok(config)
}
