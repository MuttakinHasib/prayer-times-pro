// AppSettings mirror of the Rust prayer-core model + its IPC. Keys are camelCase
// to match the serde encoding.
import { invoke } from "@tauri-apps/api/core";

export type PrayerKey = "fajr" | "sunrise" | "dhuhr" | "asr" | "maghrib" | "isha";
export const OBLIGATORY: PrayerKey[] = ["fajr", "dhuhr", "asr", "maghrib", "isha"];

export type MenuBarStyle =
  | "iconOnly"
  | "countdownOnly"
  | "iconCountdown"
  | "nextPrayerCountdown"
  | "iconNameCountdown"
  | "nextPrayerClock"
  | "iconNameClock";
export type MenuBarCountdownMode = "nextPrayer" | "currentWaqt";
export type FocusBlurIntensity = "low" | "medium" | "high" | "opaque";
export type FocusTrigger = "obligatory" | "all" | "fajrIsha";
export type CalculationMode = "calculated" | "manual";
export type LocationMode = "automatic" | "manual";
export type HighLatitudeRule =
  | "automatic"
  | "none"
  | "middleOfNight"
  | "seventhOfNight"
  | "angleBased";
export type NotificationSound =
  | "none"
  | "systemDefault"
  | "softChime"
  | "takbir"
  | "adhanMakkah"
  | "adhanMadinah";

export interface Coordinates {
  latitude: number;
  longitude: number;
  elevation: number;
}

export interface NotificationDefaults {
  sound: NotificationSound;
  playFullAdhan: boolean;
  earlyReminderMinutes: number;
  iqamahOffsetMinutes: number;
}

export interface PrayerNotificationConfig {
  notify: boolean;
  playFullAdhan: boolean;
  earlyReminderEnabled: boolean;
  soundOverride: NotificationSound | null;
  earlyLeadMinutesOverride: number | null;
  iqamahOffsetMinutesOverride: number | null;
}

export interface AppSettings {
  methodId: string;
  manualParameters: unknown | null;
  hanafiAsr: boolean;
  highLatitudeRule: HighLatitudeRule;
  locationMode: LocationMode;
  manualCoordinates: Coordinates | null;
  timezoneOverride: string | null;
  autoDetectMethod: boolean;
  calculationMode: CalculationMode;
  azanBeforeJamaat: number;
  manualKeepWaqt: boolean;
  jamaatTimes: Partial<Record<PrayerKey, number>>;
  menuBarStyle: MenuBarStyle;
  menuBarCountdownMode: MenuBarCountdownMode;
  showIshraqTime: boolean;
  showHijriDate: boolean;
  hijriDayAdjustment: number;
  focusModeEnabled: boolean;
  focusDurationMinutes: number;
  focusBlurIntensity: FocusBlurIntensity;
  focusTrigger: FocusTrigger;
  focusEmergencyExitEnabled: boolean;
  launchAtLogin: boolean;
  languageOverride: string | null;
  masterNotificationsEnabled: boolean;
  notificationDefaults: NotificationDefaults;
  notifications: Partial<Record<PrayerKey, PrayerNotificationConfig>>;
  autoUpdateEnabled: boolean;
  didCompleteOnboarding: boolean;
}

/** Built-in calculation methods (id → display name), matching the Rust registry. */
export const METHODS: { id: string; name: string }[] = [
  { id: "diyanet", name: "Diyanet İşleri (Türkiye)" },
  { id: "mwl", name: "Muslim World League" },
  { id: "isna", name: "Islamic Society of North America" },
  { id: "ummalqura", name: "Umm al-Qura (Makkah)" },
  { id: "egyptian", name: "Egyptian General Authority of Survey" },
  { id: "karachi", name: "University of Islamic Sciences, Karachi" },
  { id: "jakim", name: "JAKIM (Malaysia)" },
  { id: "kemenag", name: "Kemenag (Indonesia)" },
  { id: "moonsighting", name: "Moonsighting Committee Worldwide" },
];

export const getSettings = () => invoke<AppSettings>("get_settings");
export const applySettings = (settings: AppSettings) =>
  invoke<void>("apply_settings", { settings });
