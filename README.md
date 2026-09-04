# Waplus ⚡

A high-performance, resource-efficient native desktop shell for WhatsApp Web built with **Tauri v2**, **Svelte 5**, **TypeScript**, and **Rust**, powered by **Bun**.

<p align="center">
  <img src="static/logo.png" alt="Waplus Logo" width="120" height="120" style="border-radius: 24px;" />
</p>

<p align="center">
  <strong>Fast. Lightweight. Privacy-focused.</strong>
</p>

---

## ✨ Features

- 🚀 **Ultra-Low Memory Footprint**: Optimized Chromium WebView2 runtime consuming only ~250–350 MB RAM (compared to ~1,000+ MB on Electron / multi-process wrappers).
- ⚡ **Instant Warm Resume (< 200 ms)**: Minimizes directly to the Windows System Tray on close and restores instantly.
- 🔒 **Encrypted Persistent Session**: Session data, credentials, and local storage persist securely across application restarts and OS reboots.
- 🌐 **External Link Interception**: External hyperlinks automatically open in your default operating system browser, keeping your WhatsApp shell clean and focused.
- 🔔 **Native Notification Bridge**: Background notification support with auto-granted permissions.
- 🎨 **Svelte 5 Runes & Modern UI**: Fast, responsive on-demand settings window with light/dark appearance controls and deep session clearing.
- 🖼️ **Custom Branded Icon**: Native squircle icon with transparency across Windows Taskbar and System Tray.

---

## 🛠️ Tech Stack

- **Desktop Framework**: [Tauri v2](https://v2.tauri.app/)
- **Backend Language**: [Rust](https://www.rust-lang.org/) (with Win32 memory working-set trimming)
- **Frontend Framework**: [Svelte 5](https://svelte.dev/) + [SvelteKit](https://kit.svelte.dev/) + [TypeScript](https://www.typescriptlang.org/)
- **Package Manager & Runtime**: [Bun](https://bun.sh/)
- **Bundler**: [Vite](https://vitejs.dev/)

---

## 📋 Prerequisites

Before running or building Waplus, make sure you have installed:

1. [Bun](https://bun.sh/) (v1.1+ recommended)
2. [Rust toolchain](https://rustup.rs/) (Stable 2021 edition)
3. [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/) (Pre-installed on Windows 10/11)
4. C++ Build Tools (MSVC or MinGW toolchain)

---

## 🚀 Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/waplus.git
cd waplus
```

### 2. Install Dependencies

```bash
bun install
```

### 3. Run in Development Mode

```bash
bun run tauri dev
```

---

## 📦 Building for Production

To compile an optimized, standalone executable installer for Windows:

```bash
bun run tauri build
```

The compiled binaries (`.msi` installer or standalone `.exe`) will be generated inside `src-tauri/target/release/bundle/`.

---

## 🛡️ Security & Privacy

- **No Remote Telemetry**: Waplus does not collect, log, or transmit personal conversations or analytics.
- **Direct WhatsApp Connection**: The client communicates directly with official `web.whatsapp.com` servers via WebView2.
- **Least Privilege Permissions**: Tauri security capabilities restrict access strictly to essential system window, tray, notification, and opener APIs.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
