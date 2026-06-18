import { useEffect, useMemo, useState } from "react";
import {
  checkForUpdates,
  getPrayerState,
  hidePanel,
  onStateChanged,
  openSettings,
  quitApp,
  type PrayerInstant,
  type PrayerState,
} from "../../lib/ipc";
import {
  clock,
  countdownLong,
  hijriDate,
  longDate,
  shortCountdown,
} from "../../lib/format";
import {
  GearIcon,
  MoonIcon,
  PinIcon,
  PowerIcon,
  PrayerIcon,
  PRAYER_NAMES,
  RefreshIcon,
} from "../../components/icons";

export default function Panel() {
  const [state, setState] = useState<PrayerState | null>(null);
  const [now, setNow] = useState(Date.now());

  // Hydrate + subscribe to schedule changes; run a local 1 Hz countdown.
  useEffect(() => {
    getPrayerState()
      .then(setState)
      .catch(() => {});
    const unlisten = onStateChanged(setState);
    const id = setInterval(() => setNow(Date.now()), 1000);
    return () => {
      unlisten.then((fn) => fn()).catch(() => {});
      clearInterval(id);
    };
  }, []);

  // Build the rendered list (optionally inserting Ishraq right after Sunrise).
  const rows = useMemo<PrayerInstant[]>(() => {
    if (!state) return [];
    if (!state.show_ishraq || state.ishraq_ms == null) return state.times;
    const out: PrayerInstant[] = [];
    for (const t of state.times) {
      out.push(t);
      if (t.prayer === "sunrise")
        out.push({ prayer: "ishraq", at_ms: state.ishraq_ms });
    }
    return out;
  }, [state]);

  const shell =
    "m-1.5 overflow-hidden rounded-[13px] border-[0.5px] border-white/12 bg-surface text-content shadow-[0_12px_40px_rgba(0,0,0,0.45)] backdrop-blur-[30px] backdrop-saturate-[1.8]";

  if (!state) return <div className={`${shell} min-h-[200px]`} />;

  const next = state.next;
  const secsToNext = next ? Math.max(0, (next.at_ms - now) / 1000) : 0;

  return (
    <div className={shell}>
      {/* Header: date, Hijri, next-prayer hero */}
      <div className="px-4 pb-3 pt-3.5">
        <div className="text-[11px] font-semibold tracking-[0.05em] text-content-muted">
          {longDate(now, state.tz).toUpperCase()}
        </div>
        {state.show_hijri && (
          <div className="mt-0.5 text-xs text-content-subtle">
            {hijriDate(now, state.tz, state.hijri_adjustment)}
          </div>
        )}

        {next && (
          <div className="mt-3 flex items-center gap-2.5">
            <PrayerIcon
              prayer={next.prayer}
              width={22}
              height={22}
              className="shrink-0 text-accent-emphasis"
            />
            <div className="flex-1">
              <div className="text-[22px] font-bold leading-tight tracking-[-0.01em]">
                {PRAYER_NAMES[next.prayer] ?? next.prayer}
              </div>
              <div className="mt-px text-[13px] tabular-nums text-content-muted">
                in {countdownLong(secsToNext)}
              </div>
            </div>
            <div className="text-lg font-semibold tabular-nums text-accent-emphasis">
              {clock(next.at_ms, state.tz)}
            </div>
          </div>
        )}
      </div>

      {/* Times list */}
      <div className="px-2 pb-2 pt-1">
        {rows.map((p) => {
          const isNext =
            !!next && p.prayer === next.prayer && p.at_ms === next.at_ms;
          const isPast = !isNext && p.at_ms <= now;
          const minor = p.prayer === "ishraq" || p.prayer === "sunrise";
          return (
            <div
              key={`${p.prayer}-${p.at_ms}`}
              className={[
                "flex items-center gap-2.5 rounded-lg px-2 py-[7px] text-[13.5px]",
                isNext && "bg-accent/[0.18]",
                isPast && "opacity-40",
              ]
                .filter(Boolean)
                .join(" ")}
            >
              <span
                className={`flex w-5 justify-center ${isNext ? "text-accent-emphasis" : "text-content-muted"}`}
              >
                <PrayerIcon prayer={p.prayer} />
              </span>
              <span
                className={[
                  "flex-1 font-medium",
                  isNext && "font-bold text-accent-emphasis",
                  minor && !isNext && "text-content-muted",
                ]
                  .filter(Boolean)
                  .join(" ")}
              >
                {PRAYER_NAMES[p.prayer] ?? p.prayer}
              </span>
              {isNext && (
                <span className="rounded-full bg-accent/[0.22] px-1.5 py-px text-[11px] font-semibold tabular-nums text-accent-emphasis">
                  {shortCountdown(secsToNext)}
                </span>
              )}
              <span
                className={[
                  "tabular-nums font-medium",
                  isNext
                    ? "font-bold text-accent-emphasis"
                    : minor
                      ? "text-content-muted"
                      : "text-content",
                ].join(" ")}
              >
                {clock(p.at_ms, state.tz)}
              </span>
            </div>
          );
        })}
      </div>

      {/* Summary: method + location */}
      <div className="h-px bg-border" />
      <div className="flex flex-col gap-1 px-4 py-2 text-xs text-content-muted">
        <div className="flex items-start gap-2">
          <MoonIcon
            width={13}
            height={13}
            className="mt-0.5 shrink-0 text-content-subtle"
          />
          <span>{state.method_name}</span>
        </div>
        <div className="flex items-start gap-2">
          <PinIcon
            width={13}
            height={13}
            className="mt-0.5 shrink-0 text-content-subtle"
          />
          <span>
            {state.latitude.toFixed(4)}, {state.longitude.toFixed(4)} ·{" "}
            {state.tz}
          </span>
        </div>
      </div>

      {/* Footer actions */}
      <div className="h-px bg-border" />
      <div className="flex flex-col p-1.5">
        <FooterButton
          onClick={() => openSettings().then(hidePanel)}
          icon={<GearIcon width={15} height={15} />}
        >
          Settings…
        </FooterButton>
        <FooterButton
          onClick={() => checkForUpdates()}
          icon={<RefreshIcon width={15} height={15} />}
        >
          Check for Updates…
        </FooterButton>
        <FooterButton
          onClick={() => quitApp()}
          icon={<PowerIcon width={15} height={15} />}
        >
          Quit
        </FooterButton>
      </div>
    </div>
  );
}

function FooterButton({
  onClick,
  icon,
  children,
}: {
  onClick: () => void;
  icon: React.ReactNode;
  children: React.ReactNode;
}) {
  return (
    <button
      onClick={onClick}
      className="flex w-full items-center gap-2.5 rounded-md px-2.5 py-[7px] text-left text-[13.5px] text-content [&>svg]:text-content-muted hover:bg-surface-hover"
    >
      {icon}
      {children}
    </button>
  );
}
