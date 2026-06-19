import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { Panel } from "./windows/panel/panel";
import { Backdrop } from "./windows/backdrop/backdrop";
import "./index.css";

// One Vite bundle, multiple Tauri webview windows routed by label. M2 ships the
// panel + a click-catching backdrop; settings/focus/onboarding join later.
const rootFor = (label: string) => {
  switch (label) {
    case "backdrop":
      return <Backdrop />;
    case "panel":
    default:
      return <Panel />;
  }
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>{rootFor(getCurrentWindow().label)}</React.StrictMode>,
);
