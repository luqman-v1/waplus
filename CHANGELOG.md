# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.2] - 2026-09-10

### 🔔 Native Desktop Notification Bridge
- **Direct Tauri IPC Bridge**: Intercepts `new window.Notification()` and dispatches payloads to Tauri's `plugin:notification|notify` command for native Windows Toast delivery.
- **WhatsApp Web Remote Capabilities**: Added `whatsapp.json` capability scoped to `https://web.whatsapp.com/**` with permissions for desktop notifications and openers.
- **Service Worker & Permissions Interceptor**: Intercepts `ServiceWorkerRegistration.prototype.showNotification` and `navigator.permissions.query({ name: 'notifications' })` ensuring background alert compatibility.
- **Automated E2E Contract Tests**: Added test suite validating notification script integrity, DOM EventTarget listeners, and Tauri IPC payload dispatch.

### 🐛 Bug Fixes & Shell Improvements
- **Safe Session Clearing**: Refactored `clear_session` command to utilize native `window.clear_all_browsing_data()`, resolving Windows file-locking permission errors (`os error 5 / 32`).
- **External Link Interceptor**: Added `.on_new_window()` handler to intercept `target="_blank"` and `window.open()` popups, opening all external URLs in the default OS browser while denying unmanaged webviews.
- **Developer Tooling**: Added root `Makefile` providing quick targets for `run`, `dev`, `build`, `check`, and `test`.

## [0.1.1] - 2026-09-05

### 🚀 Performance & Responsiveness
- **GPU Hardware Acceleration**: Enabled GPU rasterization, zero-copy buffer transfer, and OOP canvas rasterization for smooth 60 FPS chat transitions and scrolling.
- **Removed Artificial Bottlenecks**: Eliminated V8 heap limit caps (`--max-old-space-size=256`), low-end device throttling (`--enable-low-end-device-mode`), and single-site renderer limits for instant chat opening.
- **Expanded Disk & Media Cache**: Increased disk cache to 512 MB and media cache to 256 MB with GPU shader disk caching, making contact avatars, stickers, and media previews load instantly from local storage.
- **Zero-Lag Window Wakeup**: Removed aggressive Windows pagefile working-set flushing on minimize/hide, ensuring the window restores instantly from system tray without freezing.

### ✨ Auto-Update System
- **Cryptographically Signed Updates**: Integrated `tauri-plugin-updater` with automated Minisign verification.
- **Background Startup Check**: Non-blocking update check on application launch with native desktop alerts when a new version is released.
- **Interactive In-App Updater**: Added Software Update manager in Settings with live download progress bar, changelog display, and seamless one-click installation.
- **CI/CD Auto-Sign Pipeline**: GitHub Actions release workflow now automatically signs multi-platform installers and generates `latest.json`.

### 🎨 UI & Settings Enhancements
- **Settings Polish**: Added official author credit to [@luqman-v1](https://github.com/luqman-v1) with direct link opener, replacing development framework labels.
- **Dynamic Version Reporting**: Settings window now retrieves and displays the active runtime app version dynamically.
- **Download Documentation**: Added ready-to-use release download tables and badges directly in `README.md`.

### 🔒 Core Shell Features
- **Single-WebView Architecture**: Native, high-performance WebView2 desktop shell loading official WhatsApp Web directly with zero secondary webview overhead.
- **Instant Warm Resume**: Seamless minimize-to-tray logic via Windows System Tray for instant wakeup without cold boot latency.
- **Encrypted Persistent Session**: Dedicated data directory preserving logins, sessions, and indexedDB across app restarts and reboots.
- **External Link Interceptor**: Automatically forwards external hyperlinks to your operating system's default browser.
- **Native Notification Bridge**: Background notification bridge to auto-resolve permissions and display desktop alerts.
- **Custom Branded Icon**: Native transparent squircle icon integrated across Windows taskbar, titlebar, and system tray.

## [0.1.0] - 2026-09-04

### 🚀 Initial Release
- Initial release of Waplus desktop shell.
