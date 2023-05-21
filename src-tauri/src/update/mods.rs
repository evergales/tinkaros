use std::{path::Path, collections::HashMap, sync::{Arc, Mutex}};

use futures_util::{StreamExt, future::join_all};
use reqwest::Client;
use tokio::{sync::Semaphore, fs::File, io::AsyncWriteExt};

use crate::resolve::structs::{ResolveData, ModVersion};

use super::{new_modrinth, new_curseforge, status::{update_progress, update_status}};

pub async fn update_mods(path: &Path, app: &tauri::AppHandle) {
    let path = path.join("mods");
    let modrinth = new_modrinth(app).expect("unable to create modrinth client");
    let curseforge = new_curseforge();
    let data = ResolveData::get().await.expect("unable to resolve configs");

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
    let client = Arc::new(Client::new());
    let app = Arc::new(app.clone());


    let tasks = to_install.into_iter().map(|(filename, url)| {
        let semaphore = Arc::clone(&semaphore);
        let client = Arc::clone(&client);
        let app = Arc::clone(&app);
        let path = path.to_owned();
        let progress = Arc::clone(&progress);

        tokio::spawn(async move {
            let permit = semaphore.acquire().await.unwrap();

            update_status(&format!("updating {}", filename.replace(".jar", "")), &app);
            download_file(&client, &path.join(filename), &url).await.expect("could not download file");

            *progress.lock().unwrap() += progress_per_mod;
            update_progress(*progress.lock().unwrap() as i32, &app);
            
            drop(permit);
        })
    });

    let results = join_all(tasks).await;
    for result in results {
        if let Err(err) = result {
            eprintln!("An error occurred: {}", err);
        }
    }

    update_progress(85, &app);

    // clean up old mods
    path.read_dir().unwrap()
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