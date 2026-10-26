fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "get_settings",
            "set_settings",
            "notifications_enabled",
        ]),
    ))
    .expect("failed to build Tauri application");
}
