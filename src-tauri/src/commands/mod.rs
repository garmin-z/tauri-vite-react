use serde::Deserialize;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Menu, Runtime, Window,
};
mod github;
mod updsocket;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 定义一个结构体来接收对象参数
#[derive(Deserialize)]
struct CheckHostAllowRd {
    ip: String,
    port: String,
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn check_host_allow(data: CheckHostAllowRd) -> Result<String, String> {
    let url = format!("http://{}:{}/check", data.ip, data.port);
    let response = reqwest::get(url).await.expect(" Error");

    if response.status().is_success() {
        let body = response.text().await.expect(" Error");
        println!("Response: {}", body);
        Ok(format!("ok"))
    } else {
        println!("Request failed with status: {}", response.status());
        Err(format!("Request failed with status: {}", response.status()))
    }
}

#[tauri::command]
async fn close_splashscreen<R: Runtime>(app: Window<R>) {
    // 关闭初始屏幕
    if let Some(splashscreen) = app.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // 显示主窗口
    app.get_window("main").unwrap().show().unwrap();
}

// remember to call `.manage(MyState::default())`
#[tauri::command]
fn open_setting_page<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    std::thread::spawn(move || {
        let main = app.get_window("main").clone().unwrap();
        let hwnd = main.hwnd().unwrap();
        tauri::WindowBuilder::new(&app, "setting", tauri::WindowUrl::App("/setting".into()))
            .title("Setting")
            .resizable(false)
            .minimizable(false)
            .menu(Menu::new())
            .owner_window(hwnd)
            .build()
            .expect(" Error");

        // ds.hide().unwrap();
    });
    Ok(())
}

#[tauri::command]
fn show_setting_page<R: Runtime>(app: tauri::AppHandle<R>) -> Result<(), String> {
    app.get_window("setting").unwrap().show().unwrap();
    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::<R>::new("commands")
        .invoke_handler(tauri::generate_handler![
            greet,
            check_host_allow,
            close_splashscreen,
            open_setting_page,
            show_setting_page,
            updsocket::start_udp_broadcast,
            updsocket::stop_udp_listener
        ])
        .build()
}
