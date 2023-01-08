#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use tauri::{Manager, PhysicalPosition, PhysicalSize, RunEvent, WindowEvent, Window};
use tauri_plugin_store::{PluginBuilder, StoreBuilder};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::mpsc;
use device_query::{DeviceEvents, DeviceState};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::Size::Physical;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};
use window_vibrancy::NSVisualEffectState::Active;

const KEYCODE_LCONTROL: i32 = 2;
const KEYCODE_LALT: i32 = 4;
const KEYCODE_LSHIFT: i32 = 8;
const KEYCODE_META: i32 = 16;
const KEYCODE_COMMAND: i32 = 32;

#[tokio::main]
async fn main() {

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show window..."))
        .add_item(CustomMenuItem::new("add-shortcut".to_string(), "Add shortcut..."))
        .add_item(CustomMenuItem::new("settings".to_string(), "Settings"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let main_window = app.get_window("main").unwrap();

                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "show" => {
                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                    "add-shortcut"=>{
                        main_window.emit("router::push", "/projects").unwrap();
                        main_window.emit("add-shortcut", "").unwrap();

                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
                    }
                    "settings" => {
                        main_window.emit("router::push", "/settings").unwrap();

                        main_window.show().unwrap();
                        main_window.set_focus().unwrap();
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
        .invoke_handler(tauri::generate_handler![])
        .on_window_event(move |event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if event.window().label() == "main" {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("Error building app")
        .run(|app_handle, event| match event {
            // clone app_handler
            RunEvent::Ready => {
                let new_handler = app_handle.clone();

                tauri::async_runtime::spawn(async move {

                    let device_state = DeviceState::new();

                    let (tx, mut rx) = mpsc::channel::<i32>(3);

                    let keydown_tx = tx.clone();
                    let _guard = device_state.on_key_down( move |key| {
                        if key == &device_query::Keycode::LControl {
                            let _ = keydown_tx.try_send(KEYCODE_LCONTROL);
                        } else if key == &device_query::Keycode::LAlt {
                            let _ = keydown_tx.try_send(KEYCODE_LALT);
                        } else if key == &device_query::Keycode::LShift {
                            let _ = keydown_tx.try_send(KEYCODE_LSHIFT);
                        } else if key == &device_query::Keycode::Meta {
                            let _ = keydown_tx.try_send(KEYCODE_META);
                        }
                    });

                    let keyup_tx = tx.clone();
                    let _guard = device_state.on_key_up( move |key| {
                        if key == &device_query::Keycode::LControl {
                            let _ = keyup_tx.try_send(-KEYCODE_LCONTROL);
                        } else if key == &device_query::Keycode::LAlt {
                            let _ = keyup_tx.try_send(-KEYCODE_LALT);
                        } else if key == &device_query::Keycode::LShift {
                            let _ = keyup_tx.try_send(-KEYCODE_LSHIFT);
                        }else if key == &device_query::Keycode::Meta {
                            let _ = keyup_tx.try_send(-KEYCODE_META);
                        }
                    });

                    let mut current_state = 0;
                    let mut is_show = false;
                    let viewer_window = new_handler.get_window("viewer").unwrap();

                    // get store from manage state
                    let store = app.state::<StoreCollection>();


                    loop {
                        if let Some(value) = rx.recv().await {
                            current_state += value;
                        }

                        if current_state == KEYCODE_LCONTROL + KEYCODE_LALT {
                            is_show = true;

                            viewer_window.show().unwrap();
                            viewer_window.set_always_on_top(true).unwrap();
                            viewer_window.emit("shortcuts", "demo").unwrap();
                        } else if current_state == 0 && is_show {
                            is_show = false;

                            viewer_window.hide().unwrap();
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
