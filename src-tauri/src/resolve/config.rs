use std::fs;
use tauri::{Config, api::path};

use super::structs::{AppConfig, LauncherPath, Launcher};

#[tauri::command]
pub fn get_config() -> AppConfig {
    let config = path::app_config_dir(&Config::default()).expect("Couldnt load config").join("ahms/config.toml");
    if !config.is_file() {
        fs::create_dir_all(config.parent().unwrap()).expect("unable to create config parent dirs");
        return AppConfig::default();
    }
    let file = fs::read_to_string(&config).expect("unable to read config");
    let confs: AppConfig = toml::from_str(&file).unwrap_or_default();
    confs
}

pub fn write_config(config: AppConfig) -> AppConfig {
    let config_path = path::app_config_dir(&Config::default()).expect("Couldnt load config").join("ahms/config.toml");
    if !config_path.is_file() {
        fs::create_dir_all(config_path.parent().unwrap()).expect("unable to create parent directory of file");
        fs::File::create(&config_path).expect("unable to create file");
    }

    let toml_string = toml::to_string(&config).expect("unable to convert back to toml");
    fs::write(config_path, toml_string).expect("unable to write file");
    config
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