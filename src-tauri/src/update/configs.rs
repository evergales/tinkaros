use std::{path::PathBuf, fs};

use chrono::Utc;
use fs_extra::dir::CopyOptions;
use reqwest::Client;
use serde_json::Map;
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};

use crate::{resolve::structs::LauncherPath, get_version, error::TinkarosError, state::State};

use super::{status::{update_progress, update_status}, structs::{LauncherProfiles, Profile}, mods::download_file, zip_extract};

pub async fn resolve_configs(app: &tauri::AppHandle, path: &PathBuf, launcher: String) -> Result<(), TinkarosError> {
    let client = Client::new();
    let ver = get_version(path.to_string_lossy().to_string()).await?;

    if ver.version != ver.latest_version {
        download_file(&client, &path.join("conf.zip"), "https://drive.google.com/uc?export=download&id=1qa7gThngkqNooUweuyVs6Kes8w_pIJ0l&confirm=t").await?;
        zip_extract(&path.join("conf.zip"), path).unwrap();
    }

    if launcher == "default" {
        if LauncherPath::dotminecraft().exists() {
            let options = CopyOptions { overwrite: false, skip_exist: true, buffer_size: 64000, copy_inside: false, content_only: false, depth: 0 };
            update_status("installing required versions", app)?;
            fs_extra::move_items(&[path.join("versions").to_string_lossy().to_string()], LauncherPath::dotminecraft().to_string_lossy().to_string(), &options).ok();
            update_progress(90, app)?;
        }
        if LauncherPath::dotminecraft().join("launcher_profiles.json").exists() {
            let last_version_id = State::get().await.map_err(|err| TinkarosError::DataInvalid(err.to_string()))?.modpack.mod_loader_version.clone();
            let launcher_profiles = fs::read_to_string(LauncherPath::dotminecraft().join("launcher_profiles.json"))?;

            if !launcher_profiles.is_empty()  {
                let mut launcher_json: LauncherProfiles = serde_json::from_str(&launcher_profiles).map_err(|_| TinkarosError::InvalidLauncherConfig)?;
                if !launcher_json.profiles.contains_key("ahms") {
                    let mut other = Map::new();
                    other.insert("gameDir".to_string(), serde_json::Value::String(path.to_string_lossy().to_string()));

                    update_status("installing ahms in mc launcher", app)?;
                    launcher_json.profiles.insert(
                        "ahms".to_string(),
                        Profile {
                            name: "AHMS".to_owned(),
                            profile_type: "custom".into(),
                            created: Utc::now(),
                            last_version_id,
                            icon: format!("data:image/png;base64,{}", BASE64.encode(reqwest::get("https://cdn.discordapp.com/attachments/1050691362128924743/1112084612060033104/wFc68vW.png").await?.bytes().await?)),
                            other
                        },
                    );
                    let writer = fs::OpenOptions::new().read(true).write(true).truncate(true).open(LauncherPath::dotminecraft().join("launcher_profiles.json"))?;
                    serde_json::to_writer_pretty(writer, &launcher_json).map_err(|err| TinkarosError::Unknown(Box::new(err)))?;
                } else {
                    let latest_version = State::get().await.unwrap().modpack.mod_loader_version.clone();
                    if launcher_json.profiles.get("ahms").unwrap().last_version_id != latest_version {
                        launcher_json.profiles.get_mut("ahms").unwrap().last_version_id = latest_version.to_owned();
                        if let Some(ver) = launcher_json.profiles.get_mut("ahms") {
                            ver.last_version_id = latest_version
                        }
                        let writer = fs::OpenOptions::new().read(true).write(true).truncate(true).open(LauncherPath::dotminecraft().join("launcher_profiles.json"))?;
                        serde_json::to_writer_pretty(writer, &launcher_json).map_err(|err| TinkarosError::Unknown(Box::new(err)))?;
                    }
                }
            }

            update_progress(95, app)?;
        }
    } else if launcher == "curseforge" {
        if !path.join("minecraftinstance.json").exists() {
            resolve_lconfigs(path, launcher, app).await?;
        }
    } else if launcher == "prism" {
        let prism_main = &mut path.clone();
        prism_main.pop();
        resolve_lconfigs(prism_main, launcher, app).await?;
    }

    update_status("cleaning up", app)?;
    fs::remove_dir_all(path.join("versions")).ok();
    fs::remove_file(path.join("conf.zip")).ok();

    Ok(())
}

async fn resolve_lconfigs(path: &PathBuf, ltype: String, app: &tauri::AppHandle) -> Result<(), TinkarosError> {
    let client = Client::new();
    if ltype == "curseforge" {
        update_status("downloading curseforge configs", app)?;
        download_file(&client, &path.join("curseforge.zip"),"https://drive.google.com/uc?export=download&id=1BKqLWvB287lv6zgGhGp3UOyr6Dy0jeIe&confirm=t").await?;
        update_progress(90, app)?;

        update_status("extracting configs", app)?;
        zip_extract(&path.join("curseforge.zip"), path)?;
        update_progress(95, app)?;
        fs::remove_file(path.join("curseforge.zip"))?;
    } else if ltype == "prism" {
        update_status("downloading prism configs", app)?;
        download_file(&client, &path.join("prism.zip"), "https://drive.google.com/uc?export=download&id=18r_C-tvMEjcbBUA8TqApOXMkqaR_XFrs&confirm=t").await?;
        update_progress(90, app)?;

        update_status("extracting configs", app)?;
        zip_extract(&path.join("prism.zip"), path)?;
        update_progress(95, app)?;
        fs::remove_file(path.join("prism.zip"))?;
    }

    Ok(())
}