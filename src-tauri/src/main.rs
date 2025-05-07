use std::{env, fs::File, io::{self, Cursor}, path::PathBuf};
use tauri::{command, AppHandle, Manager};
use tauri_plugin_dialog::DialogExt;
use reqwest;
use zip::ZipArchive;
use tokio::process::Command;
use window_vibrancy::{apply_vibrancy, apply_blur, NSVisualEffectMaterial};


fn sanitize_filename(title: &str) -> String {
    title
        .replace(|c: char| !c.is_ascii_alphanumeric() && c != ' ' && c != '-' && c != '_', "")
}

#[command]
async fn download_youtube(url: String, app: AppHandle) -> String {
    use std::path::{Path, PathBuf};

    eprintln!("[1] Start download_mp3");

    if !(url.contains("youtube.com") || url.contains("youtu.be")) {
        eprintln!("[Error] Invalid YouTube URL: {}", url);
        return "invalid URL".into();
    }

    let yt_dlp_path = match ensure_yt_dlp_exists().await {
        Ok(p) => {
            eprintln!("[2] yt-dlp path resolved: {:?}", p);
            p
        }
        Err(e) => {
            eprintln!("[Error] Failed to ensure yt-dlp: {}", e);
            return format!("Error ensuring yt-dlp: {}", e);
        }
    };

    let ffmpeg_path = ensure_ffmpeg_exists().await.unwrap_or(None);

    let title_output = Command::new(&yt_dlp_path)
        .args(["--print", "title", &url])
        .output()
        .await;

    let video_title = match title_output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            eprintln!("[3] yt-dlp title: {}", stdout);
            stdout
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return format!("Error: yt-dlp failed to get title: {}", stderr);
        }
        Err(e) => return format!("Error: Failed to execute yt-dlp: {}", e),
    };

    let sanitized_title = sanitize_filename(&video_title);
    eprintln!("[4] Sanitized title: {}", sanitized_title);

    let default_dir = app
        .path()
        .download_dir()
        .unwrap_or_else(|_| PathBuf::from("~/Downloads"));

    let folder = app
        .dialog()
        .file()
        .set_directory(default_dir)
        .blocking_pick_folder(); // 폴더만 선택

    let Some(folder_path) = folder else {
        eprintln!("[Info] Folder pick canceled");
        return "canceled".into();
    };

    let folder_path = match folder_path.into_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[Error] Invalid folder path: {e}");
            return format!("invalid folder path: {e}");
        }
    };
    eprintln!("[5] User-selected folder: {}", folder_path.display());

    let output_path = folder_path.join(format!("{}.mp3", sanitize_filename(&video_title)));
    eprintln!("[6] Output path: {:?}", output_path);

    let mut cmd = Command::new(&yt_dlp_path);
    cmd.args(["-x", "--audio-format", "mp3", "-o"])
        .arg(output_path.to_str().unwrap())
        .arg(&url);

    if let Some(ref ffmpeg) = ffmpeg_path {
        if let Some(p) = ffmpeg.to_str() {
            cmd.args(["--ffmpeg-location", p]);
        }
    }


    let output = cmd.output().await;

    if let Ok(ref r) = output {
        // 디버그용 로그 저장 (선택)
        std::fs::write("/tmp/yt-dlp-stdout.txt", &r.stdout).ok();
        std::fs::write("/tmp/yt-dlp-stderr.txt", &r.stderr).ok();
    }

    match output {
        Ok(res) if res.status.success() => {
            eprintln!("[7] Download completed successfully");
            format!("Ok: path={}", output_path.display())
        }
        Ok(res) => {
            let stderr = String::from_utf8_lossy(&res.stderr);
            format!("Error: yt-dlp failed: {}", stderr)
        }
        Err(e) => format!("Error: Failed to run yt-dlp: {}", e),
    }
}


async fn ensure_yt_dlp_exists() -> io::Result<PathBuf> {
    let app_dir = match env::current_exe()?.parent() {
        Some(dir) => dir.to_path_buf(),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to determine app directory",
            ))
        }
    };

    let yt_dlp_path = app_dir.join("yt-dlp");


    if !yt_dlp_path.exists() {
        eprintln!("[ensure] yt-dlp not found, downloading...");

        let download_status = Command::new("curl")
            .args([
                "-L",
                "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
                "-o",
                yt_dlp_path.to_str().unwrap(),
            ])
            .status()
            .await;

        if let Err(e) = download_status {
            return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to run curl: {}", e)));
        }

        if !download_status.unwrap().success() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to download yt-dlp"));
        }

        let chmod_status = Command::new("chmod")
            .args(["+x", yt_dlp_path.to_str().unwrap()])
            .status()
            .await;

        if let Err(e) = chmod_status {
            return Err(io::Error::new(io::ErrorKind::Other, format!("Failed to chmod: {}", e)));
        }

        if !chmod_status.unwrap().success() {
            return Err(io::Error::new(io::ErrorKind::Other, "chmod failed for yt-dlp"));
        }

        eprintln!("[ensure] yt-dlp downloaded and made executable");
    }

    Ok(yt_dlp_path)
}


