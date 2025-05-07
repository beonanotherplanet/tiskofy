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
import { match_url } from "./utils/url";

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
  "invalid-url": "❓ The URL is wrong. Please check the URL.",
  unknown: "❌ An unexpected error has occurred.",
  processing: "📁 busy extracting the audio from the web page...",
  canceled:
    "🥺 Oh, you canceled! That’s okay—just press the Download button again to continue.",
  none: "-",
};

function App() {
  const [url, setUrl] = useState("");
  const [status, setStatus] = useState<Status>("none");
  const [isLoading, setIsLoading] = useState(false);
  const [err, setErr] = useState("");

  async function handleDownload() {
    const service = match_url(url);

    if (!service || !url.trim()) {
      setStatus("invalid-url");
      return;
    }

    if (isLoading) {
      setStatus("canceled");
      setIsLoading(false);
      return;
    }

    setIsLoading(true);
    setStatus("processing");

    try {
      const getDownload = async () => {
        switch (service) {
          case "youtube":
            return await invoke<string>("download_youtube", { url });
          case "soundcloud":
            return await invoke<string>("download_soundcloud", { url });
        }
      };

      const response: any = await getDownload();

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
          <h1 className={head}>
            Get the mp3 file
            <br />
            from the url of various music platforms
          </h1>
        </header>
        <main>
          <div className={input_box}>
            <input
              className={input}
              type="text"
              placeholder="Copy & Paste the URL"
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
            <p>ver 1.2.0</p>
            <p>Currently, only YouTube and SoundCloud are supported.</p>
            <p className={status_style}>{ERROR_MSG[status]}</p>
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
