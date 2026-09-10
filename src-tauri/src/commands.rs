use tauri::{AppHandle, Manager};
use url::Url;

#[tauri::command]
pub async fn clear_session(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.clear_all_browsing_data();
        let url: Url = "https://web.whatsapp.com"
            .parse()
            .map_err(|e: url::ParseError| e.to_string())?;
        let _ = window.navigate(url);
    }
    Ok(())
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
