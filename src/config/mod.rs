/// All feature settings are stored in here. Menu state is stored by egui persistence.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::cheat::features::Features;

#[derive(Debug)]
pub enum ConfigError {
    FailedToSave,
    FailedToLoad,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Config {
    pub features: Features,
}

impl Config {
    const CONFIG_PATH: &'static str = "config.json";

    pub fn save(&self) -> Result<(), ConfigError> {
        // Retrieve config json as String
        let json = match serde_json::to_string_pretty(self) {
            Ok(json) => json,
            Err(e) => {
                log::error!("Failed to serialize config: {}", e);
                return Err(ConfigError::FailedToSave);
            }
        };

        // Write String json to file
        match fs::write(Self::CONFIG_PATH, json) {
            Ok(_) => {
                log::debug!("Config saved successfully to {}", Self::CONFIG_PATH);
                Ok(())
            }
            Err(e) => {
                log::error!("Failed to write config file: {}", e);
                Err(ConfigError::FailedToSave)
            }
        }
    }

    pub fn load() -> Result<Self, ConfigError> {
        if !Path::new(Self::CONFIG_PATH).exists() {
            log::debug!("No config file found, creating default config");
            let config = Config::default();
            config.save()?;
            return Ok(config);
        }

        // Retrieve config json as String
        let json = match fs::read_to_string(Self::CONFIG_PATH) {
            Ok(json) => json,
            Err(e) => {
                log::error!("Failed to read config file: {}", e);
                return Err(ConfigError::FailedToLoad);
            }
        };

        // Load existing config
        match serde_json::from_str(&json) {
            Ok(config) => {
                log::debug!("Config loaded successfully from {}", Self::CONFIG_PATH);
                Ok(config)
            },
            Err(e) => {
                log::error!("Failed to deserialize config: {}", e);
                Err(ConfigError::FailedToLoad)
            }
        }
    }
}
