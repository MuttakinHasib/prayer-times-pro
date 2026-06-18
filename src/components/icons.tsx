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

export const DawnIcon = (p: P) => (
  <Svg {...p}>
    <path d="M4 16.5h16M6.5 13a5.5 5.5 0 0 1 11 0" />
    <path d="M12 4.5v1.6M5.6 6.6l1.1 1.1M18.4 6.6l-1.1 1.1" />
  </Svg>
);
export const SunriseIcon = (p: P) => (
  <Svg {...p}>
    <path d="M3 18h18M7.5 14a4.5 4.5 0 0 1 9 0" />
    <path d="M12 3v5M12 3 9.5 5.5M12 3l2.5 2.5M2 18.5h2M20 18.5h2" />
  </Svg>
);
export const SunIcon = (p: P) => (
  <Svg {...p}>
    <circle cx="12" cy="12" r="4" />
    <path d="M12 2v2M12 20v2M22 12h-2M4 12H2M19 5l-1.5 1.5M6.5 17.5 5 19M19 19l-1.5-1.5M6.5 6.5 5 5" />
  </Svg>
);
export const SunsetIcon = (p: P) => (
  <Svg {...p}>
    <path d="M3 18h18M7.5 14a4.5 4.5 0 0 1 9 0" />
    <path d="M12 8V3M12 8 9.5 5.5M12 8l2.5-2.5M2 18.5h2M20 18.5h2" />
  </Svg>
);
export const MoonIcon = (p: P) => (
  <Svg {...p}>
    <path d="M20 13.4A8 8 0 1 1 10.6 4 6.4 6.4 0 0 0 20 13.4z" />
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

/** Map a prayer key to its time-of-day icon. */
export function PrayerIcon({ prayer, ...p }: P & { prayer: string }) {
  switch (prayer) {
    case "fajr":
      return <DawnIcon {...p} />;
    case "sunrise":
      return <SunriseIcon {...p} />;
    case "dhuhr":
    case "ishraq":
      return <SunIcon {...p} />;
    case "asr":
      return <SunIcon {...p} />;
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
