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
export const ADHAN_EVENT = "prayer://adhan-state";

export const getPrayerState = () => invoke<PrayerState>("get_prayer_state");
export const hidePanel = () => invoke<void>("hide_panel");
/** Report the panel's measured content size so Rust can size the window on show. */
export const reportContentSize = (width: number, height: number) =>
  invoke<void>("report_content_size", { width, height });
export const quitApp = () => invoke<void>("quit_app");
export const openSettings = () => invoke<void>("open_settings");
export const completeOnboarding = () => invoke<void>("complete_onboarding");
export const openOnboarding = () => invoke<void>("open_onboarding");
export const checkForUpdates = () => invoke<void>("check_for_updates");
export const stopAdhan = () => invoke<void>("stop_adhan");
export const sendTestNotification = () => invoke<void>("send_test_notification");
/** Returns true if notifications are granted (or just got granted), false if denied. */
export const ensureNotificationPermission = () => invoke<boolean>("ensure_notification_permission");

/// Subscribe to schedule-change pushes from Rust. Returns an unlisten fn.
export const onStateChanged = (cb: (s: PrayerState) => void): Promise<UnlistenFn> =>
  listen<PrayerState>(STATE_EVENT, (e) => cb(e.payload));

/// Subscribe to Adhan start/stop (`true`/`false`). Returns an unlisten fn.
export const onAdhanState = (cb: (playing: boolean) => void): Promise<UnlistenFn> =>
  listen<boolean>(ADHAN_EVENT, (e) => cb(e.payload));

export const FOCUS_ENGAGE_EVENT = "focus://engage";
export const FOCUS_DISMISS_EVENT = "focus://dismiss";

export interface FocusCue {
  prayer: string;
  durationMinutes: number;
  blur: "low" | "medium" | "high" | "opaque";
  emergencyExit: boolean;
}

/** Engage Focus Mode; omit `prayer` to use the next prayer (settings preview). */
export const engageFocus = (prayer?: string) =>
  invoke<void>("engage_focus", { prayer: prayer ?? null });
export const dismissFocus = () => invoke<void>("dismiss_focus");

/// Subscribe to Focus Mode engage cues. Returns an unlisten fn.
export const onFocusEngage = (cb: (cue: FocusCue) => void): Promise<UnlistenFn> =>
  listen<FocusCue>(FOCUS_ENGAGE_EVENT, (e) => cb(e.payload));

/// Subscribe to the dismiss broadcast so every overlay clears together.
export const onFocusDismiss = (cb: () => void): Promise<UnlistenFn> =>
  listen(FOCUS_DISMISS_EVENT, () => cb());
