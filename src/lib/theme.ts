// Apply the user's AppearanceTheme to <html data-theme>. `auto` follows the
// system's prefers-color-scheme and re-applies if the OS switches at runtime.
import type { AppearanceTheme } from "./settings";

const QUERY = "(prefers-color-scheme: light)";
let mql: MediaQueryList | null = null;
let mqlListener: ((e: MediaQueryListEvent) => void) | null = null;

const resolved = (theme: AppearanceTheme): "dark" | "light" => {
  if (theme === "dark" || theme === "light") return theme;
  return typeof window !== "undefined" && window.matchMedia(QUERY).matches ? "light" : "dark";
};

export const applyTheme = (theme: AppearanceTheme) => {
  if (typeof document === "undefined") return;
  document.documentElement.dataset.theme = resolved(theme);

  // Detach any previous system listener; only `auto` needs one.
  if (mql && mqlListener) {
    mql.removeEventListener("change", mqlListener);
    mql = null;
    mqlListener = null;
  }
  if (theme === "auto") {
    mql = window.matchMedia(QUERY);
    mqlListener = () => {
      document.documentElement.dataset.theme = resolved("auto");
    };
    mql.addEventListener("change", mqlListener);
  }
};
