mod organizer;
mod watcher;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::State;
use organizer::Organizer;

struct AppState {
    organizer: Arc<Mutex<Organizer>>,
    watching: Arc<Mutex<bool>>,
}

#[derive(serde::Serialize)]
struct MovedFile {
    original_name: String,
    new_name: String,
    category: String,
}

#[tauri::command]
fn organize_now(folder: String, state: State<AppState>) -> Result<Vec<MovedFile>, String> {
    let organizer = state.organizer.lock().map_err(|e| e.to_string())?;
    let dir = PathBuf::from(&folder);

    let mut moved = Vec::new();

    let entries = std::fs::read_dir(&dir).map_err(|e| e.to_string())?;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let result = organizer.classify(&path);
        let dest_dir = dir.join(&result.category);
        let original_name = path.file_name().unwrap().to_string_lossy().to_string();

        if organizer.move_file(&path, &dest_dir, Some(&result.suggested_filename)).is_ok() {
            moved.push(MovedFile {
                original_name,
                new_name: format!("{}{}", result.suggested_filename,
                    path.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default()),
                category: result.category,
            });
        }
    }

    Ok(moved)
}

#[tauri::command]
fn toggle_watching(state: State<AppState>) -> Result<bool, String> {
    let mut watching = state.watching.lock().map_err(|e| e.to_string())?;
    *watching = !*watching;
    Ok(*watching)
}

#[tauri::command]
fn is_watching(state: State<AppState>) -> Result<bool, String> {
    let watching = state.watching.lock().map_err(|e| e.to_string())?;
    Ok(*watching)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let organizer = Organizer::new(vec![
        "office".to_string(),
        "health".to_string(),
        "bank".to_string(),
        "academics".to_string(),
        "gov_ids".to_string(),
    ]).expect("failed to initialize organizer");

    let state = AppState {
        organizer: Arc::new(Mutex::new(organizer)),
        watching: Arc::new(Mutex::new(false)),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            organize_now,
            toggle_watching,
            is_watching,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}