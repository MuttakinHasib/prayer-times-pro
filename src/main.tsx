import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Panel } from "./windows/panel/panel";
import { Backdrop } from "./windows/backdrop/backdrop";
import { Settings } from "./windows/settings/settings";
import { Focus } from "./windows/focus/focus";
import "@fontsource/newsreader/400.css";
import "@fontsource/newsreader/500.css";
import "@fontsource/newsreader/400-italic.css";
import "@fontsource-variable/jetbrains-mono/index.css";
import "@fontsource/amiri/400.css";
import "@fontsource/amiri/700.css";
import "@fontsource/noto-sans-bengali/400.css";
import "@fontsource/noto-sans-bengali/600.css";
import "./index.css";

// One Vite bundle, multiple Tauri webview windows routed by label. M2 ships the
// panel + a click-catching backdrop; settings/focus/onboarding join later.
const rootFor = (label: string) => {
  if (label.startsWith("focus")) return <Focus />;
  switch (label) {
    case "backdrop":
      return <Backdrop />;
    case "settings":
      return <Settings />;
    case "panel":
    default:
      return <Panel />;
  }
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>{rootFor(getCurrentWindow().label)}</React.StrictMode>,
);
