use std::fs;
use tauri::{AppHandle, Manager};
use url::Url;

#[tauri::command]
pub async fn clear_session(app: AppHandle) -> Result<(), String> {
    if let Some(app_data_dir) = app.path().app_data_dir().ok() {
        let webview_dir = app_data_dir.join("webview");

        if let Some(window) = app.get_webview_window("main") {
            let url: Url = "https://web.whatsapp.com"
                .parse()
                .map_err(|e: url::ParseError| e.to_string())?;
            let _ = window.navigate(url);
        }

        if webview_dir.exists() {
            let _ = fs::remove_dir_all(&webview_dir);
        }
    }
    Ok(())
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
