use std::{path::PathBuf, fs::{self, canonicalize}, env::{self, var}};

use fs_extra::dir::CopyOptions;
use futures_util::StreamExt;
use tauri::Manager;
use {
    serde::Serialize,
    reqwest::Client,
    std::{
        cmp::min,
        fs::File,
        io::Write
    },
    zip_extensions::*,
};

#[derive(Clone, Serialize)]
pub struct StatusUpdate {
  status: String,
}

#[derive(Clone, Serialize)]
pub struct ProgressUpdate {
  progress: i32
}

pub struct LauncherPath;
impl LauncherPath { // bunch of path declarations
    pub fn mclauncher() -> PathBuf { PathBuf::from(var("programfiles(x86)").unwrap()).join(r"Minecraft Launcher\MinecraftLauncher.exe") }
    pub fn dotminecraft() -> PathBuf { PathBuf::from(var("APPDATA").unwrap()).join(".minecraft") }
    pub fn curseforge() -> PathBuf { PathBuf::from(var("programfiles(x86)").unwrap()).join(r"Overwolf\OverwolfLauncher.exe") }
    pub fn curseforge_instance() -> PathBuf { PathBuf::from(var("USERPROFILE").unwrap()).join(r"curseforge\minecraft\Instances\ahms") }
    pub fn prism() -> PathBuf { PathBuf::from(var("LOCALAPPDATA").unwrap()).join(r"Programs\PrismLauncher\prismlauncher.exe") }
    pub fn prism_instance() -> PathBuf { PathBuf::from(var("APPDATA").unwrap()).join(r"PrismLauncher\instances\ahms\.minecraft") }
}


pub fn resolve(path: &PathBuf) {
    if !path.exists() {
        fs::create_dir_all(path).unwrap();
    }
}

pub fn mclauncher() -> PathBuf {
    PathBuf::from(var("programfiles(x86)").unwrap()).join(r"Minecraft Launcher\MinecraftLauncher.exe")
}

pub async fn resolve_configs(app: &tauri::AppHandle, path: &PathBuf, launcher: String, custom: bool) {
    if launcher == "default" {
        if LauncherPath::dotminecraft().exists() {
            let options = CopyOptions { overwrite: false, skip_exist: true, buffer_size: 64000, copy_inside: false, content_only: false, depth: 0 };
            update_status("installing required versions", app);
            fs_extra::move_items(&[path.join("versions").to_string_lossy().to_string()], LauncherPath::dotminecraft().to_string_lossy().to_string(), &options).expect("unable to move files");
            update_progress(90, app);
        }
        if !custom && LauncherPath::dotminecraft().exists() {
            update_status("creating new installation in minecraft launcher", app);
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
    let _path = path.clone();
    let downloadurl = "https://drive.google.com/uc?export=download&id=1qa7gThngkqNooUweuyVs6Kes8w_pIJ0l&confirm=t";
    let update: bool = path.join("mods").exists();
    let emptydir: bool = !update && fs::read_dir(path).unwrap().count() == 0;
    let raw_mczip = if !update && !emptydir { _path.join("modpack/mcmods.zip") } else {path.join("./mcmods.zip")};
    let mcmodszip = raw_mczip.to_str().unwrap();
    let raw_modsdir = if !update && !emptydir { path.join("modpack")} else { path.to_path_buf() };
    let mods_dir = raw_modsdir.to_str().unwrap();
    if !update {
        if !emptydir {fs::create_dir(mods_dir).ok();}
    } else {
        update_status("removing old data", app);
        fs::remove_dir_all(format!(r"{}\mods", mods_dir)).expect("unable to delete old mods");    
        fs::remove_dir_all(format!(r"{}\versions", mods_dir)).ok();

    };
    
    let cwd = env::current_dir().unwrap();
    let mut a_path = cwd.into_os_string().into_string().unwrap();
    if mcmodszip != "./mcmods.zip" { a_path.push_str(r"\modpack"); }
    update_status("downloading files", app);
    download_file(&Client::new(), downloadurl, mcmodszip, app, true).await.unwrap();

    update_status("extracting files", app);
    let mczip = PathBuf::from(&mcmodszip);
    let extract_dir = PathBuf::from(&mods_dir);
    zip_extract(&mczip, &extract_dir).expect("Could not extract zip file");
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
