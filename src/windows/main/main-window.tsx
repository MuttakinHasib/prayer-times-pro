import { useEffect, useMemo, useState } from "react";
import { Bell, CalendarDays, House, Settings as Gear, Target, type LucideIcon } from "lucide-react";
import { cn } from "../../lib/cn";
import { clock, dateEyebrow, hijriDate } from "../../lib/format";
import { engageFocus, openSettings } from "../../lib/ipc";
import type { PrayerInstant, PrayerState } from "../../lib/ipc";
import { initPrayerStore, usePrayerStore } from "../../stores/prayer.store";
import { PRAYER_NAMES, PrayerIcon } from "../../components/icons";
import { AppIconTile } from "../../components/logo";
import { LiveCountdown } from "../panel/live-countdown";
import { PrayerRing } from "../panel/prayer-ring";

// Canonical display order — never sort the timeline by clock time.
const ORDER = ["fajr", "sunrise", "ishraq", "dhuhr", "asr", "maghrib", "isha"];
const orderOf = (p: string) => {
  const i = ORDER.indexOf(p);
  return i === -1 ? ORDER.length : i;
};
const FALLBACK_WINDOW_MS = 6 * 60 * 60 * 1000;

type Pane = "today" | "schedule" | "notifications" | "focus";

interface NavItem {
  id: Pane;
  label: string;
  Icon: LucideIcon;
}

const NAV: NavItem[] = [
  { id: "today", label: "Today", Icon: House },
  { id: "schedule", label: "Schedule", Icon: CalendarDays },
  { id: "notifications", label: "Notifications", Icon: Bell },
  { id: "focus", label: "Focus", Icon: Target },
];

/** The main "Today" dashboard window: nav rail + the active pane. */
export const MainWindow = () => {
  const [pane, setPane] = useState<Pane>("today");
  const state = usePrayerStore((s) => s.state);

  useEffect(() => initPrayerStore(), []);

  return (
    <div className="flex h-screen w-screen overflow-hidden bg-bg text-content">
      <NavRail pane={pane} onSelect={setPane} />
      <main className="flex-1 overflow-y-auto px-10 py-9 [scrollbar-width:none] [&::-webkit-scrollbar]:hidden">
        {pane === "today" && state ? (
          <Today state={state} />
        ) : (
          <Placeholder label={NAV.find((n) => n.id === pane)?.label ?? ""} />
        )}
      </main>
    </div>
  );
};

const NavRail = ({ pane, onSelect }: { pane: Pane; onSelect: (p: Pane) => void }) => (
  <nav className="flex w-[228px] shrink-0 flex-col border-r border-border bg-black/20 px-3.5 py-2">
    <div className="flex items-center gap-3 px-2.5 pb-5 pt-3.5">
      <AppIconTile size={28} className="rounded-[7px]" />
      <span className="font-display text-[15px] text-content">Prayer Times</span>
    </div>

    {NAV.map(({ id, label, Icon }) => {
      const active = pane === id;
      return (
        <button
          key={id}
          type="button"
          onClick={() => onSelect(id)}
          className={cn(
            "mb-0.5 flex items-center gap-3 rounded-[9px] px-3 py-2.5 text-[13.5px] transition-colors",
            active
              ? "bg-accent-soft font-semibold text-content"
              : "text-content-muted hover:bg-surface-hover hover:text-content",
          )}
        >
          {active ? (
            <span className="h-[7px] w-[7px] shrink-0 rounded-full bg-accent" />
          ) : (
            <Icon size={15} className="shrink-0" strokeWidth={1.75} />
          )}
          {label}
        </button>
      );
    })}

    <button
      type="button"
      onClick={() => void openSettings()}
      className="mt-auto flex items-center gap-3 rounded-[9px] px-3 py-2.5 text-[13.5px] text-content-muted transition-colors hover:bg-surface-hover hover:text-content"
    >
      <Gear size={15} className="shrink-0" strokeWidth={1.75} />
      Settings
    </button>
  </nav>
);

