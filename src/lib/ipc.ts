// Thin typed bridge over the Tauri IPC surface. Rust sends epoch-millis instants;
// the frontend formats + counts down locally (see lib/format.ts).
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface PrayerInstant {
  prayer: string;
  at_ms: number;
}

export interface WaqtDto {
  prayer: string;
  end_ms: number;
  is_obligatory: boolean;
}

export interface PrayerState {
  tz: string;
  now_ms: number;
  method_name: string;
  latitude: number;
  longitude: number;
  show_ishraq: boolean;
  show_hijri: boolean;
  hijri_adjustment: number;
  ishraq_ms: number | null;
  next: PrayerInstant | null;
  current_waqt: WaqtDto | null;
  times: PrayerInstant[];
}

export const STATE_EVENT = "prayer://state-changed";

export const getPrayerState = () => invoke<PrayerState>("get_prayer_state");
export const hidePanel = () => invoke<void>("hide_panel");
/** Resize the panel window to the measured content size and re-pin its top-left. */
export const fitPanel = (width: number, height: number) =>
  invoke<void>("fit_panel", { width, height });
export const quitApp = () => invoke<void>("quit_app");
export const openSettings = () => invoke<void>("open_settings");
export const checkForUpdates = () => invoke<void>("check_for_updates");

/// Subscribe to schedule-change pushes from Rust. Returns an unlisten fn.
export const onStateChanged = (cb: (s: PrayerState) => void): Promise<UnlistenFn> =>
  listen<PrayerState>(STATE_EVENT, (e) => cb(e.payload));
