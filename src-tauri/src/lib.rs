use std::{path::PathBuf, fs::{self, canonicalize}, env::{consts, var}, collections::HashMap, cmp::min, fs::File, io::Write};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use fs_extra::dir::CopyOptions;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};
use chrono::{DateTime, Utc};
use tauri::Manager;
use reqwest::Client;
use zip_extensions::*;


#[derive(Clone, Serialize)]
pub struct StatusUpdate {
  status: String,
}

#[derive(Clone, Serialize)]
pub struct ProgressUpdate {
  progress: i32
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherProfiles {
    profiles: HashMap<String, Profile>,
    #[serde(flatten)]
    other: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Profile {
    name: String,
    #[serde(rename = "type")]
    profile_type: String,
    created: DateTime<Utc>,
    last_version_id: String,
    icon: String,
    #[serde(flatten)]
    other: Map<String, Value>,
}

pub struct LauncherPath;
impl LauncherPath { // bunch of path declarations
    pub fn mclauncher() -> PathBuf {
        match consts::OS {
            "windows" => {
                let default = PathBuf::from(var("programfiles(x86)").unwrap()).join(r"Minecraft Launcher\MinecraftLauncher.exe");
                let msstore = PathBuf::from(var("ProgramFiles").unwrap()).join(r"WindowsApps\Microsoft.4297127D64EC6_1.1.28.0_x64__8wekyb3d8bbwe\Minecraft.exe"); //microsoft sucks
                if default.exists() { default } else { msstore }
            },
            "linux" => PathBuf::from(var("HOME").unwrap()).join(".minecraft"),
            _ => panic!("incompatible os")
        }
    }
    pub fn dotminecraft() -> PathBuf {
        match consts::OS {
            "windows" => {
                let case1 = PathBuf::from(var("APPDATA").unwrap()).join(".minecraft");
                let case2 = PathBuf::from(var("APPDATA").unwrap()).join(r"Roaming\.minecraft");
                if case1.exists() { case1 } else { case2 }
            },
            "linux" => PathBuf::from(var("HOME").unwrap()).join(".minecraft"),
            _ => panic!("incompatible os")
        }
    }
    pub fn curseforge() -> PathBuf { 
        match consts::OS {
            "windows" => PathBuf::from(var("programfiles(x86)").unwrap()).join(r"Overwolf\OverwolfLauncher.exe"),
            _ => PathBuf::from("")
        }
    }
    pub fn curseforge_instance() -> PathBuf { 
        match consts::OS {
            "windows" => PathBuf::from(var("USERPROFILE").unwrap()).join(r"curseforge\minecraft\Instances\ahms"),
            _ => PathBuf::from("")
        }
    }
    pub fn prism() -> PathBuf { 
        match consts::OS {
            "windows" => PathBuf::from(var("LOCALAPPDATA").unwrap()).join(r"Programs\PrismLauncher\prismlauncher.exe"),
            "linux" => {
                let def = PathBuf::from("/usr/share/applications/org.prismlauncher.PrismLauncher.desktop");
                let flatpak = PathBuf::from("/var/lib/flatpak/exports/share/applications/org.prismlauncher.PrismLauncher.desktop");
                if def.exists() { def } else { flatpak }
            }
            _ => panic!("incompatible os")
        }
    }
    pub fn prism_instance() -> PathBuf {
        match consts::OS {
            "windows" => PathBuf::from(var("APPDATA").unwrap()).join(r"PrismLauncher\instances\ahms\.minecraft"),
            "linux" => {
                let def = PathBuf::from(var("HOME").unwrap()).join(".local/share/PrismLauncher/instances");
                let flatpak = PathBuf::from(var("HOME").unwrap()).join(".var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances");
                if def.exists() { def } else { flatpak }
            }
            _ => panic!("incompatible os")
        }
    }
}


pub fn resolve(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }
}   

pub async fn resolve_configs(app: &tauri::AppHandle, path: &PathBuf, launcher: String, _custom: bool) {
    if launcher == "default" {
        if LauncherPath::dotminecraft().exists() {
            let options = CopyOptions { overwrite: false, skip_exist: true, buffer_size: 64000, copy_inside: false, content_only: false, depth: 0 };
            update_status("installing required versions", app);
            fs_extra::move_items(&[path.join("versions").to_string_lossy().to_string()], LauncherPath::dotminecraft().to_string_lossy().to_string(), &options).ok();
            update_progress(90, app);
        }
        if LauncherPath::dotminecraft().join("launcher_profiles.json").exists() {
            let launcher_profiles = fs::read_to_string(LauncherPath::dotminecraft().join("launcher_profiles.json")).expect("unable to read profiles");
            if !launcher_profiles.is_empty()  {
                let mut launcher_json: LauncherProfiles = serde_json::from_str(&launcher_profiles).expect("unable to convert to json");
                if !launcher_json.profiles.contains_key("ahms") {
                    let mut other = Map::new();
                    other.insert("gameDir".to_string(), serde_json::Value::String(path.to_string_lossy().to_string()));

                    update_status("installing ahms in mc launcher", app);
                    launcher_json.profiles.insert(
                        "ahms".to_string(),
                        Profile {
                            name: "AHMS".to_owned(),
                            profile_type: "custom".into(),
                            created: Utc::now(),
                            last_version_id: reqwest::get("https://raw.githubusercontent.com/Hbarniq/ahms/main/launcher_version").await.unwrap().text().await.unwrap(),
                            icon: format!("data:image/png;base64,{}", BASE64.encode(reqwest::get("https://raw.githubusercontent.com/Hbarniq/ahms/main/assets/icon.png").await.unwrap().bytes().await.unwrap())),
                            other
                        },
                    );
                    let writer = fs::OpenOptions::new().read(true).write(true).truncate(true).open(LauncherPath::dotminecraft().join("launcher_profiles.json")).expect("unable to open file");
                    serde_json::to_writer_pretty(writer, &launcher_json).expect("unable to write to profiles");
                } else {
                    let latest_version = reqwest::get("https://raw.githubusercontent.com/Hbarniq/ahms/main/launcher_version").await.unwrap().text().await.unwrap();
                    if launcher_json.profiles.get("ahms").unwrap().last_version_id != latest_version {
                        launcher_json.profiles.get_mut("ahms").unwrap().last_version_id = latest_version.to_owned();
                        if let Some(ver) = launcher_json.profiles.get_mut("ahms") {
                            ver.last_version_id = latest_version
                        }
                        println!("{:?}", serde_json::to_string_pretty(&launcher_json).unwrap());
                        let writer = fs::OpenOptions::new().read(true).write(true).truncate(true).open(LauncherPath::dotminecraft().join("launcher_profiles.json")).expect("unable to open file");
                        serde_json::to_writer_pretty(writer, &launcher_json).expect("unable to write to profiles");
                    }
                }
            }

            update_progress(95, app);
        }
    } else if launcher == "curseforge" {
        if !path.join("minecraftinstance.json").exists() {
            resolve_lconfigs(path, launcher, app).await;
        }
    } else if launcher == "prism" {
        let prism_main = &mut path.clone();
        prism_main.pop();
        resolve_lconfigs(prism_main, launcher, app).await;
    }

    update_status("cleaning up", app);
    fs::remove_dir_all(path.join("versions")).ok();
    fs::remove_file(path.join("mcmods.zip")).ok();
    if !path.join("updater_log.txt").exists() {
        let exe_path = canonicalize(PathBuf::from("./")).unwrap().to_string_lossy().to_string();
        fs::write(path.join("updater_loc.txt"), exe_path.replace(r"\\?\", "") + r"\mci-reloaded.exe").unwrap();
    }
}

pub fn update_status(msg: &str, app: &tauri::AppHandle) {
    app.emit_all("status", StatusUpdate {status: msg.to_string()}).unwrap();
}

pub fn update_progress(progress: i32, app: &tauri::AppHandle) {
    app.emit_all("progressUpdate", ProgressUpdate { progress }).unwrap();
}

pub async fn update_files(path: &PathBuf, app: &tauri::AppHandle) {
    let downloadurl = "https://drive.google.com/uc?export=download&id=1qa7gThngkqNooUweuyVs6Kes8w_pIJ0l&confirm=t";
    let update: bool = path.join("mods").exists();
    let mcmodszip = path.join("mcmods.zip");

    if update {
        update_status("removing old data", app);
        fs::remove_dir_all(path.join("mods")).unwrap_or_else(|e| {
            eprintln!("unable to delete old mods: {}", e);
        });
        fs::remove_dir_all(path.join("versions")).ok();
    }

    update_status("downloading files", app);
    download_file(&Client::new(), downloadurl, &mcmodszip.to_str().unwrap(), app, true).await.unwrap_or_else(|e| eprintln!("unable to download files: {}", e));

    update_status("extracting files", app);
    zip_extract(&mcmodszip, &path).unwrap_or_else(|e| {
        eprintln!("unable to extract files: {}", e);
    });
}

async fn resolve_lconfigs(path: &PathBuf, ltype: String, app: &tauri::AppHandle) {
    if ltype == "curseforge" {
        update_status("downloading curseforge configs", app);
        download_file(&Client::new(), "https://drive.google.com/uc?export=download&id=1BKqLWvB287lv6zgGhGp3UOyr6Dy0jeIe&confirm=t", path.join("curseforge.zip").to_str().unwrap(), app, false).await.unwrap();
        update_progress(90, app);

        update_status("extracting configs", app);
        zip_extract(&path.join("curseforge.zip"), path).expect("Could not extract zip file");
        update_progress(95, app);
        fs::remove_file(path.join("curseforge.zip")).expect("could not clean up");
    } else if ltype == "prism" {
        update_status("downloading prism configs", app);
        download_file(&Client::new(), "https://drive.google.com/uc?export=download&id=18r_C-tvMEjcbBUA8TqApOXMkqaR_XFrs&confirm=t", path.join("prism.zip").to_str().unwrap(), app, false).await.unwrap();
        update_progress(90, app);

        update_status("extracting configs", app);
        zip_extract(&path.join("prism.zip"), path).expect("Could not extract zip file");
        update_progress(95, app);
        fs::remove_file(path.join("prism.zip")).expect("could not clean up");
    }
}

async fn download_file(client: &Client, url: &str, path: &str, app: &tauri::AppHandle, main: bool) -> Result<(), String> {
    let res = client
        .get(url)
        .send()
        .await
        .or(Err(format!("Failed to GET from '{}'", &url)))?;
    let total_size = res.content_length().ok_or(format!("Failed to get content length from '{}'", &url))?;

    let mut file = File::create(path).or(Err(format!("Failed to create file '{}'", path)))?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut count: i16 = 0; // dont send too many requests..

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file".to_string()))?;
        file.write_all(&chunk)
            .or(Err("Error while writing to file".to_string()))?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        if main {
            if count >= 1000 || new == total_size { // dont emit events too often
                let percent = (80 * new / total_size) + 5; // 5-85% on main download
                update_progress(percent as i32, app);
                count = 0
            } else {count += 1}
        }
    }

    Ok(())
}
