# WhatheRust

<img src="apps/desktop/src-tauri/icons/icon.png" alt="WhatheRust logo" width="128">

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
apps/
├── desktop/
│   ├── src/                            Static fallback required by Tauri
│   └── src-tauri/
│       ├── Cargo.toml                  Rust package and dependencies
│       ├── build.rs                    Tauri build integration
│       ├── tauri.conf.json             Window, frontend, bundle, and updater configuration
│       ├── capabilities/whatsapp-web.json
│       ├── icons/                      App and tray icons; SVG files are the sources
│       └── src/                        Webview, tray, and plugin setup
└── web/                                SvelteKit website
.github/workflows/                      CI and release workflows
package.json                            Root scripts and package manager
pnpm-workspace.yaml                     Workspace package list
turbo.json                              Monorepo task configuration
```

## Prerequisites

- Rust stable installed with [rustup](https://rustup.rs/).
- Tauri CLI v2: `cargo install tauri-cli --version "^2" --locked`.
- Linux: `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`, `libxdo-dev`, and `libssl-dev` (see the [Tauri Linux prerequisites](https://v2.tauri.app/start/prerequisites/)).
- Windows: the WebView2 runtime (preinstalled on Windows 10/11) and MSVC build tools.
- macOS: Xcode Command Line Tools.

## Run and build

Install the workspace dependencies from the repository root:

```sh
pnpm install
```

Run the desktop wrapper or website from the repository root:

```sh
pnpm dev:desktop
pnpm dev:web
```

Build the desktop bundles from the repository root:

```sh
pnpm --filter @whatherust/desktop tauri build
```

Alternatively, run `cargo tauri dev` from `apps/desktop/src-tauri`. Bundles are written under
`apps/desktop/src-tauri/target/release/bundle/`.

## Auto-update

Release builds check GitHub Releases for updates on launch. Updates are signed with the updater
key; configure the `TAURI_SIGNING_PRIVATE_KEY` repository secret for release builds. Push a `vX.Y.Z`
tag to create a draft release, which is published after all platform builds succeed.

## Website

The SvelteKit website lives in `apps/web`. Run it with `pnpm dev:web`; an optional `GITHUB_TOKEN`
environment variable raises the GitHub API rate limit without needing any scopes. The site is
deployed on Vercel.

## Limitations

- Clicking a native notification does not focus the corresponding chat.
- Calls and voice features may not work in WebKitGTK.
- This is an experimental wrapper scaffold; it is not an independent WhatsApp client.

## Icons

The icons are generated from the SVG sources in `apps/desktop/src-tauri/icons/`:

```sh
cd apps/desktop/src-tauri
rsvg-convert -w 1024 -h 1024 icons/icon.svg -o /tmp/icon.png
cargo tauri icon /tmp/icon.png -o icons
rsvg-convert -w 44 -h 44 icons/tray-template.svg -o icons/tray-template.png
```
