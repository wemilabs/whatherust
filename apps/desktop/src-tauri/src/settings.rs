use std::{fs, path::PathBuf, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State, command};
use tauri_plugin_autostart::ManagerExt;

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
    let notifications_changed = settings.notifications_enabled != previous.notifications_enabled;

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
    if notifications_changed && let Some(window) = app.get_webview_window("main") {
        let _ = window.eval("window.__whatherustSyncNotifications?.()");
    }
    Ok(settings)
}

#[command]
pub fn notifications_enabled(store: State<'_, SettingsStore>) -> bool {
    store.get().notifications_enabled
}

pub fn apply_autostart(app: &AppHandle, enabled: bool) -> Result<(), String> {
    if enabled {
        app.autolaunch().enable().map_err(|error| error.to_string())
    } else {
        app.autolaunch()
            .disable()
            .map_err(|error| error.to_string())
    }
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
}
