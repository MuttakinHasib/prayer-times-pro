// Locale + timezone-aware formatting via Intl. The Hijri date uses the
// Umm al-Qura calendar. (M6 wires full i18n; these already honor the master
// timezone passed from Rust.)

/** Short clock time in `tz`, e.g. "4:35 AM" / "16:35". */
export const clock = (ms: number, tz: string): string =>
  new Intl.DateTimeFormat(undefined, {
    hour: "numeric",
    minute: "2-digit",
    timeZone: tz,
  }).format(new Date(ms));

// Intl.DateTimeFormat construction is relatively expensive, so memoize one
// long-date formatter per timezone.
const longDateFormatters = new Map<string, Intl.DateTimeFormat>();
const longDateFormatter = (tz: string): Intl.DateTimeFormat => {
  let formatter = longDateFormatters.get(tz);
  if (!formatter) {
    formatter = new Intl.DateTimeFormat(undefined, {
      weekday: "long",
      day: "numeric",
      month: "long",
      year: "numeric",
      timeZone: tz,
    });
    longDateFormatters.set(tz, formatter);
  }
  return formatter;
};

/** Long date in `tz`, day-first, e.g. "Friday, 19 June, 2026". Field order is
 *  fixed (locale only supplies the weekday/month names) so it doesn't flip to
 *  month-first under en-US. */
export const longDate = (ms: number, tz: string): string => {
  const parts = longDateFormatter(tz).formatToParts(new Date(ms));
  const get = (type: Intl.DateTimeFormatPartTypes) =>
    parts.find((p) => p.type === type)?.value ?? "";
  return `${get("weekday")}, ${get("day")} ${get("month")}, ${get("year")}`;
};

/** Umm al-Qura Hijri date, e.g. "3 Muharram 1448 AH", with a whole-day shift. */
export const hijriDate = ({
  ms,
  tz,
  adjustmentDays,
}: {
  ms: number;
  tz: string;
  adjustmentDays: number;
}): string => {
  const shifted = new Date(ms + adjustmentDays * 86_400_000);
  const parts = new Intl.DateTimeFormat("en-u-ca-islamic-umalqura", {
    day: "numeric",
    month: "long",
    year: "numeric",
    era: "short",
    timeZone: tz,
  }).formatToParts(shifted);
  const get = (type: string) => parts.find((p) => p.type === type)?.value ?? "";
  return `${get("day")} ${get("month")} ${get("year")} ${get("era")}`.trim();
};

/** Full H:MM:SS countdown for the panel hero. */
export const countdownLong = (seconds: number): string => {
  const total = Math.max(0, Math.floor(seconds));
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  return `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
};

/** Compact countdown chip: "3h 25m", "25m", or "45s". */
export const shortCountdown = (seconds: number): string => {
  const total = Math.max(0, Math.floor(seconds));
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  if (h > 0) return `${h}h ${m}m`;
  if (m > 0) return `${m}m`;
  return `${s}s`;
};