const Today = ({ state }: { state: PrayerState }) => {
  const { next, times, tz, now_ms, hijri_adjustment } = state;

  const rows = useMemo<PrayerInstant[]>(
    () => [...times].sort((a, b) => orderOf(a.prayer) - orderOf(b.prayer)),
    [times],
  );

  const fromMs = useMemo(() => {
    if (!next) return now_ms;
    const past = times.filter((t) => t.at_ms < next.at_ms).map((t) => t.at_ms);
    return past.length ? Math.max(...past) : next.at_ms - FALLBACK_WINDOW_MS;
  }, [times, next, now_ms]);

  // The prayer after "next", for the "ends at …" line.
  const after = useMemo<PrayerInstant | undefined>(() => {
    if (!next) return undefined;
    return rows.find((r) => r.at_ms > next.at_ms);
  }, [rows, next]);

  return (
    <>
      <header className="mb-8 flex items-start justify-between">
        <div>
          <div className="font-mono text-[10.5px] font-semibold tracking-[0.16em] text-content-subtle">
            {dateEyebrow(now_ms, tz)}
          </div>
          <div className="mt-1.5 font-display text-[30px] leading-none">
            {hijriDate({ ms: now_ms, tz, adjustmentDays: hijri_adjustment })}
          </div>
        </div>
        <div className="flex items-center gap-2 rounded-full border border-border bg-surface px-3.5 py-1.5 text-[12.5px] text-content-muted">
          <span className="h-[7px] w-[7px] rounded-full bg-success" />
          {state.latitude.toFixed(2)}, {state.longitude.toFixed(2)} · {tz}
        </div>
      </header>

      {next && (
        <section className="mb-7 flex items-center gap-8 rounded-2xl border border-border bg-[radial-gradient(130%_130%_at_85%_0%,var(--c-elevated),var(--c-surface)_60%)] px-9 py-8">
          <PrayerRing prayer={next.prayer} fromMs={fromMs} toMs={next.at_ms} size={124} />
          <div className="min-w-0 flex-1">
            <div className="text-[13px] text-content-muted">Up next</div>
            <div className="mt-1 whitespace-nowrap font-display text-[40px] leading-none">
              {PRAYER_NAMES[next.prayer] ?? next.prayer}{" "}
              <span className="italic text-accent">
                in <LiveCountdown targetMs={next.at_ms} variant="long" className="tabular-nums" />
              </span>
            </div>
            <div className="mt-3 text-[14px] text-content-muted">
              Begins {clock(next.at_ms, tz)}
              {after && ` · ends at ${PRAYER_NAMES[after.prayer] ?? after.prayer} ${clock(after.at_ms, tz)}`}
            </div>
          </div>
          <button
            type="button"
            onClick={() => void engageFocus()}
            className="shrink-0 rounded-[10px] bg-accent px-5 py-2.5 text-[13.5px] font-semibold text-accent-on transition-colors hover:bg-accent-emphasis"
          >
            Enter Focus
          </button>
        </section>
      )}

      <div className="flex flex-col gap-0.5">
        {rows.map((p) => {
          const isNext = !!next && p.prayer === next.prayer && p.at_ms === next.at_ms;
          const isPast = !!next && !isNext && p.at_ms < next.at_ms;
          return (
            <div
              key={`${p.prayer}-${p.at_ms}`}
              className={cn(
                "flex items-center gap-3.5 rounded-xl px-4 py-3",
                isNext && "border border-accent-ring bg-accent-soft",
              )}
            >
              <PrayerIcon
                prayer={p.prayer}
                size={17}
                strokeWidth={1.75}
                className={cn("shrink-0", {
                  "text-accent": isNext,
                  "text-content-subtle": isPast,
                  "text-content-muted": !isNext && !isPast,
                })}
              />
              <span
                className={cn("flex-1 text-[14px]", {
                  "font-semibold text-content": isNext,
                  "text-content-subtle line-through": isPast,
                  "text-content": !isNext && !isPast,
                })}
              >
                {PRAYER_NAMES[p.prayer] ?? p.prayer}
              </span>
              <span
                className={cn("font-mono text-[13.5px] tabular-nums", {
                  "font-semibold text-accent": isNext,
                  "text-content-subtle": isPast,
                  "text-content": !isNext && !isPast,
                })}
              >
                {clock(p.at_ms, tz)}
              </span>
            </div>
          );
        })}
      </div>
    </>
  );
};

const Placeholder = ({ label }: { label: string }) => (
  <div className="flex h-full flex-col items-center justify-center gap-2 text-center">
    <div className="font-display text-[26px] text-content">{label}</div>
    <div className="text-[14px] text-content-muted">This view arrives in a later update.</div>
  </div>
);
