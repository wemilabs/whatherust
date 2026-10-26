use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

use tauri::{
    AppHandle, Manager, WebviewWindow, WebviewWindowBuilder,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::NewWindowResponse,
};
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogResult};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_updater::UpdaterExt;

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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
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

            if !cfg!(debug_assertions) {
                tauri::async_runtime::spawn(check_for_update(app.handle().clone()));
            }

            let data_dir = app.path().app_local_data_dir()?;
            move_legacy(&data_dir.with_file_name(LEGACY_IDENTIFIER), &data_dir);

            #[cfg(target_os = "macos")]
            let migration_pending = !migrate_legacy_webkit_data(
                &data_dir,
                &app.path().home_dir()?.join("Library"),
                &app.config().identifier,
                &app.package_info().name,
            );

            let builder = WebviewWindowBuilder::from_config(app.handle(), window_config)?
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
                .on_document_title_changed({
                    let shown_count = AtomicU32::new(0);
                    move |window, title| {
                        if let Some(tray) = window.app_handle().tray_by_id("main") {
                            let _ = tray.set_tooltip(Some(title.replace("WhatsApp", "WhatheRust")));
                        }
                        let count = unread_count(&title);
                        if shown_count.swap(count, Ordering::Relaxed) != count {
                            show_unread_badge(&window, count);
                        }
                    }
                });
            // Invariant: any launch without the migration marker is incognito, so wiping the leftover new store never destroys a real session.
            #[cfg(target_os = "macos")]
            let builder = builder.incognito(migration_pending);
            let window = builder.build()?;

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

async fn check_for_update(app: AppHandle) {
    async fn check(app: &AppHandle) -> tauri_plugin_updater::Result<()> {
        let Some(update) = app.updater()?.check().await? else {
            return Ok(());
        };
        let (sender, receiver) = tokio::sync::oneshot::channel();

        app.dialog()
            .message(format!(
                "WhatheRust {} is available. Install it now? The app will restart.",
                update.version
            ))
            .title("WhatheRust update")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Install".into(),
                "Later".into(),
            ))
            .show_with_result(move |result| {
                let _ = sender.send(result);
            });

        if matches!(
            receiver.await,
            Ok(MessageDialogResult::Custom(button)) if button == "Install"
        ) {
            update.download_and_install(|_, _| {}, || {}).await?;
            app.restart();
        }

        Ok(())
    }

    let _ = check(&app).await;
}

/// Moves data left by builds made before the app identifier was renamed.
/// Moved, not copied, on purpose: the same WhatsApp linked-device keys must never live in two apps.
fn move_legacy(from: &Path, to: &Path) -> bool {
    from.exists() && !to.exists() && std::fs::rename(from, to).is_ok()
}

#[cfg(target_os = "macos")]
fn migrate_legacy_webkit_data(
    data_dir: &Path,
    library: &Path,
    identifier: &str,
    app_name: &str,
) -> bool {
    // Once set, a legacy folder recreated by the old app must never replace this app's session.
    let marker = data_dir.join(".legacy-migrated");
    let mark_migrated = || {
        let _ = std::fs::create_dir_all(data_dir);
        let _ = std::fs::File::create(&marker);
    };
    if marker.exists() {
        return true;
    }
    let webkit = library.join("WebKit");
    let cookies = library.join("HTTPStorages");
    let legacy = webkit.join(LEGACY_IDENTIFIER);
    if !legacy.exists() {
        mark_migrated();
        return true;
    }
    // Copy the key first; discard leftover new stores only after success so denied prompts retry next launch.
    if !copy_webcrypto_master_key(identifier, app_name) {
        return false;
    }
    let current = webkit.join(identifier);
    let _ = std::fs::remove_dir_all(&current);
    let _ = std::fs::remove_file(cookies.join(format!("{identifier}.binarycookies")));
    let _ = std::fs::remove_dir_all(cookies.join(identifier));
    if !move_legacy(&legacy, &current) {
        return false;
    }
    mark_migrated();

    move_legacy(
        &cookies.join(format!("{LEGACY_IDENTIFIER}.binarycookies")),
        &cookies.join(format!("{identifier}.binarycookies")),
    );
    move_legacy(&cookies.join(LEGACY_IDENTIFIER), &cookies.join(identifier));
    delete_legacy_webcrypto_master_key();
    true
}

