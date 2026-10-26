use std::path::Path;

use tauri::{
    AppHandle, Manager, WebviewWindowBuilder,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::NewWindowResponse,
};
use tauri_plugin_opener::OpenerExt;

const LEGACY_IDENTIFIER: &str = "com.wemilabs.whatsapp-desktop-rs";

#[cfg(target_os = "linux")]
const USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36";
#[cfg(target_os = "windows")]
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36";
#[cfg(target_os = "macos")]
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/146.0.0.0 Safari/537.36";

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_main_window(app);
        }))
        .plugin(tauri_plugin_notification::init())
        // The injected link-click handler calls opener IPC, which the remote page may not use;
        // let new-window requests reach `on_new_window` instead.
        .plugin(
            tauri_plugin_opener::Builder::new()
                .open_js_links_on_click(false)
                .build(),
        )
        .setup(|app| {
            let window_config = app
                .config()
                .app
                .windows
                .iter()
                .find(|config| config.label == "main")
                .ok_or("missing main window configuration")?;
            let app_handle = app.handle().clone();

            let data_dir = app.path().app_local_data_dir()?;
            move_legacy(&data_dir.with_file_name(LEGACY_IDENTIFIER), &data_dir);

            #[cfg(target_os = "macos")]
            migrate_legacy_webkit_data(
                &app.path().home_dir()?.join("Library"),
                &app.config().identifier,
                &app.package_info().name,
            );

            let window = WebviewWindowBuilder::from_config(app.handle(), window_config)?
                // macOS ignores this override; WKWebView persists by bundle identifier.
                .data_directory(data_dir.join("webview"))
                // WhatsApp Web rejects or degrades WebKit user agents.
                .user_agent(USER_AGENT)
                .on_navigation({
                    let app_handle = app_handle.clone();
                    move |url| {
                        if is_allowed_url(url) {
                            true
                        } else {
                            open_external(&app_handle, url.as_str());
                            false
                        }
                    }
                })
                .on_new_window({
                    let app_handle = app_handle.clone();
                    move |url, _features| {
                        open_external(&app_handle, url.as_str());
                        NewWindowResponse::Deny
                    }
                })
                .on_document_title_changed(|webview, title| {
                    if let Some(tray) = webview.app_handle().tray_by_id("main") {
                        let _ = tray.set_tooltip(Some(title.replace("WhatsApp", "WhatheRust")));
                    }
                })
                .build()?;

            #[cfg(target_os = "windows")]
            round_corners(&window);

            #[cfg(target_os = "macos")]
            window.set_background_color(Some(titlebar_color(window.theme()?)))?;

            #[cfg(target_os = "macos")]
            window.with_webview(|webview| {
                // Let WebKit dispatch target=_blank; on_new_window still opens URLs externally.
                unsafe {
                    let webview = &*webview.inner().cast::<objc2_web_kit::WKWebView>();
                    webview
                        .configuration()
                        .preferences()
                        .setJavaScriptCanOpenWindowsAutomatically(true);
                }
            })?;

            let close_window = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    api.prevent_close();
                    let _ = close_window.hide();
                }
                #[cfg(target_os = "macos")]
                if let tauri::WindowEvent::ThemeChanged(theme) = event {
                    let _ = close_window.set_background_color(Some(titlebar_color(*theme)));
                }
            });

            let show_item = MenuItem::with_id(app, "show", "Show WhatheRust", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            let mut tray_builder = TrayIconBuilder::with_id("main")
                .tooltip("WhatheRust")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_tray_icon_event({
                    let app_handle = app_handle.clone();
                    move |_tray, event| {
                        if matches!(
                            event,
                            TrayIconEvent::Click {
                                button: MouseButton::Left,
                                button_state: MouseButtonState::Up,
                                ..
                            }
                        ) {
                            toggle_main_window(&app_handle);
                        }
                    }
                })
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => show_main_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                });
            #[cfg(target_os = "macos")]
            {
                tray_builder = tray_builder
                    .icon(tauri::include_image!("icons/tray-template.png"))
                    .icon_as_template(true);
            }
            #[cfg(not(target_os = "macos"))]
            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            tray_builder.build(app)?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building WhatheRust")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen {
                has_visible_windows: false,
                ..
            } = event
            {
                show_main_window(app);
            }

            #[cfg(not(target_os = "macos"))]
            let _ = (app, event);
        });
}

/// Moves data left by builds made before the app identifier was renamed.
/// Moved, not copied, on purpose: the same WhatsApp linked-device keys must never live in two apps.
fn move_legacy(from: &Path, to: &Path) -> bool {
    from.exists() && !to.exists() && std::fs::rename(from, to).is_ok()
}

