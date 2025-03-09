use tauri::{
    menu::{Menu, MenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
    Runtime,
};

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    // let edit_i = MenuItem::with_id(app, "edit_file", "编辑", true, None::<&str>)?;
    // let new_i = MenuItem::with_id(app, "new_file", "添加", true, None::<&str>)?;
    // let a = Submenu::with_id_and_items(app, "File", "文件", true, &[&new_i, &edit_i])?;
    // 分割线
    // let menu = Menu::with_items(app, &[&a, &quit_i])?;
    // 分割线
    let menu = Menu::with_items(app, &[&quit_i])?;

    let _ = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                for (_, window) in app.windows() {
                    let _ = window.destroy();
                }
                app.exit(0);
            }
            "edit_file" => {
                // TODO: Implement file edit logic here
            }
            "new_file" => {
                // TODO: Implement file add logic here
            }
            // Add more events here
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app);

    Ok(())
}

