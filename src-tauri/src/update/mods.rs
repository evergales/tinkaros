use std::{path::Path, collections::HashMap, sync::{Arc, Mutex}};

use futures_util::{StreamExt, future::{join_all, join}};
use reqwest::Client;
use tokio::{sync::Semaphore, fs::File, io::AsyncWriteExt};

use crate::{resolve::structs::{ResolveData, ModVersion}, error::TinkarosError};

use super::{new_modrinth, new_curseforge, status::{update_progress, update_status}, structs::CombinedProjects};

pub async fn update_mods(path: &Path, app: &tauri::AppHandle) -> Result<(), TinkarosError> {
    let path = path.join("mods");
    let modrinth = new_modrinth(app).unwrap();
    let curseforge = new_curseforge();
    let data = ResolveData::get().await.map_err(|err| TinkarosError::DataInvalid(err.to_string()))?;

    let mut modrinth_hashes = Vec::new();
    let mut curseforge_ids = Vec::new();

    for mod_ in data.modpack.mods {
        match mod_.version {
            ModVersion::ModrinthVersionHash(hash) => modrinth_hashes.push(hash),
            ModVersion::CurseForgeVersionId(id) => curseforge_ids.push(id),
        }
    }

    let mut to_download = HashMap::new();
    let mut to_install = HashMap::new();

    for file in curseforge.get_files(curseforge_ids).await.expect("unable to fetch mod versions") {
        if !file_exists(&file.file_name, &path).await {
            to_install.insert(file.file_name.clone(), file.download_url.clone().unwrap().to_string());
        }
        to_download.insert(file.file_name, file.download_url.unwrap().to_string());
    };

    for version in modrinth.get_versions_from_hashes(modrinth_hashes).await.expect("unable to fetch mod versions") {
        for file in version.1.files {
            if file.primary {
                if !file_exists(&file.filename, &path).await {
                    to_install.insert(file.filename.clone(), file.url.clone().to_string());
                }
                to_download.insert(file.filename, file.url.to_string());
            }
        }
    };

    let progress_per_mod = if !to_install.is_empty() {80.0 / to_install.len() as f32} else {0.0};
    let progress = Arc::new(Mutex::new(5.0));

    let semaphore = Arc::new(Semaphore::new(50)); // 50 max concurrent downloads
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
            && !to_download.contains_key(file_path.file_name().unwrap().to_str().unwrap())
    })
    .for_each(|file| {
        std::fs::remove_file(file.path()).expect("unable to remove file")
    });
    
    Ok(())
}

async fn file_exists(filename: &str, path: &Path) -> bool {
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

    let modrinth_future = modrinth.get_multiple_projects(modrinth_ids_slice);
    let curseforge_future = curseforge.get_mods(curseforge_ids);

    let (modrinth_projects, curseforge_mods) = join(modrinth_future, curseforge_future).await;

    let combined: Vec<CombinedProjects> = modrinth_projects?
        .into_iter()
        .map(CombinedProjects::ModrinthProject)
        .chain(curseforge_mods?.into_iter().map(CombinedProjects::CurseForgeMod))
        .collect();
    
    Ok(combined)
}