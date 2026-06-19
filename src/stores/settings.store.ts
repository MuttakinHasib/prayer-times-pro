import { create } from "zustand";
import { applySettings, getSettings, type AppSettings } from "../lib/settings";

interface SettingsStore {
  settings: AppSettings | null;
  hydrate: () => Promise<void>;
  /** Optimistically patch settings and persist via the Rust `apply_settings` command. */
  update: (patch: Partial<AppSettings>) => void;
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
    void applySettings(next).catch((err) => console.error("apply_settings failed", err));
  },
}));
