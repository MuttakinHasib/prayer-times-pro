// Locale + timezone-aware formatting via Intl. The Hijri date uses the
// Umm al-Qura calendar to match the macOS app. (M6 wires full i18n; these
// already honor the master timezone passed from Rust.)

/** Short clock time in `tz`, e.g. "4:35 AM" / "16:35". */
export function clock(ms: number, tz: string): string {
  return new Intl.DateTimeFormat(undefined, {
    hour: "numeric",
    minute: "2-digit",
    timeZone: tz,
  }).format(new Date(ms));
}

/** Long date in `tz`, day-first to match the reference app, e.g.
 *  "Friday, 19 June, 2026". Field order is fixed (locale only supplies the
 *  weekday/month names) so it doesn't flip to month-first under en-US. */
export function longDate(ms: number, tz: string): string {
  const d = new Date(ms);
  const part = (opt: Intl.DateTimeFormatOptions) =>
    new Intl.DateTimeFormat(undefined, { ...opt, timeZone: tz }).format(d);
  return `${part({ weekday: "long" })}, ${part({ day: "numeric" })} ${part({
    month: "long",
  })}, ${part({ year: "numeric" })}`;
}

/** Umm al-Qura Hijri date, e.g. "3 Muharram 1448 AH", with a whole-day shift. */
export function hijriDate(ms: number, tz: string, adjustmentDays: number): string {
  const shifted = new Date(ms + adjustmentDays * 86_400_000);
  const parts = new Intl.DateTimeFormat("en-u-ca-islamic-umalqura", {
    day: "numeric",
    month: "long",
    year: "numeric",
    era: "short",
    timeZone: tz,
  }).formatToParts(shifted);
  const get = (t: string) => parts.find((p) => p.type === t)?.value ?? "";
  return `${get("day")} ${get("month")} ${get("year")} ${get("era")}`.trim();
}

/** Full H:MM:SS countdown for the panel hero. */
export function countdownLong(seconds: number): string {
  const total = Math.max(0, Math.floor(seconds));
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  return `${h}:${String(m).padStart(2, "0")}:${String(s).padStart(2, "0")}`;
}

/** Compact countdown chip: "3h 25m", "25m", or "45s". */
export function shortCountdown(seconds: number): string {
  const total = Math.max(0, Math.floor(seconds));
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  if (h > 0) return `${h}h ${m}m`;
  if (m > 0) return `${m}m`;
  return `${s}s`;
}
