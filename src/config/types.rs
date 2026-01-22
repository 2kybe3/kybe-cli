use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// Stores the location of the loaded Config
    #[serde(skip_serializing)]
    pub path: Option<PathBuf>,

    pub api: ApiConfig,
    pub generated: GeneratedConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    /// Configuration the API to use (defaults to my 2kybe3 instance hosted at https://kybe.xyz)
    /// See https://github.com/2kybe3/kybe-backend
    pub base_url: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GeneratedConfig {
    pub last_launch: Option<DateTime<Utc>>,
}
