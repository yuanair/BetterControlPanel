use std::sync::Mutex;
use tauri::Manager;

pub mod tray;
pub mod game;

#[tauri::command]
fn apply_window_vibrancy(app_handle: tauri::AppHandle, app_window: tauri::Window) {
    #[cfg(target_os = "macos")]
    window_vibrancy::apply_vibrancy(&app_window, NSVisualEffectMaterial::FullScreenUI).unwrap();

    #[cfg(target_os = "windows")]
    window_vibrancy::apply_mica(&app_window, None).unwrap();
}

#[tauri::command]
fn clear_window_vibrancy(app_handle: tauri::AppHandle, app_window: tauri::Window) {
    #[cfg(target_os = "macos")]
    window_vibrancy::clear_vibrancy(&app_window).unwrap();

    #[cfg(target_os = "windows")]
    window_vibrancy::clear_mica(&app_window).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![apply_window_vibrancy, clear_window_vibrancy])
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
