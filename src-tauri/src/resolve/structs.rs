use std::{path::PathBuf, env::{consts, var}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppConfig {
    pub init: bool,
    pub launcher: String,
    pub path: String,
    pub custom: bool
}

impl AppConfig {
    pub fn new(init: bool, launcher: String, path: String, custom: bool) -> Self { Self { init, launcher, path, custom } }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResolveData {
    pub modpack: Modpack,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Modpack {
    pub version: String,
    pub mod_loader: String,
    pub mod_loader_version: String,
    pub game_version: String,
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

impl ResolveData {
    pub async fn get() -> Result<Self, reqwest::Error> {
        let url = "https://gist.githubusercontent.com/Hbarniq/ec9d01d863083becd062b378ca01a3d8/raw/ahms.json";
        let response = reqwest::get(url).await?.json::<ResolveData>().await?;
        Ok(response)
    }
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