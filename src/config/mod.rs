use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow, bail};

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

    pub fn reload(&mut self) -> anyhow::Result<()> {
        let path = match &self.path {
            Some(p) => p,
            None => bail!("loaded config doesn't have a path associated"),
        };

        if !path.exists() {
            bail!("config didn't exist when reloading");
        }

        let config_str = std::fs::read_to_string(path).context("failed to read config file")?;
        let mut new_cfg: Config =
            toml::from_str(&config_str).context("failed to parse config (invalid config file)")?;

        new_cfg.path = Some(path.clone());

        *self = new_cfg;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_default_config_if_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");

        let cfg = Config::load(Some(path.clone())).unwrap();

        assert!(path.exists());
        assert_eq!(cfg.path.as_ref(), Some(&path));
    }

    #[test]
    fn save_and_reload_roundtrip() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");

        let mut cfg = Config::load(Some(path.clone())).unwrap();
        cfg.save().unwrap();

        let original = cfg.clone();
        cfg.reload().unwrap();

        assert_eq!(cfg, original);
    }

    #[test]
    fn reload_fails_if_file_missing() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");

        let mut cfg = Config::load(Some(path.clone())).unwrap();
        std::fs::remove_file(&path).unwrap();

        assert!(cfg.reload().is_err());
    }

    #[test]
    fn invalid_toml_fails() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("config.toml");

        std::fs::write(&path, "not = valid = toml").unwrap();

        let err = Config::load(Some(path)).unwrap_err();
        assert!(err.to_string().contains("parse"));
    }
}
