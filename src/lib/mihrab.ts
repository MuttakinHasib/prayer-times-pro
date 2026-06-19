// Single source of truth for the Mihrab brand-mark geometry (viewBox 0 0 100 100).
// Imported by both the React component (logo.tsx) and the icon renderer
// (scripts/render-icons.mjs) so the two never drift apart.

/** Outer niche-archway outline. */
export const MIHRAB_FRAME_OUTER =
  "M27,85 L27,52 C27,31 38,17 50,12 C62,17 73,31 73,52 L73,85 Z";
/** Inner cutout that turns the outline into a frame. */
export const MIHRAB_FRAME_INNER =
  "M36,85 L36,54 C36,38 43,27 50,23 C57,27 64,38 64,54 L64,85 Z";
/** Filled archway silhouette for small / monochrome use. */
export const MIHRAB_ARCH_SOLID =
  "M30,86 L30,52 C30,31 40,17 50,12 C60,17 70,31 70,52 L70,86 Z";

/** Crescent carved from two offset circles (white keep, black cut). */
export const MIHRAB_CRESCENT = {
  keep: { cx: 51.5, cy: 49, r: 8.7 },
  cut: { cx: 57.5, cy: 44.5, r: 7.1 },
} as const;

/** Gold radial-gradient stops (obsidian-and-gold brand system). */
export const MIHRAB_GOLD_STOPS = [
  { offset: 0, color: "#ecd49a" },
  { offset: 0.6, color: "#c8a968" },
  { offset: 1, color: "#a8893f" },
] as const;
