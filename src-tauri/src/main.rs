#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs::{self, File}, path::PathBuf, env::var, process::Command, time::{SystemTime, UNIX_EPOCH}};
use mci_reloaded::{resolve, update_files, update_status, resolve_configs, LauncherPath, update_progress};
use tauri::{api::path, Config};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    init: bool,
    launcher: String,
    path: String,
    custom: bool
}

#[derive(Serialize, Deserialize, Default)]
struct VersionFile {
  version: String,
  last_updated: u64
}

#[derive(Serialize)]
struct VersionRes {
  version: String,
  latest_version: String,
  last_updated: u64
}

#[derive(Serialize)]
struct Launcher {
  name: String,
  path: String
}

impl Launcher {
    fn new(name: String, path: String) -> Self { Self { name, path } }
}

#[tauri::command]
fn get_config() -> AppConfig {
    let mut config = path::app_config_dir(&Config::default()).expect("Couldnt load config");
    config.push("ahms/config.toml");
    if !config.is_file() {
      return AppConfig {init: false, launcher: "none".to_string(), path: "none".to_string(), custom: false};
    }
    let file = fs::read_to_string(&config).expect("unable to read config");
    let confs: AppConfig = toml::from_str(&file).unwrap();
    confs
}

#[tauri::command]
fn init(chosen: &str, path: &str, custom: bool) {
  let mut _path = path;
  let _p = PathBuf::from(var("APPDATA").unwrap()).join(r"ahms\game");
  if _path.is_empty() {
    _path = _p.to_str().unwrap()
  }
  let mut config = path::app_config_dir(&Config::default()).expect("Couldnt load config");
  config.push("ahms/config.toml");
  if !config.is_file() {
    fs::create_dir_all(config.parent().unwrap()).expect("unable to create parent directory of file");
    File::create(&config).expect("unable to create file");
  }
  let str_file = fs::read_to_string(&config).expect("unable to read config");
  let mut confs: AppConfig = toml::from_str(&str_file).unwrap_or_default();

  confs.init = true;
  confs.launcher = chosen.to_string();
  confs.path = _path.to_string();
  confs.custom = custom;
  resolve(&PathBuf::from(_path));


  let toml_string = toml::to_string(&confs).expect("unable to convert back to toml");
  fs::write(&config, toml_string).expect("unable to write file");
}

#[tauri::command]
async fn get_launchers() -> Vec<Launcher> {
  let mut found: Vec<Launcher> = Vec::new();

  if LauncherPath::mclauncher().exists() {
    let path =  PathBuf::from(var("APPDATA").unwrap()).join(r"ahms\game");
    found.push(Launcher::new("default".to_string(), path.to_string_lossy().to_string()))

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

#[tauri::command]
async fn update(app: tauri::AppHandle, launcher: String, path: String, custom: bool) {
  let _path = PathBuf::from(path);

  update_status("resolving location", &app);
  resolve(&_path);

  update_status("preparing", &app);
  update_files(&_path, &app).await;
  
  update_status("adding required configs", &app);
  resolve_configs(&app, &_path, launcher, custom).await;

  update_status("done!", &app);
  update_progress(100, &app);
}

#[tauri::command]
async fn log_update(path: String) {
  let mut comment = false;
  let file = PathBuf::from(path).join("version.toml");
  if !file.exists() {
    File::create(&file).expect("unable to create file");
    comment = true;
  }

  let str_file = fs::read_to_string(&file).expect("unable to read config");
  let mut data: VersionFile = toml::from_str(&str_file).unwrap_or_default();

  data.version = reqwest::get("https://raw.githubusercontent.com/Hbarniq/ahms/main/VERSION").await.unwrap().text().await.unwrap();
  data.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).expect("unable to get unix epoch").as_secs();

  let mut toml_string = toml::to_string(&data).expect("unable to convert back to toml");
  if comment { toml_string = "#needed for version checking DO NOT TOUCH\n".to_owned() + &toml_string }
  fs::write(&file, toml_string).expect("unable to write file");
}

#[tauri::command]
async fn get_version(path: String) -> VersionRes {
  let file = PathBuf::from(path).join("version.toml");
  let latest = reqwest::get("https://raw.githubusercontent.com/Hbarniq/ahms/main/VERSION").await.unwrap().text().await.unwrap();
  
  if file.exists() {
    let str_file = fs::read_to_string(&file).expect("unable to read config");
    let file_data: VersionFile = toml::from_str(&str_file).unwrap_or_default();
    VersionRes { version: file_data.version, latest_version: latest, last_updated: file_data.last_updated }
  } else {
    VersionRes { version: "not installed".to_string(), latest_version: latest, last_updated: 0 }
  }
}

#[tauri::command]
fn explorer(path: &str) {
  Command::new("explorer").args([path]).spawn().unwrap();
}

#[tauri::command]
fn check_installed(path: &str) -> bool {
  let mut installed = true;
  let _path = PathBuf::from(path);
  resolve(&_path);
  if _path.read_dir().unwrap().next().is_none() {
    installed = false
  }
  installed
}


//async fn login(selected: &str, username: &str, email: &str, password: &str) {
//  // microsoft auth costs money so not doing this yet..
//
//  // some maybe helpful resources
//  // https://wiki.vg/Microsoft_Authentication_Scheme
//  // https://github.com/dommilosz/minecraft-auth/blob/master/src/index.ts
//  // https://github.com/gorilla-devs/GDLauncher/blob/7fd15c7d75de8af1416c53c6df3acc69fc3b0fdb/src/common/reducers/actions.js#L463
//}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init, get_config, get_launchers, update, log_update, get_version, explorer, check_installed])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
