use std::{fs, path::PathBuf, sync::Mutex};

use serde::{Deserialize, Serialize};
#[cfg(target_os = "linux")]
use tauri::Manager;
use tauri::{AppHandle, State, command};
#[cfg(not(target_os = "linux"))]
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_notification::NotificationExt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub start_on_login: bool,
    pub close_to_tray: bool,
    pub notifications_enabled: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            start_on_login: false,
            close_to_tray: true,
            notifications_enabled: true,
        }
    }
}

pub struct SettingsStore {
    path: PathBuf,
    current: Mutex<Settings>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationOptions {
    id: Option<i32>,
    title: Option<String>,
    body: Option<String>,
    icon: Option<String>,
    sound: Option<String>,
    action_type_id: Option<String>,
}

impl SettingsStore {
    pub fn load(path: PathBuf) -> Self {
        let current = fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .unwrap_or_default();

        Self {
            path,
            current: Mutex::new(current),
        }
    }

    pub fn get(&self) -> Settings {
        *self.current.lock().expect("settings lock poisoned")
    }

    pub fn save(&self, settings: &Settings) -> std::io::Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let temporary_path = self.path.with_file_name("settings.json.tmp");
        fs::write(&temporary_path, serde_json::to_vec_pretty(settings)?)?;
        fs::rename(temporary_path, &self.path)
    }

    fn with_notifications(&self, show: impl FnOnce() -> Result<(), String>) -> Result<(), String> {
        let current = self.current.lock().map_err(|error| error.to_string())?;
        if !current.notifications_enabled {
            return Ok(());
        }
        show()
    }
}

#[command]
pub fn get_settings(store: State<'_, SettingsStore>) -> Settings {
    store.get()
}

#[command]
pub fn set_settings(
    app: AppHandle,
    store: State<'_, SettingsStore>,
    settings: Settings,
) -> Result<Settings, String> {
    let mut current = store
        .current
        .lock()
        .map_err(|error| format!("settings lock poisoned: {error}"))?;
    let previous = *current;

    if settings.start_on_login != previous.start_on_login {
        apply_autostart(&app, settings.start_on_login)?;
    }

    if let Err(error) = store.save(&settings) {
        if settings.start_on_login != previous.start_on_login {
            let _ = apply_autostart(&app, previous.start_on_login);
        }
        return Err(format!("failed to save settings: {error}"));
    }

    *current = settings;
    Ok(settings)
}

#[command]
pub fn show_notification(
    app: AppHandle,
    store: State<'_, SettingsStore>,
    options: NotificationOptions,
) -> Result<(), String> {
    store.with_notifications(|| {
        let mut notification = app.notification().builder();
        if let Some(id) = options.id {
            notification = notification.id(id);
        }
        if let Some(title) = options.title {
            notification = notification.title(title);
        }
        if let Some(body) = options.body {
            notification = notification.body(body);
        }
        if let Some(icon) = options.icon {
            notification = notification.icon(icon);
        }
        if let Some(sound) = options.sound {
            notification = notification.sound(sound);
        }
        if let Some(action_type_id) = options.action_type_id {
            notification = notification.action_type_id(action_type_id);
        }
        notification.show().map_err(|error| error.to_string())
    })
}

#[cfg(not(target_os = "linux"))]
pub fn apply_autostart(app: &AppHandle, enabled: bool) -> Result<(), String> {
    if enabled {
        app.autolaunch().enable().map_err(|error| error.to_string())
    } else {
        app.autolaunch()
            .disable()
            .map_err(|error| error.to_string())
    }
}

#[cfg(target_os = "linux")]
pub fn apply_autostart(app: &AppHandle, enabled: bool) -> Result<(), String> {
    let filename = format!("{}.desktop", app.package_info().name);
    let entry = app
        .path()
        .config_dir()
        .map_err(|e| e.to_string())?
        .join("autostart")
        .join(&filename);
    let legacy_entry = app
        .path()
        .home_dir()
        .map_err(|e| e.to_string())?
        .join(".config/autostart")
        .join(filename);
    let executable = match app.env().appimage {
        Some(path) => PathBuf::from(path),
        None => std::env::current_exe().map_err(|e| e.to_string())?,
    };
    write_linux_autostart(&entry, &executable, enabled).map_err(|e| e.to_string())?;
    if legacy_entry != entry {
        write_linux_autostart(&legacy_entry, &executable, false).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn linux_exec(executable: &std::path::Path) -> std::io::Result<String> {
    let executable = executable.to_str().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "unsupported autostart executable path",
        )
    })?;
    let mut quoted = String::from("\"");
    for character in executable.chars() {
        if matches!(character, '\\' | '"' | '`' | '$') {
            quoted.push('\\');
        }
        if character == '%' {
            quoted.push('%');
        }
        quoted.push(character);
    }
    quoted.push('"');
    // Desktop string escaping is undone before Exec argument unquoting.
    let argument = quoted
        .replace('\\', "\\\\")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");
    // Keep literal percent signs out of the launcher's executable-path validation.
    Ok(format!("/usr/bin/env -- {argument}"))
}

