import { memo, useCallback, useEffect, useState } from "react";
import { PrayerIcon } from "../../components/icons";

const TICK_MS = 1000;
/** Inner-disc inset and centre-glyph size, relative to the ring diameter. */
const INNER_INSET_PX = 12;
const GLYPH_RATIO = 0.34;
const FULL_TURN_DEG = 360;

interface PrayerRingProps {
  prayer: string;
  fromMs: number;
  toMs: number;
  size?: number;
}

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

/**
 * The hero countdown ring: a conic-gradient arc tracks progress toward the next
 * prayer, wrapping an inner disc with that prayer's glyph.
 */
export const PrayerRing = memo(({ prayer, fromMs, toMs, size = 56 }: PrayerRingProps) => {
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
        <PrayerIcon
          prayer={prayer}
          size={Math.round(size * GLYPH_RATIO)}
          strokeWidth={1.75}
          className="text-accent"
        />
      </div>
    </div>
  );
});
PrayerRing.displayName = "PrayerRing";
