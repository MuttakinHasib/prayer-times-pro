// SF-Symbols-style line icons, ported from design_handoff/components/icons.jsx.
import type { SVGProps } from "react";

type P = SVGProps<SVGSVGElement>;

const Svg = ({ children, ...p }: P & { children: React.ReactNode }) => (
  <svg
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth={1.7}
    strokeLinecap="round"
    strokeLinejoin="round"
    width={16}
    height={16}
    {...p}
  >
    {children}
  </svg>
);

/** Fajr — dawn: sun rising above the horizon with an up arrow. */
export const DawnIcon = (p: P) => (
  <Svg {...p}>
    <path d="M3 18h18M7.5 14a4.5 4.5 0 0 1 9 0" />
    <path d="M12 3v5M12 3 9.5 5.5M12 3l2.5 2.5M2 18.5h2M20 18.5h2" />
  </Svg>
);
/** Sunrise — sun resting on the horizon with rays. */
export const SunHorizonIcon = (p: P) => (
  <Svg {...p}>
    <path d="M3 18h18" />
    <path d="M7 18a5 5 0 0 1 10 0" />
    <path d="M12 4.5v3M4.6 8.6l1.4 1.4M19.4 8.6 18 10M2 14h1.5M20.5 14H22" />
  </Svg>
);
/** Dhuhr — full midday sun. */
export const SunIcon = (p: P) => (
  <Svg {...p}>
    <circle cx="12" cy="12" r="4" />
    <path d="M12 2v2M12 20v2M22 12h-2M4 12H2M19 5l-1.5 1.5M6.5 17.5 5 19M19 19l-1.5-1.5M6.5 6.5 5 5" />
  </Svg>
);
/** Asr — afternoon: cloud drifting over the sun. */
export const CloudSunIcon = (p: P) => (
  <Svg {...p}>
    <circle cx="8.5" cy="8" r="3" />
    <path d="M8.5 2.5v1.4M3.5 8H2.1M4.6 4.1l1 1M13.4 4.1l-1 1M2.9 12.4l1-.6" />
    <path d="M7 19h9.5a3.2 3.2 0 0 0 .3-6.4 4.4 4.4 0 0 0-8.3-.6A3.4 3.4 0 0 0 7 19z" />
  </Svg>
);
/** Maghrib — sunset: sun dropping below the horizon with a down arrow. */
export const SunsetIcon = (p: P) => (
  <Svg {...p}>
    <path d="M3 18h18M7.5 14a4.5 4.5 0 0 1 9 0" />
    <path d="M12 8V3M12 8 9.5 5.5M12 8l2.5-2.5M2 18.5h2M20 18.5h2" />
  </Svg>
);
/** Isha — night: crescent moon with a star. */
export const MoonIcon = (p: P) => (
  <Svg {...p}>
    <path d="M19.5 13.6A7.5 7.5 0 1 1 10.4 4.5 5.8 5.8 0 0 0 19.5 13.6z" />
    <path d="M18 3.2l.7 1.6 1.6.7-1.6.7-.7 1.6-.7-1.6L15.7 5.5l1.6-.7z" />
  </Svg>
);
export const GearIcon = (p: P) => (
  <Svg {...p}>
    <circle cx="12" cy="12" r="3.2" />
    <path d="M12 2.6v2.2M12 19.2v2.2M21.4 12h-2.2M4.8 12H2.6M18.4 5.6l-1.6 1.6M7.2 16.8l-1.6 1.6M18.4 18.4l-1.6-1.6M7.2 7.2 5.6 5.6" />
  </Svg>
);
export const RefreshIcon = (p: P) => (
  <Svg {...p}>
    <path d="M20 11a8 8 0 0 0-14-4.5L4 8M4 13a8 8 0 0 0 14 4.5L20 16" />
    <path d="M4 4v4h4M20 20v-4h-4" />
  </Svg>
);
export const PowerIcon = (p: P) => (
  <Svg {...p}>
    <path d="M12 3v9" />
    <path d="M7.5 6.5a7 7 0 1 0 9 0" />
  </Svg>
);
export const PinIcon = (p: P) => (
  <Svg {...p}>
    <path d="M20.5 3.5 3.8 10.2c-.7.3-.7 1.3.1 1.5l6.7 1.9 1.9 6.7c.2.8 1.2.8 1.5.1z" />
  </Svg>
);

/** Map a prayer key to its time-of-day icon (mirrors the reference SF Symbols). */
export function PrayerIcon({ prayer, ...p }: P & { prayer: string }) {
  switch (prayer) {
    case "fajr":
      return <DawnIcon {...p} />;
    case "sunrise":
      return <SunHorizonIcon {...p} />;
    case "dhuhr":
    case "ishraq":
      return <SunIcon {...p} />;
    case "asr":
      return <CloudSunIcon {...p} />;
    case "maghrib":
      return <SunsetIcon {...p} />;
    case "isha":
      return <MoonIcon {...p} />;
    default:
      return <SunIcon {...p} />;
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
