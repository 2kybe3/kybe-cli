use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub api: ApiConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiConfig {
    pub base_url: String,
    pub timeout_secs: u64,
}
