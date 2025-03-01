use std::sync::Mutex;
use tauri::Manager;
use window_vibrancy::NSVisualEffectMaterial;

pub mod tray;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn lock_window(app_handle: tauri::AppHandle) -> bool {
    static WINDOW_ALWAYS_ON_TOP: Mutex<bool> = Mutex::new(false);
    let window = app_handle.get_window("main").unwrap();
    let mut always_on_top = WINDOW_ALWAYS_ON_TOP.lock().unwrap();
    *always_on_top = !*always_on_top;
    println!("always_on_top: {}", *always_on_top);
    window.set_always_on_top(*always_on_top).unwrap();
    *always_on_top
}

#[tauri::command]
fn close_window(app_handle: tauri::AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    window.close().unwrap();
}

#[tauri::command]
fn minimize_window(app_handle: tauri::AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    window.minimize().unwrap();
}

#[tauri::command]
fn maximize_window(app_handle: tauri::AppHandle) {
    let window = app_handle.get_window("main").unwrap();
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
fn window_vibrancy(app_handle: tauri::AppHandle) -> bool {
    static IS_WINDOW_VIBRANCY: Mutex<bool> = Mutex::new(false);
    let window = app_handle.get_window("main").unwrap();
    let mut vibrancy = IS_WINDOW_VIBRANCY.lock().unwrap();
    if *vibrancy {
        #[cfg(target_os = "macos")]
        window_vibrancy::apply_vibrancy(&win, NSVisualEffectMaterial::FullScreenUI).unwrap();

        #[cfg(target_os = "windows")]
        window_vibrancy::apply_mica(&window, None).unwrap();
    } else {
        #[cfg(target_os = "macos")]
        window_vibrancy::clear_vibrancy(&win).unwrap();

        #[cfg(target_os = "windows")]
        window_vibrancy::clear_mica(&window).unwrap();
    }
    *vibrancy = !*vibrancy;
    *vibrancy
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, lock_window, close_window, minimize_window, maximize_window, window_vibrancy])
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
