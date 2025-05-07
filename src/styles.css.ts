import { globalFontFace, style, keyframes } from "@vanilla-extract/css";

const wantedSans = "wantedSans";

globalFontFace(wantedSans, [
  {
    src: "url(/fonts/WantedSansStd-Regular.woff2)",
    fontWeight: 400,
  },
  {
    src: "url(/fonts/WantedSansStd-Bold.woff2)",
    fontWeight: 700,
  },
  {
    src: "url(/fonts/WantedSansStd-Black.woff2)",
    fontWeight: 900,
  },
]);

export const font = style({
  fontFamily: wantedSans,
});

export const background = style({
  width: "100%",
  display: "flex",
  flexDirection: "column",
});

export const box_container = style({
  width: 400,
  display: "flex",
  flexWrap: "wrap",
});

export const delete_txt = style({
  opacity: 0.5,
  fontSize: 12,
});

export const description_box = style({
  width: "100%",
  color: "#111",
  fontFamily: wantedSans,
  borderRadius: 12,
  fontSize: 14,
  padding: "12px 40px",
  // background: "blue",
  boxSizing: "border-box",
});

const spin = keyframes({
  from: { transform: "rotate(0deg)" },
  to: { transform: "rotate(360deg)" },
});

export const loading = style({
  animation: `${spin} 1s linear infinite`,
});

export const input_box = style({
  display: "flex",
  alignItems: "center",
  width: "100%",
  padding: "20px 40px 0",
  boxSizing: "border-box",
});

export const input = style({
  padding: "2px 12px",
  fontSize: "14px",
  minWidth: 400,
  height: 36,
  color: "#111",
  boxSizing: "border-box",
  border: "1px solid #ddd",
  boxShadow: "0px 0px 8px #eeeeee99",
  fontFamily: wantedSans,
  borderRadius: 8,
  ":focus": {
    outline: "none",
  },
  "::placeholder": {
    color: "#55555555",
  },
});

export const download_button = style({
  borderRadius: 8,
  padding: "2px 12px",
  fontSize: "14px",
  height: 36,
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  color: "#fff",
  background: "#111",
  width: 120,
  marginLeft: 12,
  border: "1px solid #111",
  boxShadow: "0px 0px 8px #eeeeee99",
  cursor: "pointer",
  ":hover": {
    background: "#222",
  },
  ":disabled": {
    background: "#aaaaaa55",
    color: "#ffffff55",
    cursor: "not-allowed",
  },
  fontFamily: wantedSans,
});

export const footer = style({
  position: "fixed",
  bottom: 0,
  display: "flex",
  justifyContent: "center",
  left: 0,
  textAlign: "center",
  width: "100%",
  padding: 10,
  fontSize: 12,
  fontFamily: wantedSans,
  color: "#aaa",
  zIndex: 100,
});

export const head = style({
  fontFamily: wantedSans,
  fontWeight: 800,
  fontSize: 32,
  padding: "40px 40px 0",
  color: "#111",
  margin: 0,
  textTransform: "uppercase",
});

export const status_style = style({
  fontWeight: "bolder",
});

export const footer_link = style({
  color: "#1E232555",
  textDecoration: "none",
});

export const loading_container = style({
  display: "flex",
  fontSize: 36,
  color: "#111",
  padding: "16px 32px",
});
