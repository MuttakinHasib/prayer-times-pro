import { memo, useCallback, useEffect, useId, useState } from "react";

const TICK_MS = 1000;
/** Inner disc inset and crescent size, as fractions of the ring diameter. */
const INNER_INSET_PX = 12;
const CRESCENT_RATIO = 0.22;
const FULL_TURN_DEG = 360;

/**
 * Whole-degree progress (0–360) of the current→next window, re-evaluated each
 * second. State holds the rounded degree, so a tick that doesn't move the arc a
 * full degree bails out of `setState` — the ring re-renders ~once per ≈15 s, not
 * every second.
 */
const useRingDegrees = (fromMs: number, toMs: number): number => {
  const compute = useCallback(() => {
    const span = toMs - fromMs;
    const frac = span <= 0 ? 1 : Math.min(1, Math.max(0, (Date.now() - fromMs) / span));
    return Math.round(FULL_TURN_DEG * frac);
  }, [fromMs, toMs]);

  const [deg, setDeg] = useState(compute);
  useEffect(() => {
    setDeg(compute());
    const id = setInterval(() => {
      // Returning the previous value (same reference) makes React skip the render.
      setDeg((prev) => {
        const next = compute();
        return next === prev ? prev : next;
      });
    }, TICK_MS);
    return () => clearInterval(id);
  }, [compute]);
  return deg;
};

/** A small gold crescent built from two offset circles (static — memoized). */
const Crescent = memo(({ size }: { size: number }) => {
  const id = useId().replace(/:/g, "");
  return (
    <svg width={size} height={size} viewBox="0 0 100 100" aria-hidden="true">
      <defs>
        <mask id={`crescent-${id}`}>
          <rect width="100" height="100" fill="#000" />
          <circle cx="48" cy="50" r="42" fill="#fff" />
          <circle cx="63" cy="40" r="35" fill="#000" />
        </mask>
      </defs>
      <rect width="100" height="100" fill="var(--c-accent)" mask={`url(#crescent-${id})`} />
    </svg>
  );
});

/**
 * The hero countdown ring: a conic-gradient arc tracks progress toward the next
 * prayer, wrapping an inner disc with the crescent glyph.
 */
export const PrayerRing = ({
  fromMs,
  toMs,
  size = 64,
}: {
  fromMs: number;
  toMs: number;
  size?: number;
}) => {
  const deg = useRingDegrees(fromMs, toMs);
  const inner = size - INNER_INSET_PX;
  return (
    <div
      className="relative flex shrink-0 items-center justify-center rounded-full"
      style={{
        width: size,
        height: size,
        background: `conic-gradient(var(--c-accent) ${deg}deg, var(--c-elevated) 0)`,
      }}
    >
      <div
        className="flex items-center justify-center rounded-full bg-surface"
        style={{ width: inner, height: inner }}
      >
        <Crescent size={Math.round(size * CRESCENT_RATIO)} />
      </div>
    </div>
  );
};
