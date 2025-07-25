use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub threads: bool,
    pub color: bool,
    pub timestamp: bool,
    pub file: String,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threads: true,
            color: true,
            timestamp: true,
            file: "latest.log".to_string(),
        }
    }
}