pub async fn ensure_ffmpeg_exists() -> io::Result<Option<PathBuf>> {
    let app_dir = env::current_exe()?.parent().unwrap().to_path_buf();
    let ffmpeg_path = app_dir.join("ffmpeg");

    if ffmpeg_path.exists() {
        return Ok(Some(ffmpeg_path));
    }

    eprintln!("[ensure] ffmpeg not found, downloading zip...");

    let zip_url = "https://evermeet.cx/ffmpeg/ffmpeg-118896-g9f0970ee35.zip";
    let response = reqwest::get(zip_url).await.map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Download failed: {}", e))
    })?;
    let bytes = response.bytes().await.map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Read failed: {}", e))
    })?;

    let reader = Cursor::new(bytes);
    let mut archive = ZipArchive::new(reader).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Zip parse failed: {}", e))
    })?;

    let mut extracted = false;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = ffmpeg_path.clone();

        if file.name().ends_with("ffmpeg") {
            let mut outfile = File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
            extracted = true;
            break;
        }
    }

    if !extracted {
        return Err(io::Error::new(io::ErrorKind::Other, "ffmpeg not found in zip"));
    }

    // chmod +x
    Command::new("chmod")
        .args(["+x", ffmpeg_path.to_str().unwrap()])
        .status()
        .await?;

    eprintln!("[ensure] ffmpeg downloaded, extracted, and made executable");
    Ok(Some(ffmpeg_path))
}

#[command]
async fn download_soundcloud(url: String, app: AppHandle) -> String {
    use std::path::PathBuf;

    eprintln!("[SC1] Start download_soundcloud");

    if !url.contains("soundcloud.com") {
        eprintln!("[Error] Invalid SoundCloud URL: {}", &url);
        return "invalid URL".into();
    }

    let yt_dlp_path = match ensure_yt_dlp_exists().await {
        Ok(p) => {
            eprintln!("[SC2] yt-dlp path resolved: {:?}", p);
            p
        }
        Err(e) => {
            eprintln!("[Error] Failed to ensure yt-dlp: {}", e);
            return format!("Error ensuring yt-dlp: {e}");
        }
    };
    let ffmpeg_path = ensure_ffmpeg_exists().await.unwrap_or(None);

    let title_output = Command::new(&yt_dlp_path)
        .args(["--print", "title", &url])
        .output()
        .await;

    let video_title = match title_output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout).trim().to_owned();
            eprintln!("[SC3] yt-dlp title: {}", stdout);
            stdout
        }
        Ok(out) => {
            let stderr = String::from_utf8_lossy(&out.stderr);
            return format!("Error: yt-dlp failed to get title: {stderr}");
        }
        Err(e) => return format!("Error: Failed to execute yt-dlp: {e}"),
    };

    let sanitized_title = sanitize_filename(&video_title);
    eprintln!("[SC4] Sanitized title: {}", sanitized_title);

    let default_dir = app
        .path()
        .download_dir()
        .unwrap_or_else(|_| PathBuf::from("~/Downloads"));

    let folder = app
        .dialog()
        .file()
        .set_directory(default_dir)
        .blocking_pick_folder(); // 폴더 선택

    let Some(folder_path) = folder else {
        eprintln!("[Info] Folder pick canceled");
        return "canceled".into();
    };

    let folder_path = match folder_path.into_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("[Error] Invalid folder path: {e}");
            return format!("invalid folder path: {e}");
        }
    };
    eprintln!("[SC5] User-selected folder: {}", folder_path.display());

    let output_path = folder_path.join(format!("{sanitized_title}.mp3"));
    eprintln!("[SC6] Output path: {:?}", output_path);

    let mut cmd = Command::new(&yt_dlp_path);
    cmd.args(["-x", "--audio-format", "mp3", "--audio-quality", "0"]) // 최고 품질
        .args(["--embed-thumbnail", "--embed-metadata", "--add-metadata"])
        .args(["-o", output_path.to_str().unwrap()])
        .arg(&url);

    if let Some(ref ffmpeg) = ffmpeg_path {
        if let Some(p) = ffmpeg.to_str() {
            cmd.args(["--ffmpeg-location", p]);
        }
    }

    let output = cmd.output().await;

    if let Ok(ref r) = output {
        std::fs::write("/tmp/sc-yt-dlp-stdout.txt", &r.stdout).ok();
        std::fs::write("/tmp/sc-yt-dlp-stderr.txt", &r.stderr).ok();
    }

    match output {
        Ok(res) if res.status.success() => {
            eprintln!("[SC7] Download completed successfully");
            format!("Ok: path={}", output_path.display())
        }
        Ok(res) => {
            let stderr = String::from_utf8_lossy(&res.stderr);
            format!("Error: yt-dlp failed: {stderr}")
        }
        Err(e) => format!("Error: Failed to run yt-dlp: {e}"),
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            download_youtube,
            download_soundcloud
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(
                &window,
                NSVisualEffectMaterial::UnderWindowBackground,
                None,
                None,
            )
            .expect("macOS vibrancy 실패");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Windows blur 실패");

            Ok(())
        })
        .run(tauri::generate_context!())          // ⬅︎ Result 소비
        .expect("error while running tauri application");
}
