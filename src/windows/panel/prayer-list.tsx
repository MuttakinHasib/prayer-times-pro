import { useMemo } from "react";
import type { PrayerInstant, PrayerState } from "../../lib/ipc";
import { clock } from "../../lib/format";
import { cn } from "../../lib/cn";
import { PRAYER_NAMES, PrayerIcon } from "../../components/icons";
import { LiveCountdown } from "./live-countdown";

/** Today's times, the next one highlighted and past ones dimmed. */
export const PrayerList = ({ state }: { state: PrayerState }) => {
  const { times, next, show_ishraq, ishraq_ms } = state;

  // Insert the optional Ishraq row right after Sunrise.
  const rows = useMemo<PrayerInstant[]>(() => {
    if (!show_ishraq || ishraq_ms == null) return times;
    return times.flatMap((t) =>
      t.prayer === "sunrise" ? [t, { prayer: "ishraq", at_ms: ishraq_ms }] : [t],
    );
  }, [times, show_ishraq, ishraq_ms]);

  return (
    <div className="px-2 pb-2 pt-1">
      {rows.map((p) => {
        const isNext = !!next && p.prayer === next.prayer && p.at_ms === next.at_ms;
        // The next prayer is the first future one, so anything earlier is past.
        const isPast = !!next && !isNext && p.at_ms < next.at_ms;
        const minor = p.prayer === "ishraq" || p.prayer === "sunrise";

        return (
          <div
            key={`${p.prayer}-${p.at_ms}`}
            className={cn("flex items-center gap-2.5 rounded-lg px-2 py-1 text-[12px]", {
              "bg-accent/15": isNext,
              "opacity-40": isPast,
            })}
          >
            <span
              className={cn("flex w-[18px] justify-center", {
                "text-accent-emphasis": isNext,
                "text-content-muted": !isNext,
              })}
            >
              <PrayerIcon prayer={p.prayer} size={15} strokeWidth={2} />
            </span>
            <span
              className={cn("flex-1", {
                "font-semibold text-accent-emphasis": isNext,
                "text-content-muted": minor && !isNext,
              })}
            >
              {PRAYER_NAMES[p.prayer] ?? p.prayer}
            </span>
            {isNext && (
              <LiveCountdown
                targetMs={p.at_ms}
                variant="short"
                className="rounded-full bg-accent/20 px-1.5 py-px text-[10px] font-semibold tabular-nums text-accent-emphasis"
              />
            )}
            <span
              className={cn("tabular-nums", {
                "font-semibold text-accent-emphasis": isNext,
                "text-content-muted": minor && !isNext,
                "text-content": !isNext && !minor,
              })}
            >
              {clock(p.at_ms, state.tz)}
            </span>
          </div>
        );
      })}
    </div>
  );
};
