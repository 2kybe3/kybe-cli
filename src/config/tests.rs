#![cfg(test)]

use std::fs::File;

use chrono::Utc;
use tempfile::{TempDir, tempdir};

use super::*;

fn config_in_temp_dir() -> (Config, TempDir) {
    let dir = tempdir().unwrap();
    let config = Config::load_from_path(dir.path().to_path_buf()).unwrap();
    (config, dir)
}

#[test]
fn creates_config_folder_if_missing() {
    let dir = tempdir().unwrap();
    let folder = dir.path().to_path_buf();
    assert!(folder.exists());

    std::fs::remove_dir_all(&folder).unwrap();

    assert!(!folder.exists());

    let result = Config::load_from_path(folder.clone());
    assert!(result.is_ok());

    assert!(folder.exists());
    assert!(folder.is_dir());
}

#[test]
fn creates_user_and_generated_configs_if_missing() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_path_buf();

    let config = Config::load_from_path(path.clone()).unwrap();

    assert!(config.user_file.exists());
    assert!(config.generated_file.exists());

    assert!(config.user_file.is_file());
    assert!(config.generated_file.is_file());
}

#[test]
fn rejects_config_path_that_is_a_file() {
    let dir = tempdir().unwrap();
    let fake_config_path = dir.path().join("fake");
    File::create(&fake_config_path).unwrap();

    let err = Config::load_from_path(fake_config_path);
    assert!(err.is_err())
}

#[test]
fn loads_existing_config_files() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_path_buf();

    let user_path = path.join(USER_CONFIG_NAME);
    let generated_path = path.join(GENERATED_CONFIG_NAME);

    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(&user_path, toml::to_string(&UserConfig::default()).unwrap()).unwrap();
    std::fs::write(
        &generated_path,
        toml::to_string(&GeneratedConfig::default()).unwrap(),
    )
    .unwrap();

    let config = Config::load_from_path(path).unwrap();

    assert!(config.user_file.exists());
    assert!(config.generated_file.exists());
}

#[test]
fn rejects_user_config_path_that_is_directory() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_path_buf();

    let user_path = path.join(USER_CONFIG_NAME);
    std::fs::create_dir_all(&user_path).unwrap();

    let err = Config::load_from_path(path);
    assert!(err.is_err());
}

#[test]
fn rejects_generated_config_path_that_is_directory() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_path_buf();

    let generated_path = path.join(GENERATED_CONFIG_NAME);
    std::fs::create_dir_all(&generated_path).unwrap();

    let err = Config::load_from_path(path);
    assert!(err.is_err());
}

#[test]
fn rejects_invalid_user_config_toml() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_path_buf();

    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(path.join(USER_CONFIG_NAME), "this = = invalid").unwrap();

    let err = Config::load_from_path(path);
    assert!(err.is_err());
}

#[test]
fn rejects_invalid_generated_config_toml() {
    let dir = tempdir().unwrap();
    let path = dir.path().to_path_buf();

    std::fs::create_dir_all(&path).unwrap();
    std::fs::write(
        path.join(USER_CONFIG_NAME),
        toml::to_string(&UserConfig::default()).unwrap(),
    )
    .unwrap();

    std::fs::write(path.join(GENERATED_CONFIG_NAME), "not = toml = {").unwrap();

    let err = Config::load_from_path(path);
    assert!(err.is_err());
}

#[test]
fn save_writes_generated_config_to_disk() {
    let (mut config, _dir) = config_in_temp_dir();

    let original = toml::to_string(&config.generated).unwrap();

    config.generated = GeneratedConfig::default();
    config.generated.last_launch = Some(Utc::now());

    let updated = toml::to_string(&config.generated).unwrap();
    assert_ne!(
        original, updated,
        "generated config should differ after mutation"
    );

    config.save().unwrap();

    let on_disk =
        std::fs::read_to_string(&config.generated_file).expect("failed to read generated config");

    assert_eq!(on_disk, updated);
}
