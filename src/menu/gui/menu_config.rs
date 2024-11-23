use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TabMenuConfig {
    menu_settings: MenuSettings,
    configuration: Configuration,
}

impl Default for TabMenuConfig {
    fn default() -> Self {
        Self {
            menu_settings: MenuSettings::default(),
            configuration: Configuration::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MenuSettings {}

#[derive(Serialize, Deserialize)]
pub struct Configuration {}

impl Default for MenuSettings {
    fn default() -> Self {
        Self {}
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {}
    }
}