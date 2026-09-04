# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-09-04

### Added
- **Single-WebView Architecture**: Native, high-performance WebView2 shell loading official WhatsApp Web directly with zero secondary webview overhead.
- **Resource & Memory Optimization**: Reduced memory consumption to ~250–350 MB RAM with Win32 working-set trimming and Chromium flag optimizations.
- **Instant Warm Resume (< 200 ms)**: Seamless minimize-to-tray logic via Windows System Tray for instant wakeup without cold boot latency.
- **Encrypted Persistent Session**: Dedicated data directory (`%APPDATA%/Waplus/webview`) preserving logins, sessions, and indexedDB across app restarts and reboots.
- **External Link Interceptor**: Automatically forwards external hyperlinks to the operating system's default browser.
- **Native Notification Bridge**: Integrated notification bridge script to auto-resolve permissions and display desktop alerts.
- **On-Demand Settings**: Lightweight Svelte 5 settings window supporting theme selection (System / Dark / Light) and deep session reset.
- **Custom Branded Icon**: Native transparent squircle icon integrated across Windows taskbar, titlebar, and system tray.
- **Automated CI/CD**: GitHub Actions workflow to build release installers and create GitHub releases on tag push.
