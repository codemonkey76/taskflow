use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub selected_script: Option<String>,
    pub output_directory: Option<PathBuf>,
    pub logging_enabled: bool,
    pub available_scripts: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            selected_script: None,
            output_directory: None,
            logging_enabled: false,
            available_scripts: vec![],
        }
    }
}

impl Config {
    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if let Ok(contents) = std::fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str(&contents) {
                    return config;
                }
            }
        }
        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let contents = serde_json::to_string_pretty(self)?;
            std::fs::write(path, contents)?;
        }
        Ok(())
    }

    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("taskflow").join("config.json"))
    }
}
