use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub config_folder: PathBuf,
    pub user_file: PathBuf,
    pub generated_file: PathBuf,

    pub user: UserConfig,
    pub generated: GeneratedConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserConfig {
    pub api: ApiConfig,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiConfig {
    /// Configuration the API to use (defaults to my 2kybe3 instance hosted at https://kybe.xyz)
    /// See https://github.com/2kybe3/kybe-backend
    pub base_url: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct GeneratedConfig {
    pub last_launch: Option<DateTime<Utc>>,
}
