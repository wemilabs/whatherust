# WhatheRust

<img src="src-tauri/icons/icon.png" alt="WhatheRust logo" width="128">

An unofficial native-webview wrapper for the official WhatsApp Web client, named WhatheRust so it isn't confused with Meta's own WhatsApp desktop app. It uses the operating system's webview instead of bundling Electron and Chromium.

## Why Tauri instead of Electron?

Tauri uses WebView2, WKWebView, or WebKitGTK provided by the operating system rather than shipping Chromium and Node.js, resulting in a smaller binary and lower memory use. WhatsApp has no official third-party API; reusing its official web client avoids reimplementing the protocol and the associated account-ban risk.

## Features

- Persistent session data at `<app local data dir>/webview` (for example, `~/.local/share/com.wemilabs.whatherust/webview` on Linux).
- Desktop Chrome user agent for compatibility with WhatsApp Web.
- System tray with show/quit actions, close-to-tray behavior, and the unread count in the tray tooltip.
- Native notifications through Tauri's notification plugin.
- External links open in the system browser.
- Single-instance behavior.

## Architecture

```text
src/index.html                         Static fallback required by Tauri
src-tauri/
├── Cargo.toml                          Rust package and dependencies
├── build.rs                            Tauri build integration
├── tauri.conf.json                     Window, frontend, and bundle configuration
├── capabilities/whatsapp-web.json      Remote notification permissions
├── icons/                              App and tray icons; icon.svg and tray-template.svg are the sources
└── src/
    ├── lib.rs                          Webview, tray, and plugin setup
    └── main.rs                         Application entry point
```

## Prerequisites

- Rust stable installed with [rustup](https://rustup.rs/).
- Tauri CLI v2: `cargo install tauri-cli --version "^2" --locked`.
- Linux: `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `libxdo-dev`, and `libssl-dev` (see the [Tauri Linux prerequisites](https://v2.tauri.app/start/prerequisites/)).
- Windows: the WebView2 runtime (preinstalled on Windows 10/11) and MSVC build tools.
- macOS: Xcode Command Line Tools.

## Run and build

From the `src-tauri` directory:

```sh
cargo tauri dev
cargo tauri build
```

Bundles are written under `src-tauri/target/release/bundle/`.

## Limitations

- Clicking a native notification does not focus the corresponding chat.
- Calls and voice features may not work in WebKitGTK.
- This is an experimental wrapper scaffold; it is not an independent WhatsApp client.

## Icons

The icons are generated from the SVG sources in `src-tauri/icons/`:

```sh
rsvg-convert -w 1024 -h 1024 icons/icon.svg -o /tmp/icon.png
cargo tauri icon /tmp/icon.png -o /tmp/icons  # copy 32x32, 128x128, 128x128@2x, icon.png, icon.icns, icon.ico into icons/
rsvg-convert -w 44 -h 44 icons/tray-template.svg -o icons/tray-template.png
```
