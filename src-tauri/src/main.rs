#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{Manager, PhysicalPosition, PhysicalSize, RunEvent, WindowEvent, Window};
use tauri_plugin_autostart::MacosLauncher;
use tokio::sync::mpsc;
use device_query::{DeviceEvents, DeviceState, Keycode, DeviceQuery};
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Wry};
use tauri::Size::Physical;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};
use window_vibrancy::NSVisualEffectState::Active;

const KEYCODE_LCONTROL: i32 = 2;
const KEYCODE_LALT: i32 = 4;
const KEYCODE_LSHIFT: i32 = 8;
const KEYCODE_META: i32 = 16;

// event keycodes struct
#[derive(Clone, serde::Serialize)]
struct ModState {
    ctrl: bool,
    alt: bool,
    shift: bool,
    meta: bool,
}

#[tauri::command]
fn show_viewer(window: tauri::Window<Wry>) {
    window.show().unwrap();
    window.set_always_on_top(true).unwrap();
}

#[tauri::command]
fn hide_viewer(window: tauri::Window<Wry>) {
    window.hide().unwrap();
    window.set_always_on_top(false).unwrap();
}

#[tokio::main]
async fn main() {

    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Show window..."))
        .add_item(CustomMenuItem::new("add-shortcut".to_string(), "Add shortcut..."))
        .add_item(CustomMenuItem::new("settings".to_string(), "Settings"))
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit"));

    tauri::Builder::default()
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
        .invoke_handler(tauri::generate_handler![show_viewer, hide_viewer])
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

                    let (tx, mut rx) = mpsc::channel::<Keycode>(1);

                    let keydown_tx = tx.clone();
                    /*let _guard = device_state.on_key_down( move |key| {
                        println!("Key down: {:?}", key);
                        keydown_tx.try_send(*key).unwrap();
                    });*/

                    let keyup_tx = tx.clone();
                   /*let _guard = device_state.on_key_up( move |key| {
                        println!("Key up: {:?}", key);
                        keyup_tx.try_send(*key).unwrap();
                    });*/

                    let mut is_mod = false;
                    let viewer_window = new_handler.get_window("viewer").unwrap();

                    let mut keycodes = ModState {
                        ctrl: false,
                        alt: false,
                        shift: false,
                        meta: false,
                    };

                    let mut prev_keys = vec![];

                    loop {

                        let keys = device_state.get_keys();
                        if keys != prev_keys {
                            if keys.contains(&Keycode::LControl) {
                                keycodes.ctrl = true;
                            } else {
                                keycodes.ctrl = false;
                            }

                            if keys.contains(&Keycode::LAlt) {
                                keycodes.alt = true;
                            } else {
                                keycodes.alt = false;
                            }

                            if keys.contains(&Keycode::LShift) {
                                keycodes.shift = true;
                            } else {
                                keycodes.shift = false;
                            }

                            if keys.contains(&Keycode::Meta) {
                                keycodes.meta = true;
                            } else {
                                keycodes.meta = false;
                            }

                            viewer_window.emit(
                                "shortcuts",
                                keycodes.clone()
                            ).unwrap();

                            prev_keys = keys;
                        }

                        tokio::time::sleep(std::time::Duration::from_millis(400)).await;
                    }
                });
            }
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        })
}
