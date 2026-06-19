import { invoke } from "@tauri-apps/api/core";

/**
 * Transparent full-screen click-catcher shown below the panel. A press anywhere
 * dismisses the panel (and this backdrop) and consumes the click, so it never
 * reaches the wallpaper (no macOS "reveal desktop" gesture).
 */
export const Backdrop = () => (
  <div
    className="fixed inset-0"
    onPointerDown={() => {
      void invoke("dismiss_panel");
    }}
  />
);
