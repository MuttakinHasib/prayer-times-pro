import type { PrayerState } from "../../lib/ipc";
import { clock, hijriDate, longDate } from "../../lib/format";
import { PRAYER_NAMES, PrayerIcon } from "../../components/icons";
import { LiveCountdown } from "./live-countdown";

/** Date line, optional Hijri line, and the next-prayer hero. */
export const PanelHeader = ({ state }: { state: PrayerState }) => {
  const { next, tz, now_ms, show_hijri, hijri_adjustment } = state;

  return (
    <div className="px-4 pb-2.5 pt-3">
      <div className="text-[10px] font-semibold tracking-[0.05em] text-content-muted">
        {longDate(now_ms, tz).toUpperCase()}
      </div>
      {show_hijri && (
        <div className="mt-0.5 text-[10px] text-content-subtle">
          {hijriDate({ ms: now_ms, tz, adjustmentDays: hijri_adjustment })}
        </div>
      )}

      {next && (
        <div className="mt-2.5 flex items-center gap-2.5">
          <PrayerIcon
            prayer={next.prayer}
            size={18}
            strokeWidth={2}
            className="shrink-0 text-accent-emphasis"
          />
          <div className="flex-1">
            <div className="text-[17px] font-bold leading-tight tracking-[-0.01em]">
              {PRAYER_NAMES[next.prayer] ?? next.prayer}
            </div>
            <div className="mt-px text-[11px] text-content-muted">
              in{" "}
              <LiveCountdown targetMs={next.at_ms} variant="long" className="tabular-nums" />
            </div>
          </div>
          <div className="text-[14px] font-semibold tabular-nums text-accent-emphasis">
            {clock(next.at_ms, tz)}
          </div>
        </div>
      )}
    </div>
  );
};
