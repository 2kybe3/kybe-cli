use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};

use crate::config::types::Config;

mod default;
pub mod types;

impl Config {
    pub fn load(config_path: Option<PathBuf>) -> anyhow::Result<Config> {
        let path = match config_path {
            Some(config_path) => config_path,
            None => directories::ProjectDirs::from("xyz", "2kybe3", "kybe-cli")
                .ok_or(anyhow!(
                    "no valid home directory path could be retrieved from the operating system."
                ))?
                .config_dir()
                .to_path_buf(),
        };

        Self::load_from_path(path)
    }

    fn load_from_path(path: PathBuf) -> anyhow::Result<Config> {
        let mut config = if !path.exists() {
            Self::create_config(&path)?
        } else {
            let config_str =
                std::fs::read_to_string(&path).context("failed to read config file")?;
            toml::from_str(&config_str).context("failed to parse config (invalid config file)")?
        };

        config.path = Some(path);
        Ok(config)
    }

    fn create_config(path: &Path) -> anyhow::Result<Config> {
        let config = Config::default();
        std::fs::write(path, toml::to_string(&config)?)
            .context("failed to write default config")?;
        Ok(config)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let path = self.path.as_ref().ok_or(anyhow!("config path not set"))?;
        std::fs::write(path, toml::to_string(&self)?).context("failed to write config file")?;
        Ok(())
    }
}
