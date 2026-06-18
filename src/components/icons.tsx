// Icons via lucide-react, chosen to mirror the macOS app's SF Symbols as closely
// as the set allows. The macOS originals (not bundleable on web):
//   fajr sunrise · sunrise sun.horizon.fill · dhuhr sun.max.fill ·
//   asr cloud.sun.fill · maghrib sunset.fill · isha moon.stars.fill
import {
  CloudSun,
  type LucideProps,
  Moon,
  MoonStar,
  Navigation,
  Power,
  RefreshCw,
  Settings,
  Sun,
  SunMedium,
  Sunrise,
  Sunset,
} from "lucide-react";

// UI icons (footer + summary), aliased so callers stay icon-library-agnostic.
export const GearIcon = (p: LucideProps) => <Settings {...p} />;
export const RefreshIcon = (p: LucideProps) => <RefreshCw {...p} />;
export const PowerIcon = (p: LucideProps) => <Power {...p} />;
export const MoonIcon = (p: LucideProps) => <Moon {...p} />;
export const PinIcon = (p: LucideProps) => <Navigation {...p} />;

/** Map a prayer key to its time-of-day icon (mirrors the reference SF Symbols). */
export function PrayerIcon({ prayer, ...p }: LucideProps & { prayer: string }) {
  switch (prayer) {
    case "fajr":
      return <Sunrise {...p} />;
    case "sunrise":
    case "ishraq":
      return <SunMedium {...p} />;
    case "dhuhr":
      return <Sun {...p} />;
    case "asr":
      return <CloudSun {...p} />;
    case "maghrib":
      return <Sunset {...p} />;
    case "isha":
      return <MoonStar {...p} />;
    default:
      return <Sun {...p} />;
  }
}

export const PRAYER_NAMES: Record<string, string> = {
  fajr: "Fajr",
  sunrise: "Sunrise",
  ishraq: "Ishraq",
  dhuhr: "Dhuhr",
  asr: "Asr",
  maghrib: "Maghrib",
  isha: "Isha",
};
