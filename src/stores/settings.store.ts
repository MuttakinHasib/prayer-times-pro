import { create } from "zustand";
import { applySettings, detectLocation, getSettings, type AppSettings } from "../lib/settings";

interface SettingsStore {
  settings: AppSettings | null;
  hydrate: () => Promise<void>;
  /** Optimistically patch settings and persist via the Rust `apply_settings` command. */
  update: (patch: Partial<AppSettings>) => void;
  /** Detect location from IP; Rust persists + applies and returns the new settings. */
  detect: () => Promise<void>;
}

export const useSettingsStore = create<SettingsStore>((set, get) => ({
  settings: null,
  hydrate: async () => {
    try {
      set({ settings: await getSettings() });
    } catch (err) {
      console.error("get_settings failed", err);
    }
  },
  update: (patch) => {
    const current = get().settings;
    if (!current) return;
    const next = { ...current, ...patch };
    set({ settings: next });
    // Optimistic: roll back to the last-persisted snapshot if the write fails,
    // so the UI never shows a value the backend never stored.
    void applySettings(next).catch((err) => {
      console.error("apply_settings failed", err);
      set({ settings: current });
    });
  },
  detect: async () => {
    // Rust persists + applies; just reflect the returned settings. Errors bubble
    // so the caller can surface them.
    set({ settings: await detectLocation() });
  },
}));
