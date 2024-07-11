use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

pub fn options() -> SystemTray {
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new().add_item(quit).add_item(hide);

    SystemTray::new().with_menu(tray_menu)
}

pub fn listener_menu_event(app: &tauri::AppHandle, event: tauri::SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let item_handle = app.tray_handle().get_item(&id);
            match id.as_str() {
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                        let title = "Show".to_string();
                        item_handle.set_title(&title).unwrap();
                    } else {
                        window.show().unwrap();
                        item_handle.set_title("Hide".to_string()).unwrap();
                    }
                    // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                }
                "quit" => app.exit(0),
                _ => {}
            }
        }
        _ => {}
    }
}
