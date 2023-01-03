#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use tauri::{Manager, RunEvent, WindowEvent};
use tauri_plugin_store::PluginBuilder;
use tokio::sync::mpsc;
use device_query::{DeviceEvents, DeviceState};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};

#[tokio::main]
async fn main() {

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show window..."))
        .add_item(CustomMenuItem::new("add-shortcut".to_string(), "Add shortcut..."))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {

                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                    "add-shortcut"=>{
                        let main_window = app.get_window("main").unwrap();
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                        main_window.emit("add-shortcut", "").unwrap();
                    }

                    _ => {}
                }

            }
            _ => {}
        }
        )
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let main_window = app.get_window("main").unwrap();
            let splashscreen = app.get_window("splashscreen").unwrap();

            app.listen_global("main::loaded", move |event| {
                splashscreen.close().unwrap();
                main_window.show().unwrap();
            });

            Ok(())
        })
        .enable_macos_default_menu(true)
        .invoke_handler(tauri::generate_handler![
        ])
        .on_window_event(move |event| match event.event() {
            WindowEvent::FileDrop(file_drop_event) => {
                println!("File dropped: {:?}", file_drop_event);
            }
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("Error building app")
        .run(|app_handle, event| match event {
            // clone app_handler
            RunEvent::Ready => {
                println!("App is ready, starting background tasks");
                let new_handler = app_handle.clone();

                tauri::async_runtime::spawn(async move {

                    let device_state = DeviceState::new();

                    let (tx, mut rx) = mpsc::channel::<i32>(100);

                    let key_down_tx = tx.clone();
                    let _guard = device_state.on_key_down( move |key| {
                        if key == &device_query::Keycode::LControl {
                            let _ = key_down_tx.try_send(1);
                        } else if key == &device_query::Keycode::LAlt {
                            let _ = key_down_tx.try_send(2);
                        } else if key == &device_query::Keycode::LShift {
                            let _ = key_down_tx.try_send(3);
                        }
                    });

                    let keyup_tx = tx.clone();
                    let _guard = device_state.on_key_up( move |key| {
                        if key == &device_query::Keycode::LControl {
                            let _ = keyup_tx.try_send(-1);
                        } else if key == &device_query::Keycode::LAlt {
                            let _ = keyup_tx.try_send(-2);
                        } else if key == &device_query::Keycode::LShift {
                            let _ = keyup_tx.try_send(-3);
                        }
                    });

                    let mut current_state = 0;
                    let mut is_show = false;
                    loop {
                        if let Some(value) = rx.recv().await {
                            current_state += value;
                        }

                        if current_state == 6 {
                            is_show = true;
                            let main_window = new_handler.get_window("main").unwrap();
                            main_window.show().unwrap();
                            main_window.set_always_on_top(true).unwrap();

                        } else if current_state == 0 && is_show {
                            is_show = false;
                            let main_window = new_handler.get_window("main").unwrap();
                            main_window.hide().unwrap();
                        }
                    }
                });
            }
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        })
}
