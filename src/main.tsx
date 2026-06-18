import React from "react";
import ReactDOM from "react-dom/client";
import { getCurrentWindow } from "@tauri-apps/api/window";
import Panel from "./windows/panel/Panel";
import "./index.css";

// One Vite bundle, multiple Tauri webview windows routed by label. M2 only ships
// the panel; settings/focus/onboarding windows join in later milestones.
function rootFor(label: string) {
  switch (label) {
    case "panel":
    default:
      return <Panel />;
  }
}

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>{rootFor(getCurrentWindow().label)}</React.StrictMode>,
);