#[cfg(target_os = "linux")]
fn write_linux_autostart(
    entry: &std::path::Path,
    executable: &std::path::Path,
    enabled: bool,
) -> std::io::Result<()> {
    if !enabled {
        return match fs::remove_file(entry) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            result => result,
        };
    }
    let exec = linux_exec(executable)?;
    if let Some(parent) = entry.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = entry.with_extension("desktop.tmp");
    fs::write(
        &temporary,
        format!(
            "[Desktop Entry]\nType=Application\nVersion=1.0\nName=WhatheRust\nExec={exec}\nTerminal=false\nStartupNotify=false\n"
        ),
    )?;
    fs::rename(temporary, entry)
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::{Settings, SettingsStore};

    fn test_dir() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock before Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "whatherust-settings-{}-{unique}",
            std::process::id()
        ))
    }

    #[test]
    fn missing_file_uses_defaults() {
        let dir = test_dir();
        let store = SettingsStore::load(dir.join("settings.json"));
        assert_eq!(store.get(), Settings::default());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn corrupt_file_uses_defaults() {
        let dir = test_dir();
        fs::create_dir_all(&dir).expect("create test directory");
        fs::write(dir.join("settings.json"), b"{not json").expect("write corrupt settings");

        let store = SettingsStore::load(dir.join("settings.json"));
        assert_eq!(store.get(), Settings::default());
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn partial_json_keeps_defaults_for_missing_fields() {
        let dir = test_dir();
        fs::create_dir_all(&dir).expect("create test directory");
        fs::write(dir.join("settings.json"), br#"{"close_to_tray": false}"#)
            .expect("write partial settings");

        let store = SettingsStore::load(dir.join("settings.json"));
        assert_eq!(
            store.get(),
            Settings {
                close_to_tray: false,
                ..Settings::default()
            }
        );
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn save_load_roundtrip() {
        let dir = test_dir();
        let path = dir.join("settings.json");
        let settings = Settings {
            start_on_login: true,
            close_to_tray: false,
            notifications_enabled: false,
        };
        SettingsStore::load(path.clone())
            .save(&settings)
            .expect("save settings");

        assert_eq!(SettingsStore::load(path).get(), settings);
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn native_notification_delivery_uses_current_settings_for_every_request() {
        let store = SettingsStore::load(test_dir().join("settings.json"));
        let delivered = std::cell::Cell::new(0);
        let deliver = || {
            delivered.set(delivered.get() + 1);
            Ok(())
        };
        store.with_notifications(deliver).unwrap();
        store.current.lock().unwrap().notifications_enabled = false;
        store.with_notifications(deliver).unwrap();
        assert_eq!(delivered.get(), 1);
        store.current.lock().unwrap().notifications_enabled = true;
        store.with_notifications(deliver).unwrap();
        assert_eq!(delivered.get(), 2);
    }

    #[test]
    fn native_notification_delivery_keeps_settings_locked_until_submission() {
        let store = SettingsStore::load(test_dir().join("settings.json"));
        store
            .with_notifications(|| {
                assert!(store.current.try_lock().is_err());
                Ok(())
            })
            .unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_autostart_parses_spaced_appimage_paths_as_one_executable() {
        use std::os::unix::fs::PermissionsExt;

        let dir = test_dir();
        let executable = dir.join("My Apps/WhatheRust.AppImage");
        fs::create_dir_all(executable.parent().unwrap()).unwrap();
        fs::write(&executable, "#!/bin/sh\nexit 0\n").unwrap();
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
        let entry = dir.join("autostart/whatherust.desktop");
        super::write_linux_autostart(&entry, &executable, true).unwrap();
        let key_file = gio::glib::KeyFile::new();
        key_file
            .load_from_file(&entry, gio::glib::KeyFileFlags::NONE)
            .unwrap();
        let command = key_file.string("Desktop Entry", "Exec").unwrap();
        let arguments = gio::glib::shell_parse_argv(command.as_str()).unwrap();
        assert_eq!(
            arguments,
            [
                std::ffi::OsString::from("/usr/bin/env"),
                std::ffi::OsString::from("--"),
                executable.as_os_str().to_owned()
            ]
        );
        gio::DesktopAppInfo::from_filename(&entry).expect("valid desktop entry");
        assert!(!entry.with_extension("desktop.tmp").exists());
        super::write_linux_autostart(&entry, &executable, false).unwrap();
        super::write_linux_autostart(&entry, &executable, false).unwrap();
        assert!(!entry.exists());
        fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_autostart_launches_paths_with_desktop_exec_metacharacters() {
        use gio::prelude::AppInfoExt;
        use std::os::unix::fs::PermissionsExt;

        let dir = test_dir();
        fs::create_dir_all(&dir).unwrap();
        let executable = dir.join("WhatheRust $quoted`name\"with\\slash%f.AppImage");
        fs::write(&executable, "#!/bin/sh\nprintf started > \"$0.started\"\n").unwrap();
        fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
        let entry = dir.join("whatherust.desktop");
        super::write_linux_autostart(&entry, &executable, true).unwrap();
        let info = gio::DesktopAppInfo::from_filename(&entry).expect("valid desktop entry");
        info.launch(&[], None::<&gio::AppLaunchContext>)
            .expect("launch exact executable");
        let marker = PathBuf::from(format!("{}.started", executable.display()));
        for _ in 0..100 {
            if marker.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        assert_eq!(fs::read_to_string(&marker).unwrap(), "started");
        fs::remove_dir_all(dir).unwrap();
    }
}
