use tauri::Manager;

use super::structs::{StatusUpdate, ProgressUpdate};

pub fn update_status(msg: &str, app: &tauri::AppHandle) {
    app.emit_all("status", StatusUpdate {status: msg.to_string()}).expect("unable to emit status update");
}

pub fn update_progress(progress: i32, app: &tauri::AppHandle) {
    app.emit_all("progressUpdate", ProgressUpdate { progress }).expect("unable to send status update");
}