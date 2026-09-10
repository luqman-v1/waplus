// Waplus - WhatsApp Fast Desktop Shell (v0.1.3)
mod commands;
mod tray;

use std::path::PathBuf;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_notification::NotificationExt;
use tauri_plugin_updater::UpdaterExt;
use url::Url;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36";

#[cfg(target_os = "windows")]
const BROWSER_ARGS: &str = "--disable-features=msWebOOUI,msPdfOOUI,msSmartScreenProtection,Translate,OptimizationHints,MediaRouter,EdgeShowFeatureRecommendations,PreloadMediaEngagementData,AutofillServerCommunication,CertificateTransparencyComponentUpdater,SafeBrowsing --enable-features=VaapiVideoDecoder,CanvasOopRasterization --enable-gpu-rasterization --enable-zero-copy --disable-extensions --disable-component-extensions-with-background-pages --disable-component-update --disable-domain-reliability --disable-sync --disable-breakpad --disk-cache-size=536870912 --media-cache-size=268435456";

const NOTIFICATION_SCRIPT: &str = r#"
(function() {
    try {
        function invokeIpc(cmd, args) {
            if (window.__TAURI_INTERNALS__ && typeof window.__TAURI_INTERNALS__.invoke === 'function') {
                return window.__TAURI_INTERNALS__.invoke(cmd, args);
            }
            return Promise.reject(new Error('Tauri IPC not available'));
        }

        function WaplusNotification(title, options) {
            const opts = options || {};
            this.title = String(title || '');
            this.body = opts.body ? String(opts.body) : '';
            this.tag = opts.tag ? String(opts.tag) : '';
            this.icon = opts.icon ? String(opts.icon) : '';
            this.silent = Boolean(opts.silent);
            this.data = opts.data || null;

            this.onclick = null;
            this.onclose = null;
            this.onerror = null;
            this.onshow = null;

            this._listeners = {};

            const notifyPayload = {
                options: {
                    title: this.title,
                    body: this.body || undefined,
                    silent: this.silent
                }
            };

            invokeIpc('plugin:notification|notify', notifyPayload)
                .then(() => {
                    const showEvent = { type: 'show', target: this };
                    if (typeof this.onshow === 'function') {
                        try { this.onshow(showEvent); } catch (_) {}
                    }
                    this._dispatch('show', showEvent);
                })
                .catch((err) => {
                    console.warn('Waplus notification error:', err);
                    const errorEvent = { type: 'error', error: err, target: this };
                    if (typeof this.onerror === 'function') {
                        try { this.onerror(errorEvent); } catch (_) {}
                    }
                    this._dispatch('error', errorEvent);
                });
        }

        WaplusNotification.prototype.addEventListener = function(type, listener) {
            if (typeof listener !== 'function') return;
            if (!this._listeners[type]) this._listeners[type] = [];
            this._listeners[type].push(listener);
        };

        WaplusNotification.prototype.removeEventListener = function(type, listener) {
            if (!this._listeners[type]) return;
            this._listeners[type] = this._listeners[type].filter(function(l) {
                return l !== listener;
            });
        };

        WaplusNotification.prototype._dispatch = function(type, event) {
            const listeners = this._listeners[type];
            if (Array.isArray(listeners)) {
                listeners.forEach((fn) => {
                    try { fn.call(this, event); } catch (_) {}
                });
            }
        };

        WaplusNotification.prototype.dispatchEvent = function(event) {
            if (!event || !event.type) return true;
            this._dispatch(event.type, event);
            return true;
        };

        WaplusNotification.prototype.close = function() {
            const closeEvent = { type: 'close', target: this };
            if (typeof this.onclose === 'function') {
                try { this.onclose(closeEvent); } catch (_) {}
            }
            this._dispatch('close', closeEvent);
        };

        Object.defineProperty(WaplusNotification, 'permission', {
            get: function() { return 'granted'; },
            set: function(_) {},
            configurable: true,
            enumerable: true
        });

        WaplusNotification.requestPermission = function(callback) {
            if (typeof callback === 'function') {
                try { callback('granted'); } catch (_) {}
            }
            return Promise.resolve('granted');
        };

        WaplusNotification.maxActions = 2;

        window.Notification = WaplusNotification;

        if (window.ServiceWorkerRegistration && window.ServiceWorkerRegistration.prototype) {
            window.ServiceWorkerRegistration.prototype.showNotification = function(title, options) {
                try {
                    new WaplusNotification(title, options);
                } catch (_) {}
                return Promise.resolve();
            };
        }

        if (navigator.permissions && typeof navigator.permissions.query === 'function') {
            const origQuery = navigator.permissions.query.bind(navigator.permissions);
            navigator.permissions.query = function(parameters) {
                if (parameters && parameters.name === 'notifications') {
                    return Promise.resolve({
                        state: 'granted',
                        name: 'notifications',
                        onchange: null,
                        addEventListener: function() {},
                        removeEventListener: function() {},
                        dispatchEvent: function() { return true; }
                    });
                }
                return origQuery(parameters);
            };
        }
    } catch (err) {
        console.error('Waplus notification bridge init error:', err);
    }
})();
"#;


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
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::clear_session,
            commands::get_app_version,
            commands::set_theme
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
                .additional_browser_args(BROWSER_ARGS)
                .drag_and_drop(false);
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
                .disable_drag_drop_handler()
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
                .on_new_window(|url, _features| {
                    if let Some(host) = url.host_str() {
                        if host == "web.whatsapp.com"
                            || host.ends_with(".whatsapp.com")
                            || host.ends_with(".whatsapp.net")
                        {
                            return tauri::webview::NewWindowResponse::Allow;
                        }
                    }
                    let _ = open::that(url.as_str());
                    tauri::webview::NewWindowResponse::Deny
                })
                .build()?;
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                if let Ok(updater) = handle.updater() {
                    if let Ok(Some(update)) = updater.check().await {
                        let _ = handle
                            .notification()
                            .builder()
                            .title("Waplus Update Available")
                            .body(format!("Version {} is available to install.", update.version))
                            .show();
                    }
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            // Prevent app destroy on close button click for main window, hide to tray instead
            if window.label() == "main" {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Waplus desktop application");
}
