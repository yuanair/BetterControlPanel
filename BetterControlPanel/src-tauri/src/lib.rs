use std::sync::Mutex;
use rand::Rng;
use tauri::Manager;
use tauri::path::BaseDirectory;
use crate::game::{Game, QuadraticEquation};

pub mod tray;
pub mod game;

static GAME: Mutex<Option<Game>> = Mutex::new(None);

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

#[tauri::command]
fn create_quadratic_equation() -> String {
    let mut game = GAME.lock().unwrap();
    let QuadraticEquation { a, b, c } = game.as_mut().unwrap().create_quadratic_equation();
    format!("{}x^2 {:+ }x {:+ } = 0", a, b, c)
}

#[tauri::command]
fn is_right(x1: &str, x2: &str) -> (bool, String) {
    let mut game = GAME.lock().unwrap();
    let equation = game.as_ref().unwrap().current_question();
    println!("equation: {:?}, x1: {:?}, x2: {:?}", equation, x1, x2);
    let x1: i64 = match x1.parse() {
        Ok(x) => x,
        Err(e) => return (false, format!("错误的输入：{}", e)),
    };
    let x2: i64 = match x2.parse() {
        Ok(x) => x,
        Err(e) => return (false, format!("错误的输入：{}", e)),
    };
    if let Some(equation) = equation {
        match equation.calculate() {
            Some((x1_result, x2_result)) => {
                let result = x1_result == x1 && x2_result == x2 || x1_result == x2 && x2_result == x1;
                (result, if result { "完全正确！".to_string() } else { "结果错误".to_string() })
            }
            None => (false, "无实数根".to_string()),
        }
    } else {
        (false, "当前没有要解决的问题".to_string())
    }
}

#[tauri::command]
fn get_right() -> Option<(i64, i64)> {
    let mut game = GAME.lock().unwrap();
    let equation = game.as_ref()?.current_question()?;
    equation.calculate()
}

#[tauri::command]
fn run_game(app_handle: tauri::AppHandle) {
    let resource_path = app_handle.path().resolve("resource/", BaseDirectory::Resource).unwrap();
    #[cfg(windows)]
    {
        let mut path = std::path::PathBuf::from(resource_path);
        path.push("ProjectA.exe");
        std::process::Command::new(path).spawn().unwrap();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![apply_window_vibrancy, clear_window_vibrancy, create_quadratic_equation, is_right, get_right, run_game])
        .setup(|app| {
            let game = Game::new();
            let mut game_mutex = GAME.lock().unwrap();
            *game_mutex = Some(game);
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
