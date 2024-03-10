use tauri::{Manager, Runtime};
use window_vibrancy::{
    apply_acrylic, apply_blur, apply_mica, apply_vibrancy, NSVisualEffectMaterial,
};

// pub fn set_window_shadow<R: Runtime>(app: &tauri::App<R>) {
//     // let window = app.get_window("main").unwrap();
//     // app.win
//     // set_shadow(&window, true).expect("Unsupported platform!");
//
//     // let window = app.get_window("dev").unwrap();
//     // set_shadow(&window, true).expect("Unsupported platform!");
// }

pub fn set_window_vibrancy<R: Runtime>(app: &tauri::App<R>) {
    let window = app.get_webview_window("main").unwrap();
    #[cfg(target_os = "macos")]
    apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    #[cfg(target_os = "windows")]
    // apply_acrylic(&window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");
    apply_mica(&window, Some(true))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    // let window = app.get_window("dev").unwrap();
    // set_shadow(&window, true).expect("Unsupported platform!");
}
