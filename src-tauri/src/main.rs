#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

pub mod update;
pub mod resolve;
pub mod state;
pub mod error;

use std::{fs::{self, File}, path::PathBuf, env::consts, process::Command, time::{SystemTime, UNIX_EPOCH}};
use error::TinkarosError;
use serde::{Serialize, Deserialize};
use resolve::{structs::{AppConfig, ModIdentifier, GithubRelease}, config::{write_config, get_config}};
use state::State;
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
fn init(chosen: String, path: String) -> Result<(), TinkarosError> {
  write_config(AppConfig::new(true, chosen, path, true, 75, false))?;
  Ok(())
}

#[tauri::command]
async fn check_online() -> Result<(), TinkarosError> {
  let res = reqwest::get("https://www.cloudflare.com/").await.map_err(|_| TinkarosError::NetworkTimeout)?;
  match res.status() {
    reqwest::StatusCode::OK => Ok(()),
    _ => Err(TinkarosError::NetworkTimeout)
  }
}

#[tauri::command]
async fn update(app: tauri::AppHandle, launcher: String, path: String) -> Result<(), TinkarosError> {
  check_online().await?;
  let path = PathBuf::from(path);
  fs::create_dir_all(path.join("mods"))?;

  update_status("preparing", &app)?;
  update_mods(&path, &app).await?;
  
  update_status("adding required configs", &app)?;
  resolve_configs(&app, &path, launcher).await?;

  update_status("done!", &app)?;
  update_progress(100, &app)?;

  Ok(())
}

#[tauri::command]
async fn list_mod_projects(limit: usize, app: tauri::AppHandle) -> Result<Vec<CombinedProjects>, TinkarosError> {
  let data = State::get().await?;

  let mut modrinth_ids: Vec<String> = Vec::new();
  let mut curseforge_ids: Vec<i32> = Vec::new();

  let lower_bound = std::cmp::max(limit as i32 - 25, 0) as usize;
  let upper_bound = std::cmp::min(limit, data.modpack.mods.len());

  for mod_item in &data.modpack.mods[lower_bound..upper_bound] {
      match &mod_item.identifier {
          ModIdentifier::ModrinthProject(id) => modrinth_ids.push(id.clone()),
          ModIdentifier::CurseForgeProject(id) => curseforge_ids.push(id.to_owned()),
      }
  }

  let mut result = get_projects_from_ids(modrinth_ids, curseforge_ids, &app).await?;

  result.sort_by_key(|project| match project {
    CombinedProjects::ModrinthProject(project) => project.slug.to_owned(),
    CombinedProjects::CurseForgeMod(mod_) => mod_.slug.to_owned()
  });

  Ok(result)
}

#[tauri::command]
async fn log_update(path: String) -> Result<(), TinkarosError> {
  let mut should_add_comment = false;
  
  let file = PathBuf::from(path).join("version.toml");
  if !file.exists() {
    File::create(&file)?;
    should_add_comment = true;
  }

  let str_file = fs::read_to_string(&file)?;
  let mut data: VersionFile = toml::from_str(&str_file).unwrap_or_default();

  data.version = State::get().await.unwrap().modpack.version.clone();
  data.last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

  let mut toml_string = toml::to_string(&data).unwrap();
  if should_add_comment { toml_string = "#needed for version checking DO NOT TOUCH\n".to_owned() + &toml_string }
  fs::write(&file, toml_string)?;

  Ok(())
}

#[tauri::command]
async fn get_version(path: String) -> Result<VersionRes, TinkarosError> {
  let file = PathBuf::from(path).join("version.toml");
  let latest = State::get().await?.modpack.version.clone();
  
  if file.exists() {
    let str_file = fs::read_to_string(&file)?;
    let file_data: VersionFile = toml::from_str(&str_file).unwrap_or_default();
    Ok(VersionRes { version: file_data.version, latest_version: latest, last_updated: file_data.last_updated })
  } else {
    Ok(VersionRes { version: "not installed".to_string(), latest_version: latest, last_updated: 0 })
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
fn check_modpack_installed(path: &str) -> Result<bool, TinkarosError> {
  let mut installed = true;
  let path = PathBuf::from(path);
  fs::create_dir_all(&path)?;
  if path.read_dir()?.next().is_none() {
    installed = false
  }
  Ok(installed)
}

#[tauri::command]
async fn check_tinkaros_update(app: tauri::AppHandle) -> Result<bool, TinkarosError> {
  let res_str = reqwest::Client::new()
    .get("https://api.github.com/repos/Hbarniq/tinkaros/releases")
    .header("User-Agent", "Tinkaros")
    .send()
    .await?
    .text()
    .await?;
  let res_json: Vec<GithubRelease> = serde_json::from_str(&res_str)?;
  Ok(tauri::api::version::is_greater(app.package_info().version.to_string().as_str(), res_json.first().unwrap().tag_name.as_str()).unwrap())
}

#[tauri::command]
async fn get_modpack_changelog() -> Result<String, TinkarosError> {
  let changelog = &State::get().await?.modpack.changelog_url;
  Ok(changelog.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          init,
          get_config,
          write_config,
          get_launchers,
          update,
          log_update,
          get_version,
          get_modpack_changelog,
          list_mod_projects, 
          explorer,
          check_modpack_installed,
          check_tinkaros_update,
          check_online
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
