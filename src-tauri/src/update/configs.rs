use std::{path::PathBuf, fs};

use chrono::Utc;
use fs_extra::dir::CopyOptions;
use reqwest::Client;
use serde_json::Map;

use crate::{resolve::structs::LauncherPath, get_version, error::TinkarosError, state::State};

use super::{status::{update_progress, update_status}, structs::{LauncherProfiles, Profile}, mods::download_file, zip_extract};

pub async fn resolve_configs(app: &tauri::AppHandle, path: &PathBuf, launcher: String) -> Result<(), TinkarosError> {
    let client = Client::new();
    let ver = get_version(path.to_string_lossy().to_string()).await?;

    if ver.version != ver.latest_version {
        let overrides_url = State::get().await?.modpack.overrides_url.clone();
        download_file(&client, &path.join("conf.zip"), &overrides_url).await?;
        zip_extract(&path.join("conf.zip"), path).unwrap();
    } else { return Ok(()); }

    if launcher == "default" {
        if LauncherPath::dotminecraft().await.exists() {
            let options = CopyOptions { overwrite: false, skip_exist: true, buffer_size: 64000, copy_inside: false, content_only: false, depth: 0 };
            update_status("installing required versions", app)?;
            fs_extra::move_items(&[path.join("versions").to_string_lossy().to_string()], LauncherPath::dotminecraft().await.to_string_lossy().to_string(), &options).ok();
            update_progress(90, app)?;
        }
        if LauncherPath::dotminecraft().await.join("launcher_profiles.json").exists() {
            let last_version_id = State::get().await.map_err(|err| TinkarosError::DataInvalid(err.to_string()))?.modpack.mod_loader_version.clone();
            let launcher_profiles = fs::read_to_string(LauncherPath::dotminecraft().await.join("launcher_profiles.json"))?;
            let modpack_name = State::get().await?.modpack.name.clone();

            if !launcher_profiles.is_empty()  {
                let mut launcher_json: LauncherProfiles = serde_json::from_str(&launcher_profiles).map_err(|_| TinkarosError::InvalidLauncherConfig)?;
                if !launcher_json.profiles.contains_key(&modpack_name) {
                    let mut other = Map::new();
                    other.insert("gameDir".to_string(), serde_json::Value::String(path.to_string_lossy().to_string()));

                    update_status(&format!("installing {} in mc launcher", modpack_name), app)?;
                    launcher_json.profiles.insert(
                        modpack_name.to_string(),
                        Profile {
                            name: modpack_name.to_owned(),
                            profile_type: "custom".into(),
                            created: Utc::now(),
                            last_version_id,
                            icon: "Furnace".into(),
                            other
                        },
                    );
                    let writer = fs::OpenOptions::new().read(true).write(true).truncate(true).open(LauncherPath::dotminecraft().await.join("launcher_profiles.json"))?;
                    serde_json::to_writer_pretty(writer, &launcher_json).map_err(|err| TinkarosError::Unknown(Box::new(err)))?;
                } else {
                    let latest_version = State::get().await.unwrap().modpack.mod_loader_version.clone();
                    if launcher_json.profiles.get(&modpack_name).unwrap().last_version_id != latest_version {
                        launcher_json.profiles.get_mut(&modpack_name).unwrap().last_version_id = latest_version.to_owned();
                        if let Some(ver) = launcher_json.profiles.get_mut(&modpack_name) {
                            ver.last_version_id = latest_version
                        }
                        let writer = fs::OpenOptions::new().read(true).write(true).truncate(true).open(LauncherPath::dotminecraft().await.join("launcher_profiles.json"))?;
                        serde_json::to_writer_pretty(writer, &launcher_json).map_err(|err| TinkarosError::Unknown(Box::new(err)))?;
                    }
                }
            }

            update_progress(95, app)?;
        }
    } else if launcher == "curseforge" {
        let url = &State::get().await?.modpack.launcher_configs.curseforge_url;
        let text = reqwest::get(url).await?.text().await?;
        fs::write(path.join("minecraftinstance.json"), text)?;
        
    } else if launcher == "prism" {
        let prism_main = &mut path.clone();
        prism_main.pop();
        let url = &State::get().await?.modpack.launcher_configs.prism_url;
        let text = reqwest::get(url).await?.text().await?;
        fs::write(prism_main.join("mmc-pack.json"), text)?;


        fs::write(
        prism_main.join("instance.cfg"),
        format!("InstanceType=OneSix
        JoinServerOnLaunch=false
        OverrideCommands=false
        OverrideConsole=false
        OverrideGameTime=false
        OverrideJavaArgs=false
        OverrideJavaLocation=false
        OverrideMemory=false
        OverrideMiscellaneous=false
        OverrideNativeWorkarounds=false
        OverridePerformance=false
        OverrideWindow=false
        iconKey=default
        name={}
        notes=
        ", State::get().await?.modpack.name))?;
    }

    update_status("cleaning up", app)?;
    fs::remove_dir_all(path.join("versions")).ok();
    fs::remove_file(path.join("conf.zip")).ok();

    Ok(())
}