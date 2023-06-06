use tauri::Manager;

use crate::error::TinkarosError;

use super::structs::{StatusUpdate, ProgressUpdate};

pub fn update_status(msg: &str, app: &tauri::AppHandle) -> Result<(), TinkarosError> {
    app.emit_all("status", StatusUpdate {status: msg.to_string()}).map_err(|_| TinkarosError::EmitEvent)?;
    Ok(())
}

pub fn update_progress(progress: i32, app: &tauri::AppHandle) -> Result<(), TinkarosError> {
    app.emit_all("progressUpdate", ProgressUpdate { progress }).map_err(|_| TinkarosError::EmitEvent)?;
    Ok(())
}