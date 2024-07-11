use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

pub fn listener_menu_event(event: WindowMenuEvent) {
    match event.menu_item_id() {
        "quit" => {
            // std::process::exit(0);
            println!("quit")
        }
        "close" => {
            // event.window().close().unwrap();
            println!("close")
        }
        "hide" => {
            println!("hide")
            // event.window().hide().unwrap();
        }
        _ => {}
    }
}

pub fn create_menu() -> Menu {
    // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));

    Menu::new()
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu)
}
