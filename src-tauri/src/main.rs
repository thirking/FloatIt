// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::notification::Notification;
use tauri::GlobalShortcutManager;
use tauri::{App, Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use window_shadows::set_shadow;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn float_image(app: &tauri::AppHandle) {
    let handle = app.app_handle();
    let clipboard = handle.state::<tauri_plugin_clipboard::ClipboardManager>();
    if let Ok(image_data_base64) = clipboard.read_image() {
        let img_path = format!(r#"data:image/png;base64,{}"#, image_data_base64);
        let win = app.get_window("main").expect("get window failed");
        win.show().expect("window show failed");
        win.emit("setImage", Payload { message: img_path })
            .expect("set image failed");
    } else {
        Notification::new(&app.config().tauri.bundle.identifier)
            .title("Image not found!")
            .body("Can't find image in your clipboard.")
            .show()
            .unwrap();
    }
}

fn toggle_main_window(app: &tauri::AppHandle) {
    let window = app.get_window("main").unwrap();
    if window.is_visible().unwrap() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
    }
}

const FLOATSHORTCUT: &str = "Ctrl+Shift+F1";
const TOGGLESHORTCUT: &str = "Ctrl+F2";

fn register_shortcut(app: &mut App) -> Result<(), tauri::Error> {
    let app_handle = app.app_handle();
    let mut shortcuts = app_handle.global_shortcut_manager();
    // Only register if we haven't already assigned something to
    // this key combo
    let app_copy = app_handle.clone();
    if !shortcuts.is_registered(FLOATSHORTCUT)? {
        shortcuts.register(FLOATSHORTCUT, move || float_image(&app_handle))?;
        shortcuts.register(TOGGLESHORTCUT, move || toggle_main_window(&app_copy))?;
    }
    Ok(())
}

fn main() {
    let float = CustomMenuItem::new("float".to_string(), "FloatIt");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(float)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "float" => float_image(&app),
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {}
        })
        .plugin(tauri_plugin_clipboard::init())
        .setup(|app| {
            register_shortcut(app).unwrap();
            let window = app.get_window("main").unwrap();
            set_shadow(window, true).expect("Unsupported platform!");
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("Float it!")
                .body("You can find it in the tray.")
                .show()
                .unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
