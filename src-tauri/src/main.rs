// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::State;
use tauri_plugin_clipboard::ClipboardManager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn float_image(
    app: tauri::AppHandle,
    clipboard: State<'_, ClipboardManager>,
) -> Result<(), String> {
    if let Ok(image_data_base64) = clipboard.read_image() {
        let js_code = format!(
            r#"window.imgPath = 'data:image/png;base64,{}';"#,
            image_data_base64
        );
        tauri::WindowBuilder::new(&app, "popup", tauri::WindowUrl::App("popup.html".into()))
            .initialization_script(&js_code)
            .decorations(false)
            .maximizable(false)
            .resizable(false)
            .build()
            .expect("window创建失败");
        Ok(())
    } else {
        return Err("没有图呀".into());
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard::init())
        .invoke_handler(tauri::generate_handler![greet, float_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
