use std::{fs::File, path::PathBuf};

use ferinth::Ferinth;
use furse::Furse;
use zip::result::ZipResult;

pub mod structs;
pub mod mods;
pub mod status;
pub mod configs;

pub fn new_modrinth(app: &tauri::AppHandle) -> Result<Ferinth, ferinth::Error> {
    return Ferinth::new("tinkaros", Some(app.package_info().version.to_string().as_str()), Some("@hbarni/Hbarni#4164"), None);
}

pub fn new_curseforge() -> Furse {
    Furse::new("$2a$10$Grlqtes/CrLoTgnvg174H.BKRX8caplGh0o1dOwxhhMWAgv.2J9cC")
}

pub fn zip_extract(archive_file: &PathBuf, target_dir: &PathBuf) -> ZipResult<()> {
    let file = File::open(archive_file)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(target_dir)
}