import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AiOutlineLoading } from "react-icons/ai";
// import { open } from "@tauri-apps/plugin-dialog";
import {
  background,
  input_box,
  input,
  download_button,
  loading,
  description_box,
  status_style,
  footer,
  head,
  footer_link,
  loading_container,
} from "./styles.css";

type Status =
  | "completed"
  | "invalid-url"
  | "none"
  | "processing"
  | "canceled"
  | "unknown";

type ErrorMsg = {
  [key: string]: string;
};

const ERROR_MSG: ErrorMsg = {
  completed: "✅ The download is complete.",
  "invalid-url": "❓ The URL is wrong. Please check the YouTube video URL.",
  unknown: "❌ An unexpected error has occurred.",
  processing: "📁 busy extracting the audio from the YouTube clip...",
  canceled:
    "🥺 Oh, you canceled! That’s okay—just press the Download button again to continue.",
  none: "...",
};

function App() {
  const [url, setUrl] = useState("");
  const [status, setStatus] = useState<Status>("none");
  const [isLoading, setIsLoading] = useState(false);
  const [err, setErr] = useState("");

  async function handleDownload() {
    if (isLoading) {
      setStatus("canceled");
      setIsLoading(false);
      return;
    }

    if (!url.trim()) {
      setStatus("invalid-url");
      return;
    }

    setIsLoading(true);

    setStatus("processing");
    try {
      const response = await invoke<string>("download_mp3", { url });
      console.log(response);
      if (response.startsWith("Error")) {
        setStatus("unknown");
        setIsLoading(false);
        return;
      }
      if (response.startsWith("Ok")) {
        setStatus("completed");
        setIsLoading(false);
        return;
      }
      if (response.startsWith("canceled")) {
        setStatus("canceled");
        setIsLoading(false);
        return;
      }
      if (response.startsWith("invalid")) {
        setStatus("invalid-url");
        setIsLoading(false);
        return;
      }
      setIsLoading(false);
    } catch (error: any) {
      setIsLoading(false);
      setStatus("unknown");
      setErr(error);
    }
  }

  return (
    <>
      <div className={background}>
        <header>
          <h1 className={head}>Get the mp3 file from the Youtube URL</h1>
        </header>
        <main>
          <div className={input_box}>
            <input
              className={input}
              type="text"
              placeholder="Copy & Paste the Youtube URL..."
              name="url"
              value={url}
              onChange={({ target }) => setUrl(target.value)}
            />
            <button className={download_button} onClick={handleDownload}>
              {isLoading ? (
                <span className={loading}>
                  <AiOutlineLoading />
                </span>
              ) : (
                "Download"
              )}
            </button>
          </div>
          <div className={description_box}>
            <p>Ver 1.1.0</p>
            <p className={status_style}>{ERROR_MSG[status]}</p>
            <p>{status ? "status: " + status : "--"}</p>
            <div className={loading_container}>
              {isLoading && (
                <span className={loading}>
                  <AiOutlineLoading />
                </span>
              )}
            </div>

            <p>{err || ""}</p>
          </div>
        </main>
      </div>
      <footer className={footer}>
        &copy; 2025 &nbsp;
        <a className={footer_link} href="https://beonanotherplanet.com">
          Seungha Kim
        </a>
      </footer>
    </>
  );
}

export default App;
