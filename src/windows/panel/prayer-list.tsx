import { memo, useMemo } from "react";
import type { PrayerInstant, PrayerState } from "../../lib/ipc";
import { clock } from "../../lib/format";
import { cn } from "../../lib/cn";
import { PRAYER_NAMES, PrayerIcon } from "../../components/icons";

interface PrayerListProps {
  state: PrayerState;
}

/** Today's times: past dimmed to tertiary, the next one a gold-tinted card. */
export const PrayerList = memo(({ state }: PrayerListProps) => {
  const { times, next, show_ishraq, ishraq_ms } = state;

  // Insert the optional Ishraq row right after Sunrise.
  const rows = useMemo<PrayerInstant[]>(() => {
    if (!show_ishraq || ishraq_ms == null) return times;
    return times.flatMap((t) =>
      t.prayer === "sunrise" ? [t, { prayer: "ishraq", at_ms: ishraq_ms }] : [t],
    );
  }, [times, show_ishraq, ishraq_ms]);

  return (
    <div className="px-3 py-1.5">
      {rows.map((p) => {
        const isNext = !!next && p.prayer === next.prayer && p.at_ms === next.at_ms;
        const isPast = !!next && !isNext && p.at_ms < next.at_ms;

        return (
          <div
            key={`${p.prayer}-${p.at_ms}`}
            className={cn(
              "flex items-center gap-2.5 rounded-lg px-2",
              isNext ? "border border-accent-ring bg-accent-soft py-1.5" : "py-[5px]",
            )}
          >
            <PrayerIcon
              prayer={p.prayer}
              size={15}
              strokeWidth={1.75}
              className={cn("shrink-0", {
                "text-accent": isNext,
                "text-content-subtle": isPast,
                "text-content-muted": !isNext && !isPast,
              })}
            />
            <span
              className={cn("flex-1 text-[13px]", {
                "font-semibold text-content": isNext,
                "text-content-subtle": isPast,
                "text-content": !isNext && !isPast,
              })}
            >
              {PRAYER_NAMES[p.prayer] ?? p.prayer}
            </span>
            <span
              className={cn("font-mono text-[12.5px] tabular-nums", {
                "font-semibold text-accent": isNext,
                "text-content-subtle": isPast,
                "font-medium text-content": !isNext && !isPast,
              })}
            >
              {clock(p.at_ms, state.tz)}
            </span>
          </div>
        );
      })}
    </div>
  );
});
PrayerList.displayName = "PrayerList";
