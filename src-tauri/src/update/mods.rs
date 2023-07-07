use std::{path::Path, sync::{Arc, Mutex}};

use chrono::{DateTime, Utc};
use ferinth::structures::version::LatestVersionBody;
use futures_util::{StreamExt, future::{join_all, join}};
use reqwest::Client;
use tokio::{sync::Semaphore, fs::File, io::AsyncWriteExt};

use crate::{resolve::{structs::{ResolveData, ModVersion, ModIdentifier}, config::get_config}, error::TinkarosError};

use super::{new_modrinth, new_curseforge, status::{update_progress, update_status}, structs::CombinedProjects};

pub async fn update_mods(path: &Path, app: &tauri::AppHandle) -> Result<(), TinkarosError> {
    let path = path.join("mods");
    let data = ResolveData::get().await.map_err(|err| TinkarosError::DataInvalid(err.to_string()))?;
    let config = get_config()?;

    let (to_download, to_install) = match config.bleeding_edge_updates {
      true => get_bleeding_updates(&data, &path, app).await?,
      false => get_normal_updates(&data, &path, app).await? 
    };

    let progress_per_mod = if !to_install.is_empty() {80.0 / to_install.len() as f32} else {0.0};
    let progress = Arc::new(Mutex::new(5.0));

    let max_concurrent_downloads: usize = match config.max_concurrent_downloads {
        10..=85 => config.max_concurrent_downloads.try_into().unwrap(),
        _ => 75
    };

    let semaphore = Arc::new(Semaphore::new(max_concurrent_downloads));
    let client = Client::new();

    update_status("updating mods", app)?;
    let tasks = to_install.into_iter().map(|(filename, url)| {
        let semaphore = Arc::clone(&semaphore);
        let progress = Arc::clone(&progress);
        let client = client.clone();
        let app = app.clone();
        let path = path.clone();

        tokio::spawn(async move {
            let permit = semaphore.acquire().await.unwrap();

            download_file(&client, &path.join(&filename), &url).await.unwrap_or_else(|_| panic!("unable to download {filename} from {url}"));

            *progress.lock().unwrap() += progress_per_mod;
            update_progress(*progress.lock().unwrap() as i32, &app).unwrap();
            
            drop(permit);
        })
    });

    let results = join_all(tasks).await;
    for result in results {
        if let Err(err) = result {
            return Err(TinkarosError::Update(err.to_string()));
        }
    }

    update_progress(85, app)?;

    // clean up old mods
    path.read_dir()?
    .filter_map(|entry| entry.ok())
    .filter(|entry| {
        let file_path = entry.path();
        file_path.is_file()
            && file_path.extension().map(|ext| ext == "jar").unwrap_or(false)
            && !to_download.iter().any(|f| f.0 == file_path.file_name().unwrap().to_str().unwrap())
    })
    .for_each(|file| {
        std::fs::remove_file(file.path()).expect("unable to remove file")
    });
    
    Ok(())
}

fn file_exists(filename: &str, path: &Path) -> bool {
    let file_path = path.join(filename);
    file_path.exists()
}


pub async fn download_file(client: &Client, path: &Path, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;
    let mut body = response.bytes_stream();

    let mut file = File::create(path.join(path)).await?;

    while let Some(chunk) = body.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}

pub async fn get_projects_from_ids(modrinth_ids: Vec<String>, curseforge_ids: Vec<i32>, app: &tauri::AppHandle) -> Result<Vec<CombinedProjects>, Box<dyn std::error::Error>> {
    let modrinth = new_modrinth(app)?;
    let curseforge = new_curseforge();

    let modrinth_ids_slice = &modrinth_ids.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

    let modrinth_future = async {
        if !modrinth_ids.is_empty() {
            modrinth.get_multiple_projects(modrinth_ids_slice).await
        } else {
            Ok(Vec::new())
        }
    };

    let curseforge_future = async {
        if !curseforge_ids.is_empty() {
            curseforge.get_mods(curseforge_ids).await
        } else {
            Ok(Vec::new())
        }
    };

    let (modrinth_projects, curseforge_mods) = join(modrinth_future, curseforge_future).await;

    let combined: Vec<CombinedProjects> = modrinth_projects?
        .into_iter()
        .map(CombinedProjects::ModrinthProject)
        .chain(curseforge_mods?.into_iter().map(CombinedProjects::CurseForgeMod))
        .collect();
    
    Ok(combined)
}