#[cfg(target_os = "macos")]
fn migrate_legacy_webkit_data(library: &Path, identifier: &str, app_name: &str) {
    let webkit = library.join("WebKit");
    if !move_legacy(&webkit.join(LEGACY_IDENTIFIER), &webkit.join(identifier)) {
        return;
    }

    let cookies = library.join("HTTPStorages");
    move_legacy(
        &cookies.join(format!("{LEGACY_IDENTIFIER}.binarycookies")),
        &cookies.join(format!("{identifier}.binarycookies")),
    );
    move_legacy(&cookies.join(LEGACY_IDENTIFIER), &cookies.join(identifier));
    move_webcrypto_master_key(identifier, app_name);
}

/// WebKit wraps stored CryptoKeys with this per-bundle-id keychain item; without it, moved IndexedDB keys cannot be unwrapped.
#[cfg(target_os = "macos")]
fn move_webcrypto_master_key(identifier: &str, app_name: &str) {
    use core_foundation::data::CFData;
    use security_framework::item::{
        ItemAddOptions, ItemAddValue, ItemClass, ItemSearchOptions, Location, SearchResult,
    };

    let old_account = format!("com.apple.WebKit.WebCrypto.master+{LEGACY_IDENTIFIER}");
    let mut old_search = ItemSearchOptions::new();
    old_search
        .class(ItemClass::generic_password())
        .account(&old_account)
        .load_data(true)
        .limit(1);
    let Ok(mut old_items) = old_search.search() else {
        return;
    };
    let Some(SearchResult::Data(data)) = old_items.pop() else {
        return;
    };

    let new_account = format!("com.apple.WebKit.WebCrypto.master+{identifier}");
    let mut new_search = ItemSearchOptions::new();
    new_search
        .class(ItemClass::generic_password())
        .account(&new_account);
    let _ = new_search.delete();

    let value = ItemAddValue::Data {
        class: ItemClass::generic_password(),
        data: CFData::from_buffer(&data),
    };
    let mut new_item = ItemAddOptions::new(value);
    new_item
        .set_account_name(&new_account)
        .set_label(format!("{app_name} WebCrypto Master Key"))
        .set_location(Location::DefaultFileKeychain);
    if new_item.add().is_ok() {
        let mut old_delete = ItemSearchOptions::new();
        old_delete
            .class(ItemClass::generic_password())
            .account(&old_account);
        let _ = old_delete.delete();
    }
}

#[cfg(target_os = "windows")]
fn round_corners(window: &tauri::WebviewWindow) {
    use windows::Win32::Graphics::Dwm::{
        DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_ROUND, DwmSetWindowAttribute,
    };

    let Ok(hwnd) = window.hwnd() else { return };
    let preference = DWMWCP_ROUND;
    // Windows 11 only; older versions reject the attribute, which is fine.
    let _ = unsafe {
        DwmSetWindowAttribute(
            hwnd,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            (&raw const preference).cast(),
            size_of_val(&preference) as u32,
        )
    };
}

#[cfg(target_os = "macos")]
fn titlebar_color(theme: tauri::Theme) -> tauri::window::Color {
    match theme {
        tauri::Theme::Dark => tauri::window::Color(29, 31, 31, 255),
        _ => tauri::window::Color(247, 245, 243, 255),
    }
}

fn is_allowed_url(url: &tauri::Url) -> bool {
    if matches!(url.scheme(), "blob" | "data" | "about") {
        return true;
    }

    if url.scheme() != "https" {
        return false;
    }

    url.host_str().is_some_and(|host| {
        host == "whatsapp.com"
            || host.ends_with(".whatsapp.com")
            || host == "whatsapp.net"
            || host.ends_with(".whatsapp.net")
    })
}

fn open_external(app: &AppHandle, url: &str) {
    if let Err(error) = app.opener().open_url(url, None::<&str>) {
        eprintln!("failed to open {url} in the system browser: {error}");
    }
}

fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

fn toggle_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) && !window.is_minimized().unwrap_or(false) {
            let _ = window.hide();
        } else {
            show_main_window(app);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::is_allowed_url;

    #[test]
    fn allows_only_https_whatsapp_and_internal_document_urls() {
        let cases = [
            ("https://web.whatsapp.com/", true),
            ("https://static.whatsapp.net/x", true),
            ("http://web.whatsapp.com/", false),
            ("mailto:a@b.c", false),
            ("https://evil-whatsapp.com/", false),
            ("https://example.com", false),
            ("blob:https://web.whatsapp.com/uuid", true),
            ("about:blank", true),
        ];

        for (url, expected) in cases {
            assert_eq!(
                is_allowed_url(&tauri::Url::parse(url).unwrap()),
                expected,
                "{url}"
            );
        }
    }
}
