import { useMemo } from "react";
import type { PrayerState } from "../../lib/ipc";
import { clock, dateEyebrow, hijriDate } from "../../lib/format";
import { PRAYER_NAMES } from "../../components/icons";
import { LiveCountdown } from "./live-countdown";
import { PrayerRing } from "./prayer-ring";

/** Mono date eyebrow, Hijri line, and the next-prayer hero with countdown ring. */
export const PanelHeader = ({ state }: { state: PrayerState }) => {
  const { next, times, tz, now_ms, show_hijri, hijri_adjustment } = state;

  // Ring window: from the most recent past time today to the next prayer.
  const fromMs = useMemo(() => {
    if (!next) return now_ms;
    const past = times.filter((t) => t.at_ms < next.at_ms).map((t) => t.at_ms);
    return past.length ? Math.max(...past) : next.at_ms - 6 * 3_600_000;
  }, [times, next, now_ms]);

  return (
    <div className="px-[22px] pb-[18px] pt-[22px]">
      <div className="font-mono text-[10.5px] font-semibold tracking-[0.16em] text-content-subtle">
        {dateEyebrow(now_ms, tz)}
      </div>
      {show_hijri && (
        <div className="mt-1.5 text-[12px] text-content-muted">
          {hijriDate({ ms: now_ms, tz, adjustmentDays: hijri_adjustment })}
        </div>
      )}

      {next && (
        <div className="mt-[18px] flex items-center gap-[18px]">
          <PrayerRing fromMs={fromMs} toMs={next.at_ms} size={64} />
          <div className="min-w-0">
            <div className="text-[13px] text-content-muted">
              Up next · {PRAYER_NAMES[next.prayer] ?? next.prayer}
            </div>
            <div className="font-display text-[32px] leading-none text-content">
              <LiveCountdown targetMs={next.at_ms} variant="long" className="tabular-nums" />
              <span className="text-[15px] text-content-muted"> remaining</span>
            </div>
            <div className="mt-1 text-[13px] text-accent">{clock(next.at_ms, tz)}</div>
          </div>
        </div>
      )}
    </div>
  );
};
