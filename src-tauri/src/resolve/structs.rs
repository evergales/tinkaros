use std::{path::PathBuf, env::{consts, var}};

use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppConfig {
    pub init: bool,
    pub launcher: String,
    pub path: String,
    pub check_tinkaros_update: bool,
    pub max_concurrent_downloads: i32,
    pub bleeding_edge_updates: bool
}

impl AppConfig {
    pub fn new(init: bool, launcher: String, path: String, check_tinkaros_update: bool, max_concurrent_downloads: i32, bleeding_edge_updates: bool) -> Self { Self { init, launcher, path, check_tinkaros_update, max_concurrent_downloads, bleeding_edge_updates } }
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Modpack {
    pub name: String,
    pub version: String,
    pub mod_loader: String,
    pub mod_loader_version: String,
    pub game_version: String,
    pub overrides_url: String,
    pub changelog_url: String,
    pub mods: Vec<Mod>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Mod {
    pub name: String,
    pub identifier: ModIdentifier,
    pub version: ModVersion
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ModVersion {
    ModrinthVersionHash(String),
    CurseForgeVersionId(i32)
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ModIdentifier {
    ModrinthProject(String),
    CurseForgeProject(i32),
}

#[derive(Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
}

#[derive(Serialize)]
pub struct Launcher {
  name: String,
  path: String
}

impl Launcher {
    pub fn new(name: String, path: String) -> Self { Self { name, path } }
}

pub struct LauncherPath;
impl LauncherPath { // bunch of path declarations
    pub async fn mclauncher() -> PathBuf {
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
    pub async fn dotminecraft() -> PathBuf {
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
    pub async fn curseforge() -> PathBuf { 
        match consts::OS {
            "windows" => PathBuf::from(var("programfiles(x86)").unwrap()).join(r"Overwolf\OverwolfLauncher.exe"),
            _ => PathBuf::from("")
        }
    }
    pub async fn curseforge_instance() -> PathBuf { 
        match consts::OS {
            "windows" => PathBuf::from(var("USERPROFILE").unwrap()).join(format!("curseforge/minecraft/Instances/{}", State::get().await.unwrap().modpack.name)),
            _ => PathBuf::from("")
        }
    }
    pub async fn prism() -> PathBuf { 
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
    pub async fn prism_instance() -> PathBuf {
        match consts::OS {
            "windows" => PathBuf::from(var("APPDATA").unwrap()).join(format!("PrismLauncher/instances/{}/.minecraft", State::get().await.unwrap().modpack.name)),
            "linux" => {
                let def = PathBuf::from(var("HOME").unwrap()).join(".local/share/PrismLauncher/instances");
                let flatpak = PathBuf::from(var("HOME").unwrap()).join(".var/app/org.prismlauncher.PrismLauncher/data/PrismLauncher/instances");
                if def.exists() {
                    def.join(format!("{}/.minecraft", State::get().await.unwrap().modpack.name))
                } else {
                    flatpak.join(format!("{}/.minecraft", State::get().await.unwrap().modpack.name))
                }
            }
            _ => panic!("incompatible os")
        }
    }
}