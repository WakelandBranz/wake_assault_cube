use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TabMenuConfig {
    menu_settings: MenuSettings,
    configuration: Configuration,
}

#[derive(Serialize, Deserialize)]
pub struct MenuSettings {
    example: String,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    example: String,
}

impl Default for TabMenuConfig {
    fn default() -> Self {
        Self {
            menu_settings: MenuSettings::default(),
            configuration: Configuration::default(),
        }
    }
}

impl Default for MenuSettings {
    fn default() -> Self {
        Self {
            example: "example".to_string(),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            example: "example".to_string(),
        }
    }
}