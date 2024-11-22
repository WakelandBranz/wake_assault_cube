use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TabMisc {
    example: String
}

impl Default for TabMisc {
    fn default() -> Self {
        Self {
            example: "example".to_string(),
        }
    }
}