async fn get_bleeding_updates(data: &ResolveData, mods_path: &Path, app: &tauri::AppHandle) -> Result<(Vec<(String, String)>, Vec<(String, String)>), TinkarosError> {
    let modrinth = new_modrinth(app).unwrap();
    let curseforge = new_curseforge();

    update_status("finding latest mod versions", app)?;
    
    let mut modrinth_version_hashes = Vec::new();
    let mut curseforge_ids = Vec::new();

    for mod_ in &data.modpack.mods {
        match &mod_.identifier {
            ModIdentifier::ModrinthProject(_) => match &mod_.version {
                ModVersion::ModrinthVersionHash(hash) => modrinth_version_hashes.push(hash.to_owned()),
                ModVersion::CurseForgeVersionId(_) => return Err(TinkarosError::Update("found invalid mod list".to_string())),
            },
            ModIdentifier::CurseForgeProject(id) => curseforge_ids.push(id.to_owned())
        }
    }

    let latest_modrinth_versions: Vec<(String, String)> = modrinth.latest_versions_from_hashes(
        modrinth_version_hashes,
        LatestVersionBody {
            loaders: vec![data.modpack.mod_loader.clone()],
            game_versions: vec![data.modpack.game_version.clone()],
        },
    )
    .await
    .map_err(|_| TinkarosError::Update("unable to fetch latest versions".to_string()))?
    .into_values()
    .map(|version| {
        let latest_version = match version.files.len() {
            1 => version.files[0].to_owned(),
            _ => version.files.into_iter().find(|file| file.primary).unwrap()
        };

        (latest_version.filename, latest_version.url.to_string())
    })
    .collect();

    let latest_curseforge_versions = Arc::new(Mutex::new(Vec::new()));

    let mut chars = data.modpack.mod_loader.chars();
    let mod_loader_capitalized = match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    };

    let mut tasks = Vec::new();

    for mod_id in curseforge_ids {
        let latest_curseforge_versions = Arc::clone(&latest_curseforge_versions);
        let curseforge = curseforge.clone();
        let mod_loader_capitalized = mod_loader_capitalized.clone();

        let task = async move {
            let mut compatible_files: Vec<(DateTime<Utc>, (String, String))> = Vec::new();

            curseforge.get_mod_files(mod_id).await.map_err(|_| TinkarosError::Update("unable to fetch mod versions".to_string())).unwrap()
            .into_iter()
            .for_each(|file| {
                if file.is_available && file.game_versions.contains(&mod_loader_capitalized) && file.game_versions.contains(&data.modpack.game_version) {
                    compatible_files.push((file.file_date, (file.file_name, file.download_url.unwrap().to_string())));
                }
            });
    
            let latest_compatible_file = compatible_files.into_iter().max_by_key(|(datetime, _)| *datetime).unwrap().1;
    
            latest_curseforge_versions.lock().unwrap().push((latest_compatible_file.0, latest_compatible_file.1));
        };

        tasks.push(task);
    };

    join_all(tasks).await;

    let mut to_download: Vec<(String, String)> = Vec::new();
    let mut to_install: Vec<(String, String)> = Vec::new();

    let latest_curseforge_versions = latest_curseforge_versions.lock().unwrap().clone();
    latest_modrinth_versions.into_iter().chain(latest_curseforge_versions).for_each(|(filename, url)| {
        if !file_exists(filename.as_str(), mods_path) {
            to_install.push((filename.clone(), url.clone()));
        }
        to_download.push((filename, url));
    });

    Ok((to_download, to_install))
}

async fn get_normal_updates(data: &ResolveData, mods_path: &Path, app: &tauri::AppHandle) -> Result<(Vec<(String, String)>, Vec<(String, String)>), TinkarosError> {
    let modrinth = new_modrinth(app).unwrap();
    let curseforge = new_curseforge();

    let mut modrinth_version_hashes = Vec::new();
    let mut curseforge_version_ids = Vec::new();

    for mod_ in &data.modpack.mods {
        match &mod_.version {
            ModVersion::ModrinthVersionHash(hash) => modrinth_version_hashes.push(hash.to_owned()),
            ModVersion::CurseForgeVersionId(id) => curseforge_version_ids.push(id.to_owned()),
        }
    }

    let mut to_download: Vec<(String, String)>  = Vec::new();
    let mut to_install: Vec<(String, String)>  = Vec::new();

    for file in curseforge.get_files(curseforge_version_ids).await.expect("unable to fetch mod versions") {
        if !file_exists(&file.file_name, mods_path) {
            to_install.push((file.file_name.clone(), file.download_url.clone().unwrap().to_string()));
        }
        to_download.push((file.file_name, file.download_url.unwrap().to_string()));
    };

    for version in modrinth.get_versions_from_hashes(modrinth_version_hashes).await.expect("unable to fetch mod versions") {
        for file in version.1.files {
            if file.primary {
                if !file_exists(&file.filename, mods_path) {
                    to_install.push((file.filename.clone(), file.url.clone().to_string()));
                }
                to_download.push((file.filename, file.url.to_string()));
            }
        }
    };

    Ok((to_download, to_install))
}