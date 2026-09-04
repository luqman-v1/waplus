// Waplus - WhatsApp Fast Desktop Shell (v0.1.0)
mod commands;
mod tray;

use std::path::PathBuf;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use url::Url;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36";

#[cfg(target_os = "windows")]
const BROWSER_ARGS: &str = "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection,Translate,OptimizationHints,MediaRouter,EdgeShowFeatureRecommendations,PreloadMediaEngagementData,AutofillServerCommunication,CertificateTransparencyComponentUpdater,SafeBrowsing --enable-low-end-device-mode --renderer-process-limit=2 --process-per-site --disable-extensions --disable-component-extensions-with-background-pages --disable-background-networking --disable-component-update --disable-domain-reliability --disable-sync --disable-breakpad --disable-gpu-shader-disk-cache --disk-cache-size=67108864 --media-cache-size=33554432 --js-flags=--max-old-space-size=256 --optimize-for-size --expose-gc";

const NOTIFICATION_SCRIPT: &str = r#"
(function() {
    try {
        const OrigNotification = window.Notification;
        function WaplusNotification(title, options) {
            try {
                if (OrigNotification) {
                    return new OrigNotification(title, options);
                }
            } catch (e) {}
            return {
                title: title,
                options: options,
                addEventListener: function() {},
                removeEventListener: function() {},
                close: function() {}
            };
        }

        Object.defineProperty(WaplusNotification, 'permission', {
            get: function() { return 'granted'; },
            set: function(_) {},
            configurable: true,
            enumerable: true
        });

        WaplusNotification.requestPermission = function(callback) {
            if (typeof callback === 'function') {
                callback('granted');
            }
            return Promise.resolve('granted');
        };

        if (OrigNotification) {
            try {
                Object.defineProperty(OrigNotification, 'permission', {
                    get: function() { return 'granted'; },
                    set: function(_) {},
                    configurable: true,
                    enumerable: true
                });
                OrigNotification.requestPermission = WaplusNotification.requestPermission;
            } catch (_) {}
        }

        window.Notification = WaplusNotification;

        // Auto Memory Reclamation when window is hidden or idle
        document.addEventListener('visibilitychange', function() {
            if (document.hidden && typeof window.gc === 'function') {
                try { window.gc(); } catch (_) {}
            }
        });
        setInterval(function() {
            if (document.hidden && typeof window.gc === 'function') {
                try { window.gc(); } catch (_) {}
            }
        }, 45000);
    } catch (err) {
        console.error('Waplus notification bridge init error:', err);
    }
})();
"#;

#[cfg(target_os = "windows")]
pub fn trim_memory() {
    unsafe {
        extern "system" {
            fn GetCurrentProcess() -> isize;
            fn SetProcessWorkingSetSize(
                h_process: isize,
                dw_minimum_working_set_size: usize,
                dw_maximum_working_set_size: usize,
            ) -> i32;
        }
        let handle = GetCurrentProcess();
        SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            tray::show_main_window(app);
        }))
        .invoke_handler(tauri::generate_handler![
            commands::clear_session,
            commands::get_app_version
        ])
        .setup(|app| {
            // Setup system tray
            tray::create_tray(app.handle())?;

            // Create single high-performance main window for WhatsApp Web
            let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| PathBuf::from("."));
            let webview_data_dir = app_data_dir.join("webview");
            let whatsapp_url = WebviewUrl::External("https://web.whatsapp.com".parse().unwrap());

            #[cfg(target_os = "windows")]
            let builder = WebviewWindowBuilder::new(app, "main", whatsapp_url)
                .additional_browser_args(BROWSER_ARGS);
            #[cfg(not(target_os = "windows"))]
            let builder = WebviewWindowBuilder::new(app, "main", whatsapp_url);

            let _main_window = builder
                .title("Waplus")
                .inner_size(1200.0, 800.0)
                .min_inner_size(800.0, 600.0)
                .center()
                .user_agent(USER_AGENT)
                .data_directory(webview_data_dir)
                .initialization_script(NOTIFICATION_SCRIPT)
                .on_navigation(|url: &Url| {
                    if let Some(host) = url.host_str() {
                        if host == "web.whatsapp.com"
                            || host.ends_with(".whatsapp.com")
                            || host.ends_with(".whatsapp.net")
                        {
                            return true;
                        }
                    }
                    let _ = open::that(url.as_str());
                    false
                })
                .build()?;

            Ok(())
        })
        .on_window_event(|window, event| {
            // Prevent app destroy on close button click for main window, hide to tray instead
            if window.label() == "main" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                    #[cfg(target_os = "windows")]
                    trim_memory();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Waplus desktop application");
}
