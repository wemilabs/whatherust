use std::path::Path;
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};
use std::time::Duration;

use tauri::{
    AppHandle, Manager, WebviewWindowBuilder,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::NewWindowResponse,
};
use tauri_plugin_dialog::{
    DialogExt, MessageDialogButtons, MessageDialogKind, MessageDialogResult,
};
use tauri_plugin_opener::OpenerExt;
use tauri_plugin_updater::UpdaterExt;

const LEGACY_IDENTIFIER: &str = "com.wemilabs.whatsapp-desktop-rs";
const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(6 * 60 * 60);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum UpdateCheck {
    Automatic,
    Manual,
}

#[derive(Default)]
struct UpdateState {
    busy: AtomicBool,
    manual_requested: AtomicBool,
    dismissed: Mutex<Option<String>>,
}

impl UpdateState {
    fn effective_kind(&self, kind: UpdateCheck) -> UpdateCheck {
        if kind == UpdateCheck::Manual || self.manual_requested.load(Ordering::Acquire) {
            UpdateCheck::Manual
        } else {
            UpdateCheck::Automatic
        }
    }
}

struct UpdateBusyGuard<'a>(&'a UpdateState);

impl Drop for UpdateBusyGuard<'_> {
    fn drop(&mut self) {
        self.0.manual_requested.store(false, Ordering::Release);
        self.0.busy.store(false, Ordering::Release);
    }
}

fn should_prompt(kind: UpdateCheck, version: &str, dismissed: Option<&str>) -> bool {
    kind == UpdateCheck::Manual || dismissed != Some(version)
}

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
            app.manage(UpdateState::default());
            let window_config = app
                .config()
                .app
                .windows
                .iter()
                .find(|config| config.label == "main")
                .ok_or("missing main window configuration")?;
            let app_handle = app.handle().clone();

            if !cfg!(debug_assertions) {
                let update_handle = app_handle.clone();
                tauri::async_runtime::spawn(async move {
                    loop {
                        check_for_update(update_handle.clone(), UpdateCheck::Automatic).await;
                        tokio::time::sleep(UPDATE_CHECK_INTERVAL).await;
                    }
                });
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
                .on_document_title_changed(|webview, title| {
                    if let Some(tray) = webview.app_handle().tray_by_id("main") {
                        let _ = tray.set_tooltip(Some(title.replace("WhatsApp", "WhatheRust")));
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
            let check_updates_item = MenuItem::with_id(
                app,
                "check_updates",
                "Check for updates…",
                true,
                None::<&str>,
            )?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &check_updates_item, &quit_item])?;

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
                    "check_updates" => {
                        tauri::async_runtime::spawn(check_for_update(
                            app.clone(),
                            UpdateCheck::Manual,
                        ));
                    }
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

async fn check_for_update(app: AppHandle, kind: UpdateCheck) {
    let state = app.state::<UpdateState>();
    if state.busy.swap(true, Ordering::AcqRel) {
        if kind == UpdateCheck::Manual {
            state.manual_requested.store(true, Ordering::Release);
        }
        return;
    }
    let _busy = UpdateBusyGuard(&state);

    async fn check(
        app: &AppHandle,
        state: &UpdateState,
        kind: UpdateCheck,
    ) -> tauri_plugin_updater::Result<()> {
        let Some(update) = app.updater()?.check().await? else {
            if state.effective_kind(kind) == UpdateCheck::Manual {
                app.dialog()
                    .message(format!(
                        "WhatheRust {} is the latest version.",
                        app.package_info().version
                    ))
                    .title("WhatheRust update")
                    .kind(MessageDialogKind::Info)
                    .show(|_| {});
            }
            return Ok(());
        };
        let version = update.version.clone();
        if !should_prompt(
            state.effective_kind(kind),
            &version,
            state.dismissed.lock().unwrap().as_deref(),
        ) {
            return Ok(());
        }

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
        } else {
            *state.dismissed.lock().unwrap() = Some(version);
        }

        Ok(())
    }

    match check(&app, &state, kind).await {
        Err(error) if state.effective_kind(kind) == UpdateCheck::Manual => {
            app.dialog()
                .message(format!("Could not check for updates: {error}"))
                .title("WhatheRust update")
                .kind(MessageDialogKind::Error)
                .show(|_| {});
        }
        _ => {}
    }
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
    use std::sync::atomic::Ordering;

    use super::{UpdateCheck, UpdateState, is_allowed_url, should_prompt};

    #[test]
    fn should_prompt_respects_dismissed_version_for_automatic_checks() {
        assert!(!should_prompt(
            UpdateCheck::Automatic,
            "2.0.0",
            Some("2.0.0")
        ));
        assert!(should_prompt(
            UpdateCheck::Automatic,
            "2.0.0",
            Some("1.9.0")
        ));
        assert!(should_prompt(UpdateCheck::Automatic, "2.0.0", None));
    }

    #[test]
    fn should_prompt_always_prompts_for_manual_checks() {
        assert!(should_prompt(UpdateCheck::Manual, "2.0.0", Some("2.0.0")));
    }

    #[test]
    fn effective_kind_respects_manual_requests() {
        let state = UpdateState::default();

        assert_eq!(
            state.effective_kind(UpdateCheck::Automatic),
            UpdateCheck::Automatic
        );

        state.manual_requested.store(true, Ordering::Release);

        assert_eq!(
            state.effective_kind(UpdateCheck::Automatic),
            UpdateCheck::Manual
        );
        assert_eq!(
            state.effective_kind(UpdateCheck::Manual),
            UpdateCheck::Manual
        );
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
