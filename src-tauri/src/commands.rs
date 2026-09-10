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

#[tauri::command]
pub fn set_theme(app: AppHandle, theme: String) -> Result<(), String> {
    let tauri_theme = match theme.as_str() {
        "dark" => Some(tauri::Theme::Dark),
        "light" => Some(tauri::Theme::Light),
        _ => None,
    };
    if let Some(main_win) = app.get_webview_window("main") {
        let _ = main_win.set_theme(tauri_theme);
        let js = match theme.as_str() {
            "dark" => "document.body.classList.add('dark'); try { window.localStorage.setItem('theme', '\"dark\"'); } catch(_) {}",
            "light" => "document.body.classList.remove('dark'); try { window.localStorage.setItem('theme', '\"light\"'); } catch(_) {}",
            _ => "try { window.localStorage.setItem('theme', '\"system\"'); } catch(_) {}; if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) { document.body.classList.add('dark'); } else { document.body.classList.remove('dark'); }",
        };
        let _ = main_win.eval(js);
    }
    if let Some(settings_win) = app.get_webview_window("settings") {
        let _ = settings_win.set_theme(tauri_theme);
    }
    Ok(())
}