/// WebKit wraps stored CryptoKeys with this per-bundle-id item; moved IndexedDB keys need it to be unwrapped.
#[cfg(target_os = "macos")]
fn copy_webcrypto_master_key(identifier: &str, app_name: &str) -> bool {
    use core_foundation::data::CFData;
    use security_framework::item::{
        ItemAddOptions, ItemAddValue, ItemClass, ItemSearchOptions, Location, SearchResult,
    };

    const ERR_SEC_ITEM_NOT_FOUND: i32 = -25300;

    let old_account = legacy_webcrypto_account();
    let mut old_search = ItemSearchOptions::new();
    old_search
        .class(ItemClass::generic_password())
        .account(&old_account)
        .load_data(true)
        .limit(1);
    let mut old_items = match old_search.search() {
        Ok(items) => items,
        Err(error) if error.code() == ERR_SEC_ITEM_NOT_FOUND => return true,
        Err(_) => return false,
    };
    let data = match old_items.pop() {
        None => return true,
        Some(SearchResult::Data(data)) => data,
        Some(_) => return false,
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
    new_item.add().is_ok()
}

#[cfg(target_os = "macos")]
fn legacy_webcrypto_account() -> String {
    format!("com.apple.WebKit.WebCrypto.master+{LEGACY_IDENTIFIER}")
}

/// Best effort: another signer's item can refuse deletion (errSecInvalidOwnerEdit); the keys it wraps moved with the folder.
#[cfg(target_os = "macos")]
fn delete_legacy_webcrypto_master_key() {
    use security_framework::item::{ItemClass, ItemSearchOptions};

    let mut old_delete = ItemSearchOptions::new();
    old_delete
        .class(ItemClass::generic_password())
        .account(&legacy_webcrypto_account());
    let _ = old_delete.delete();
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

/// WhatsApp Web prefixes its title with the unread chat count, e.g. `(3) WhatsApp`.
fn unread_count(title: &str) -> u32 {
    title
        .trim_start()
        .strip_prefix('(')
        .and_then(|rest| rest.split_once(')'))
        .and_then(|(count, _)| count.trim().trim_end_matches('+').parse().ok())
        .unwrap_or(0)
}

/// Shows `count` on the dock (macOS), launcher (Linux), taskbar overlay (Windows) and tray icon; 0 clears it.
fn show_unread_badge(window: &WebviewWindow, count: u32) {
    #[cfg(not(target_os = "windows"))]
    let _ = window.set_badge_count((count > 0).then_some(i64::from(count)));

    #[cfg(target_os = "windows")]
    let _ = window.set_overlay_icon((count > 0).then(|| {
        const SIZE: u32 = 32;
        let mut rgba = vec![0; (SIZE * SIZE * 4) as usize];
        draw_badge(&mut rgba, SIZE, 0, 0, SIZE, &badge_label(count));
        tauri::image::Image::new_owned(rgba, SIZE, SIZE)
    }));

    // The macOS menu bar icon is a monochrome template, so the Dock badge carries the count there.
    #[cfg(not(target_os = "macos"))]
    if let (Some(tray), Some(icon)) = (
        window.app_handle().tray_by_id("main"),
        window.app_handle().default_window_icon(),
    ) {
        let icon = if count == 0 {
            icon.clone()
        } else {
            let (width, height) = (icon.width(), icon.height());
            let size = width.min(height) * 5 / 8;
            let mut rgba = icon.rgba().to_vec();
            draw_badge(&mut rgba, width, width - size, 0, size, &badge_label(count));
            tauri::image::Image::new_owned(rgba, width, height)
        };
        let _ = tray.set_icon(Some(icon));
    }
}

#[cfg_attr(target_os = "macos", allow(dead_code))]
fn badge_label(count: u32) -> String {
    if count > 9 {
        "9+".into()
    } else {
        count.to_string()
    }
}

/// Paints a red badge with white `label` text into the `size` px square at `left`, `top` of a straight-alpha RGBA buffer.
#[cfg_attr(target_os = "macos", allow(dead_code))]
fn draw_badge(rgba: &mut [u8], width: u32, left: u32, top: u32, size: u32, label: &str) {
    const RED: [u8; 3] = [0xE5, 0x39, 0x35];
    // 3x5 pixel glyphs, one row per byte, most significant of the 3 bits on the left.
    const fn glyph(c: char) -> [u8; 5] {
        match c {
            '0' => [7, 5, 5, 5, 7],
            '1' => [2, 6, 2, 2, 7],
            '2' => [7, 1, 7, 4, 7],
            '3' => [7, 1, 7, 1, 7],
            '4' => [5, 5, 7, 1, 1],
            '5' => [7, 4, 7, 1, 7],
            '6' => [7, 4, 7, 5, 7],
            '7' => [7, 1, 1, 1, 1],
            '8' => [7, 5, 7, 5, 7],
            '9' => [7, 5, 7, 1, 7],
            '+' => [0, 2, 7, 2, 0],
            _ => [0; 5],
        }
    }

    let mut blend = |x: u32, y: u32, color: [u8; 3], alpha: f32| {
        let i = ((y * width + x) * 4) as usize;
        let Some(pixel) = rgba.get_mut(i..i + 4) else {
            return;
        };
        let below = f32::from(pixel[3]) / 255.0 * (1.0 - alpha);
        let out = alpha + below;
        if out > 0.0 {
            for (channel, &top) in pixel.iter_mut().zip(&color) {
                *channel =
                    ((f32::from(top) * alpha + f32::from(*channel) * below) / out).round() as u8;
            }
        }
        pixel[3] = (out * 255.0).round() as u8;
    };

    let radius = size as f32 / 2.0;
    for y in 0..size {
        for x in 0..size {
            let distance = (x as f32 + 0.5 - radius).hypot(y as f32 + 0.5 - radius);
            let coverage = (radius - distance + 0.5).clamp(0.0, 1.0);
            if coverage > 0.0 {
                blend(left + x, top + y, RED, coverage);
            }
        }
    }

    let columns = (label.chars().count() * 4).saturating_sub(1).max(1) as u32;
    let scale = (size * 13 / 20 / columns).min(size * 11 / 20 / 5).max(1);
    let text_left = left + size.saturating_sub(columns * scale) / 2;
    let text_top = top + size.saturating_sub(5 * scale) / 2;
    for (index, c) in label.chars().enumerate() {
        for (row, bits) in glyph(c).into_iter().enumerate() {
            for column in 0..3 {
                if bits & (4 >> column) == 0 {
                    continue;
                }
                let x = text_left + (index as u32 * 4 + column) * scale;
                let y = text_top + row as u32 * scale;
                for dy in 0..scale {
                    for dx in 0..scale {
                        blend(x + dx, y + dy, [255; 3], 1.0);
                    }
                }
            }
        }
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
    use super::{badge_label, draw_badge, is_allowed_url, unread_count};

    #[test]
    fn parses_the_unread_count_prefix() {
        let cases = [
            ("(3) WhatsApp", 3),
            ("(128) WhatsApp", 128),
            (" (1) WhatsApp", 1),
            ("(99+) WhatsApp", 99),
            ("WhatsApp", 0),
            ("(0) WhatsApp", 0),
            ("WhatsApp (3)", 0),
            ("(abc) WhatsApp", 0),
            ("(3 WhatsApp", 0),
            ("", 0),
        ];

        for (title, expected) in cases {
            assert_eq!(unread_count(title), expected, "{title}");
        }
    }

    #[test]
    fn caps_badge_labels_at_nine_plus() {
        assert_eq!(badge_label(1), "1");
        assert_eq!(badge_label(9), "9");
        assert_eq!(badge_label(10), "9+");
    }

    #[test]
    fn draws_a_red_badge_with_white_text_inside_its_square() {
        const WIDTH: u32 = 32;
        let mut rgba = vec![0; (WIDTH * WIDTH * 4) as usize];
        draw_badge(&mut rgba, WIDTH, 12, 0, 20, "8");
        let pixel = |x: u32, y: u32| {
            let i = ((y * WIDTH + x) * 4) as usize;
            [rgba[i], rgba[i + 1], rgba[i + 2], rgba[i + 3]]
        };

        assert_eq!(pixel(0, 0), [0; 4], "outside the badge square");
        assert_eq!(pixel(12, 0)[3], 0, "outside the circle in its corner");
        assert_eq!(pixel(22, 10), [255, 255, 255, 255], "centre stroke of 8");
        assert_eq!(pixel(14, 10), [0xE5, 0x39, 0x35, 255], "badge fill");
        assert_eq!(pixel(22, 25), [0; 4], "below the badge");
    }

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
