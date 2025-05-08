use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tokio::process::Command;

use crate::{
    binaries::{ffmpeg_path, yt_dlp_path},
    util::sanitize_filename,
};

/// YouTube·SoundCloud 공통 로직
pub async fn download_audio<F>(url: String, app: AppHandle, is_valid: F) -> String
where
    F: Fn(&str) -> bool,
{
    if !is_valid(&url) {
        return "invalid URL".into();
    }

    /* 1. yt-dlp / ffmpeg 경로 확보 (최초 1회만 다운로드) */
    let yt_dlp = match yt_dlp_path().await {
        Ok(p) => p.clone(),
        Err(e) => return format!("yt-dlp error: {e}"),
    };
    let ffmpeg = ffmpeg_path().await.ok().flatten();

    /* 2. 동영상 제목 추출 */
    let title_res = Command::new(&yt_dlp)
        .args(["--print", "title", &url])
        .output()
        .await;
    let title = match title_res {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).trim().to_owned(),
        Ok(out) => return format!("yt-dlp title error: {}", String::from_utf8_lossy(&out.stderr)),
        Err(e)  => return format!("yt-dlp exec error: {e}"),
    };

    /* 3. 출력 폴더 선택 */
    let default_dir = app.path().download_dir().unwrap_or_else(|_| PathBuf::from("~/Downloads"));
    let folder = app.dialog().file().set_directory(default_dir).blocking_pick_folder();
    let Some(folder) = folder else { return "canceled".into() };
    let folder = folder.into_path().map_err(|e| e.to_string()).unwrap();

    /* 4. yt-dlp 실행 */
    let out_file   = folder.join(format!("{}.mp3", sanitize_filename(&title)));
    let mut cmd    = Command::new(&yt_dlp);
    cmd.args(["-x", "--audio-format", "mp3", "-o"])
        .arg(out_file.to_str().unwrap())
        .arg(&url);

    if let Some(ff) = ffmpeg {
        cmd.args(["--ffmpeg-location", ff.to_str().unwrap()]);
    }

    match cmd.output().await {
        Ok(out) if out.status.success() => format!("Ok: path={}", out_file.display()),
        Ok(out) => format!("yt-dlp failed: {}", String::from_utf8_lossy(&out.stderr)),
        Err(e)  => format!("yt-dlp exec error: {e}"),
    }
}
