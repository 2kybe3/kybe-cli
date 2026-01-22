use crate::config::types::ApiConfig;

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "https://kybe.xyz".into(),
            timeout_secs: 5,
        }
    }
}
