mod binaries;
mod util;
mod downloader;

use tauri::{command, AppHandle, Manager};
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

#[command]
async fn download_youtube(url: String, app: AppHandle) -> String {
    downloader::download_audio(url, app, |u| u.contains("youtube.com") || u.contains("youtu.be")).await
}

#[command]
async fn download_soundcloud(url: String, app: AppHandle) -> String {
    downloader::download_audio(url, app, |u| u.contains("soundcloud.com")).await
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![download_youtube, download_soundcloud])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::UnderWindowBackground, None, None)
                .expect("macOS vibrancy 실패");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125))).expect("Windows blur 실패");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
