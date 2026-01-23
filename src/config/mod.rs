use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow, bail};

use crate::config::types::{Config, GeneratedConfig, UserConfig};

mod default;
mod tests;
pub mod types;

const USER_CONFIG_NAME: &str = "config.toml";
const GENERATED_CONFIG_NAME: &str = "generated.toml";

impl Config {
    /// Loads a config from a given path
    pub fn load() -> anyhow::Result<Config> {
        let folder = directories::ProjectDirs::from("xyz", "2kybe3", "kybe-cli")
            .ok_or(anyhow!(
                "no valid home directory path could be retrieved from the operating system."
            ))?
            .config_dir()
            .to_path_buf();

        Self::load_from_path(folder)
    }

    fn load_from_path(path: PathBuf) -> anyhow::Result<Config> {
        let mut user = path.clone();
        user.push(USER_CONFIG_NAME);
        let mut generated = path.clone();
        generated.push(GENERATED_CONFIG_NAME);

        if path.exists() && !path.is_dir() {
            bail!("config path should be a folder but is a file");
        }
        if !path.exists() {
            std::fs::create_dir_all(&path).context("failed to create config folder")?;
        }

        if user.exists() && !user.is_file() {
            bail!("user config path exists but is not a file");
        }
        let user_config: UserConfig = if !user.exists() {
            Self::create_default_user_config(&user)?
        } else {
            let user_config_str =
                std::fs::read_to_string(&user).context("failed to read user config file")?;
            toml::from_str(&user_config_str)
                .context("failed to parse user config (invalid config file)")?
        };

        let generated_config: GeneratedConfig = if !generated.exists() {
            Self::create_default_generated_config(&generated)?
        } else {
            let generated_config_str = std::fs::read_to_string(&generated)
                .context("failed to read generated config file")?;
            toml::from_str(&generated_config_str)
                .context("failed to parse generated config (invalid config file)")?
        };

        let config = Self {
            config_folder: path,
            user_file: user,
            generated_file: generated,
            user: user_config,
            generated: generated_config,
        };

        Ok(config)
    }

    fn create_default_user_config(path: &Path) -> anyhow::Result<UserConfig> {
        let config = UserConfig::default();
        std::fs::write(path, toml::to_string(&config)?)
            .context("failed to write default config")?;
        Ok(config)
    }

    fn create_default_generated_config(path: &Path) -> anyhow::Result<GeneratedConfig> {
        let config = GeneratedConfig::default();
        std::fs::write(path, toml::to_string(&config)?)
            .context("failed to write default config")?;
        Ok(config)
    }

    /// Saves generated config file
    pub fn save(&self) -> anyhow::Result<()> {
        std::fs::write(&self.generated_file, toml::to_string(&self.generated)?)
            .context("failed to write generated config file")?;
        Ok(())
    }
}
