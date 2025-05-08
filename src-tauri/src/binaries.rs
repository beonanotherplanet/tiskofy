use std::{env, io, path::PathBuf};
use tokio::{process::Command, sync::OnceCell};
use std::io::Cursor;
use zip::ZipArchive;

static YT_DLP: OnceCell<PathBuf> = OnceCell::const_new();
static FFMPEG: OnceCell<Option<PathBuf>> = OnceCell::const_new();

pub async fn yt_dlp_path() -> io::Result<&'static PathBuf> {
    YT_DLP.get_or_try_init(|| async { ensure_yt_dlp().await }).await
}
pub async fn ffmpeg_path() -> io::Result<Option<&'static PathBuf>> {
    FFMPEG
        .get_or_try_init(|| async { ensure_ffmpeg().await })
        .await
        .map(|opt| opt.as_ref())
}

async fn ensure_yt_dlp() -> io::Result<PathBuf> {
    let app_dir   = env::current_exe()?.parent()
                      .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "no exe dir"))?
                      .to_path_buf();
    let bin_path  = app_dir.join("yt-dlp");

    if bin_path.exists() {
        return Ok(bin_path);
    }

    // 다운로드
    Command::new("curl")
        .args(["-L",
               "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp",
               "-o", bin_path.to_str().unwrap()])
        .status()
        .await?
        .success()
        .then_some(())
        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "yt-dlp download failed"))?;

    // 실행 권한 수정
    Command::new("chmod")
        .args(["+x", bin_path.to_str().unwrap()])
        .status()
        .await?;

    Ok(bin_path)
}

async fn ensure_ffmpeg() -> io::Result<Option<PathBuf>> {
    let app_dir  = env::current_exe()?.parent().unwrap().to_path_buf();
    let bin_path = app_dir.join("ffmpeg");

    if bin_path.exists() {
        return Ok(Some(bin_path));
    }

    const ZIP_URL: &str = "https://evermeet.cx/ffmpeg/ffmpeg-118896-g9f0970ee35.zip";

    let resp  = reqwest::get(ZIP_URL)
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("download error: {e}")))?;
    let bytes = resp
        .bytes()
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("read body error: {e}")))?;

    let mut archive = ZipArchive::new(Cursor::new(bytes))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("zip error: {e}")))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if file.name().ends_with("ffmpeg") {
            // fs::copy → io::copy 로 교체 (첫째 인자로 Reader 필요)
            let mut out = std::fs::File::create(&bin_path)?;
            io::copy(&mut file, &mut out)?;
            // 실행 권한 부여
            #[cfg(unix)]
            Command::new("chmod")
                .args(["+x", bin_path.to_str().unwrap()])
                .status()
                .await?;
            return Ok(Some(bin_path));
        }
    }

    // ffmpeg 가 없어도 yt-dlp 가 static build 로 대체할 수도 있으니 None 허용
    Ok(None)
}
