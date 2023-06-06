use std::fs;
use tauri::{Config, api::path};

use crate::error::TinkarosError;

use super::structs::{AppConfig, LauncherPath, Launcher};

#[tauri::command]
pub fn get_config() -> Result<AppConfig, TinkarosError> {
    let config = path::app_config_dir(&Config::default()).unwrap().join("ahms/config.toml");
    if !config.is_file() {
        fs::create_dir_all(config.parent().unwrap())?;
        return Ok(AppConfig::default());
    }
    let file = fs::read_to_string(&config)?;
    let confs: AppConfig = toml::from_str(&file).unwrap_or_default();
    Ok(confs)
}

pub fn write_config(config: AppConfig) -> Result<AppConfig, TinkarosError> {
    let config_path = path::app_config_dir(&Config::default()).unwrap().join("ahms/config.toml");
    if !config_path.is_file() {
        fs::create_dir_all(config_path.parent().unwrap())?;
        fs::File::create(&config_path).map_err(|_| TinkarosError::ConfigNotFound)?;
    }

    let toml_string = toml::to_string(&config).map_err(|_| TinkarosError::ConfigInvalid)?;
    fs::write(config_path, toml_string)?;
    Ok(config)
}

#[tauri::command]
pub async fn get_launchers() -> Vec<Launcher> {
  let mut found: Vec<Launcher> = Vec::new();

  if LauncherPath::mclauncher().exists() {
    let path = path::app_config_dir(&Config::default()).unwrap().join(r"ahms/game");
    found.push(Launcher::new("default".to_string(), path.to_string_lossy().to_string()));
  }

  if LauncherPath::curseforge().exists() {
    let path = LauncherPath::curseforge_instance();
    found.push(Launcher::new("curseforge".to_string(), path.to_string_lossy().to_string()))
  }

  if LauncherPath::prism().exists() {
    let path = LauncherPath::prism_instance();
    found.push(Launcher::new("prism".to_string(), path.to_string_lossy().to_string()))
  }

  found
}