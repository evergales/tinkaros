#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub mod update;
pub mod resolve;

use std::{fs::{self, File}, path::PathBuf, env::consts, process::Command, time::{SystemTime, UNIX_EPOCH}};
use serde::{Serialize, Deserialize};
use resolve::{structs::{AppConfig, ResolveData, Tinkaros, ModIdentifier}, config::{write_config, get_config}};
use update::{mods::{update_mods, get_projects_from_ids}, status::{update_progress, update_status}, configs::resolve_configs, structs::CombinedProjects};

use crate::resolve::config::get_launchers;

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

#[tauri::command]
fn init(chosen: String, path: String, custom: bool) {
  write_config(AppConfig::new(true, chosen, path, custom));
}

#[tauri::command]
async fn update(app: tauri::AppHandle, launcher: String, path: String, custom: bool) {
  let _path = PathBuf::from(path);
  fs::create_dir_all(_path.join("mods")).expect("unable to resolve dirs");

  update_status("preparing", &app);
  update_mods(&_path, &app).await;
  
  update_status("adding required configs", &app);
  resolve_configs(&app, &_path, launcher, custom).await;

  update_status("done!", &app);
  update_progress(100, &app);
}

#[tauri::command]
async fn list_mod_projects(app: tauri::AppHandle) -> Vec<CombinedProjects> {
  let data = ResolveData::get().await.expect("unable to resolve configs");

  let mut modrinth_ids: Vec<String> = Vec::new();
  let mut curseforge_ids: Vec<i32> = Vec::new();

  for mod_item in &data.modpack.mods {
      match &mod_item.identifier {
          ModIdentifier::ModrinthProject(id) => modrinth_ids.push(id.clone()),
          ModIdentifier::CurseForgeProject(id) => curseforge_ids.push(id.to_owned()),
      }
  }

  let mut result = get_projects_from_ids(modrinth_ids, curseforge_ids, &app).await.expect("unable to list mods");

  result.sort_by_key(|project| match project {
    CombinedProjects::ModrinthProject(project) => project.slug.to_owned(),
    CombinedProjects::CurseForgeMod(mod_) => mod_.slug.to_owned()
  });

  result
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

  data.version = ResolveData::get().await.unwrap().modpack.version;
  data.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).expect("unable to get unix epoch").as_secs();

  let mut toml_string = toml::to_string(&data).expect("unable to convert back to toml");
  if comment { toml_string = "#needed for version checking DO NOT TOUCH\n".to_owned() + &toml_string }
  fs::write(&file, toml_string).expect("unable to write file");
}

#[tauri::command]
async fn get_version(path: String) -> VersionRes {
  let file = PathBuf::from(path).join("version.toml");
  let latest = ResolveData::get().await.unwrap().modpack.version;
  
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
  match consts::OS {
    "windows" => { Command::new("explorer").args([path]).spawn().unwrap(); },
    "linux" => { Command::new("xdg-open").args([path]).spawn().unwrap(); },
    _ => {}
  }
}

#[tauri::command]
fn check_installed(path: &str) -> bool {
  let mut installed = true;
  let _path = PathBuf::from(path);
  fs::create_dir_all(&_path).expect("unable to resolve dirs");
  if _path.read_dir().unwrap().next().is_none() {
    installed = false
  }
  installed
}

#[tauri::command]
async fn check_update(app: tauri::AppHandle) -> (bool, Tinkaros) {
  let latest = ResolveData::get().await.expect("unable to get latest version").tinkaros;
  if tauri::api::version::is_greater(app.package_info().version.to_string().as_str(), &latest.version).unwrap() {
    (true, latest)
  } else {
    (false, latest)
  }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![init, get_config, get_launchers, update, log_update, get_version, list_mod_projects, explorer, check_installed, check_update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
