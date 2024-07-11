// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::{Command as TCommand, CommandEvent};
use tauri::{Manager, Menu, WindowBuilder, WindowEvent, WindowUrl};

mod commands;
mod parts;
mod services;

fn main() {
    tauri::Builder::default()
        .plugin(commands::init())
        .menu(parts::menu::create_menu())
        .on_menu_event(parts::menu::listener_menu_event)
        .system_tray(parts::tray::options())
        .on_system_tray_event(parts::tray::listener_menu_event)
        .setup(|app| {
            let app_handle = app.handle();

            // TODO Judge the login status
            // Create login window
            let login_window = WindowBuilder::new(app, "login", WindowUrl::App("/login".into()))
                .title("Studio Login")
                .resizable(false)
                .minimizable(false)
                .menu(Menu::new())
                .inner_size(380.into(), 460.into())
                .center()
                .build()
                .expect("The login window failed to load!");

            // Listening window events
            login_window.on_window_event(move |event| match event {
                WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();
                    app_handle.exit(0);
                }
                _ => {}
            });

            // login_window.on_window_event(move |event| {
            //     if let WindowEvent::CloseRequested { .. } = event {
            //         let app_handel = app.handle();
            //         app_handel.exit(0);
            //     }
            // });
            // login_window.show();
            // let main_window = app.get_window("main").unwrap();
            // // 我们在新任务上执行初始化代码，这样应用程序就不会冻结
            // tauri::async_runtime::spawn(async move {
            //     // 在这里初始化而不是消磨时间
            //     println!("Initializing...");
            //     // std::thread::sleep(std::time::Duration::from_secs(2));
            //     println!("Done initializing.");

            //     // 显示主窗口
            //     splashscreen_window.close().unwrap();
            //     main_window.show().unwrap();
            // });
            // 处理执行结果

            let (mut rx, mut child) = TCommand::new_sidecar("go")
                .expect("failed to create `go` binary command")
                .spawn()
                .expect("Failed to spawn sidecar");
            let main_window = app.get_window("main").unwrap();

            // 定义关闭信号
            tauri::async_runtime::spawn(async move {
                // read events such as stdout
                while let Some(event) = rx.recv().await {
                    if let CommandEvent::Stdout(line) = event {
                        println!("【Go】: {}", line);
                        main_window
                            .emit("message", Some(format!("'{}'", line)))
                            .expect("failed to emit event");

                        // write to stdin
                        child.write("message from Rust\n".as_bytes()).unwrap();
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
