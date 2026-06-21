import { useEffect, useMemo, useState } from "react";
import { cn } from "../../lib/cn";
import { clock } from "../../lib/format";
import { completeOnboarding, ensureNotificationPermission } from "../../lib/ipc";
import type { PrayerInstant } from "../../lib/ipc";
import { useSettingsStore } from "../../stores/settings.store";
import { initPrayerStore, usePrayerStore } from "../../stores/prayer.store";
import { PRAYER_NAMES, PrayerIcon } from "../../components/icons";
import { AppIconTile } from "../../components/logo";
import { Toggle } from "../settings/controls";

const STEPS = 3;
const ORDER = ["fajr", "sunrise", "ishraq", "dhuhr", "asr", "maghrib", "isha"];
const orderOf = (p: string) => {
  const i = ORDER.indexOf(p);
  return i === -1 ? ORDER.length : i;
};

const Primary = ({ children, onClick, disabled }: ButtonProps) => (
  <button
    type="button"
    onClick={onClick}
    disabled={disabled}
    className="w-full rounded-[11px] bg-accent py-3.5 text-[15px] font-semibold text-accent-on transition-colors hover:bg-accent-emphasis disabled:opacity-50"
  >
    {children}
  </button>
);

const Tertiary = ({ children, onClick }: ButtonProps) => (
  <button
    type="button"
    onClick={onClick}
    className="py-1.5 text-[13.5px] text-content-muted transition-colors hover:text-content"
  >
    {children}
  </button>
);

interface ButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  disabled?: boolean;
}

/** First-launch wizard: Location → Notifications → Done. */
export const Onboarding = () => {
  const [step, setStep] = useState(0);
  const settings = useSettingsStore((s) => s.settings);
  const hydrate = useSettingsStore((s) => s.hydrate);

  useEffect(() => {
    void hydrate();
  }, [hydrate]);

  return (
    <div className="flex h-screen w-screen flex-col items-center bg-bg px-10 pb-12 pt-10 text-content">
      <div className="flex gap-1.5 self-start">
        {Array.from({ length: STEPS }, (_, i) => (
          <span
            key={i}
            className={cn("h-[3px] w-7 rounded-full", i <= step ? "bg-accent" : "bg-elevated")}
          />
        ))}
      </div>

      <div className="flex w-full max-w-[420px] flex-1 flex-col items-center justify-center text-center">
        {step === 0 && <LocationStep onNext={() => setStep(1)} />}
        {step === 1 && settings && (
          <NotificationsStep
            enabled={settings.masterNotificationsEnabled}
            onNext={() => setStep(2)}
          />
        )}
        {step === 2 && <DoneStep />}
      </div>
    </div>
  );
};

const Glyph = () => (
  <div
    className="mb-7 flex h-[70px] w-[70px] items-center justify-center rounded-full"
    style={{
      background: "radial-gradient(circle at 40% 35%, var(--c-elevated), var(--c-bg))",
      boxShadow: "0 0 60px 4px rgba(200,169,104,.15)",
    }}
  >
    <AppIconTile size={40} className="rounded-[26%] shadow-none" />
  </div>
);

const LocationStep = ({ onNext }: { onNext: () => void }) => {
  const detect = useSettingsStore((s) => s.detect);
  const update = useSettingsStore((s) => s.update);
  const [busy, setBusy] = useState(false);
  const [manual, setManual] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const useLocation = async () => {
    setError(null);
    setBusy(true);
    try {
      await detect();
      onNext();
    } catch (e) {
      setError(typeof e === "string" ? e : e instanceof Error ? e.message : "Couldn't detect location.");
    } finally {
      setBusy(false);
    }
  };

  if (manual) {
    return (
      <>
        <Glyph />
        <h1 className="font-display text-[34px] leading-tight">Enter your coordinates</h1>
        <p className="mt-3 text-[14px] text-content-muted">You can change these any time in Settings.</p>
        <div className="mt-7 flex w-full flex-col gap-3">
          <Coord label="Latitude" onChange={(latitude) => updateCoord(update, { latitude })} />
          <Coord label="Longitude" onChange={(longitude) => updateCoord(update, { longitude })} />
        </div>
        <div className="mt-8 flex w-full flex-col items-center gap-2">
          <Primary onClick={onNext}>Continue</Primary>
          <Tertiary onClick={() => setManual(false)}>Back</Tertiary>
        </div>
      </>
    );
  }

  return (
    <>
      <Glyph />
      <h1 className="font-display text-[42px] leading-[1.1] tracking-[-0.01em] text-content">
        Assalāmu <span className="italic text-accent">ʿalaykum</span>.
      </h1>
      <p className="mt-3 text-[15px] text-content-muted">Let's find your times.</p>
      <p className="mt-4 max-w-[360px] text-[15px] leading-relaxed text-content-muted">
        We'll detect your location and set sensible defaults. You'll be done in under a minute —
        everything is changeable later.
      </p>
      {error && <p className="mt-4 text-[13px] text-error">{error}</p>}
      <div className="mt-9 flex w-full flex-col items-center gap-2">
        <Primary onClick={useLocation} disabled={busy}>
          {busy ? "Locating…" : "Use my location"}
        </Primary>
        <Tertiary onClick={() => setManual(true)}>Enter it manually</Tertiary>
      </div>
    </>
  );
};

const updateCoord = (
  update: (patch: { manualCoordinates: { latitude: number; longitude: number; elevation: number } }) => void,
  patch: { latitude?: number; longitude?: number },
) => {
  const current = useSettingsStore.getState().settings?.manualCoordinates ?? {
    latitude: 0,
    longitude: 0,
    elevation: 0,
  };
  update({ manualCoordinates: { ...current, ...patch } });
};

const Coord = ({ label, onChange }: { label: string; onChange: (v: number) => void }) => (
  <label className="flex items-center justify-between rounded-lg border border-border bg-surface px-4 py-2.5 text-[13.5px]">
    <span className="text-content-muted">{label}</span>
    <input
      type="number"
      step={0.0001}
      defaultValue={0}
      onChange={(e) => onChange(Number(e.target.value))}
      className="w-[120px] bg-transparent text-right tabular-nums text-content outline-none"
    />
  </label>
);

const NotificationsStep = ({ enabled, onNext }: { enabled: boolean; onNext: () => void }) => {
  const update = useSettingsStore((s) => s.update);
  const [warning, setWarning] = useState<string | null>(null);

  const setEnabled = async (next: boolean) => {
    update({ masterNotificationsEnabled: next });
    setWarning(null);
    if (!next) return;
    try {
      const granted = await ensureNotificationPermission();
      if (!granted) {
        setWarning("macOS blocked notifications. You can enable them later in System Settings.");
      }
    } catch (err) {
      setWarning(typeof err === "string" ? err : err instanceof Error ? err.message : "Permission request failed.");
    }
  };

  return (
    <>
      <Glyph />
      <h1 className="font-display text-[34px] leading-tight">Never miss a prayer.</h1>
      <p className="mt-4 max-w-[360px] text-[15px] leading-relaxed text-content-muted">
        Get a gentle reminder and the Adhan at each prayer time. You can fine-tune every prayer
        later.
      </p>
      <div className="mt-8 flex w-full items-center justify-between rounded-xl border border-border bg-surface px-4 py-3.5">
        <span className="text-[14px]">Enable prayer notifications</span>
        <Toggle checked={enabled} onChange={setEnabled} />
      </div>
      {warning && <p className="mt-3 text-[12.5px] text-content-subtle">{warning}</p>}
      <div className="mt-9 w-full">
        <Primary onClick={onNext}>Continue</Primary>
      </div>
    </>
  );
};

const DoneStep = () => {
  const state = usePrayerStore((s) => s.state);
  useEffect(() => initPrayerStore(), []);

  const rows = useMemo<PrayerInstant[]>(
    () => (state ? [...state.times].sort((a, b) => orderOf(a.prayer) - orderOf(b.prayer)) : []),
    [state],
  );

  return (
    <>
      <Glyph />
      <h1 className="font-display text-[34px] leading-tight">You're all set.</h1>
      <p className="mt-3 text-[14px] text-content-muted">Here are today's times for your location.</p>

      {state && (
        <div className="mt-6 w-full rounded-xl border border-border bg-surface px-4 py-2">
          {rows.map((p) => {
            const isNext = !!state.next && p.prayer === state.next.prayer && p.at_ms === state.next.at_ms;
            return (
              <div key={`${p.prayer}-${p.at_ms}`} className="flex items-center gap-3 py-1.5">
                <PrayerIcon
                  prayer={p.prayer}
                  size={14}
                  strokeWidth={1.75}
                  className={cn("shrink-0", isNext ? "text-accent" : "text-content-muted")}
                />
                <span className={cn("flex-1 text-left text-[13px]", isNext && "font-semibold text-accent")}>
                  {PRAYER_NAMES[p.prayer] ?? p.prayer}
                </span>
                <span
                  className={cn("font-mono text-[12.5px] tabular-nums", isNext ? "text-accent" : "text-content")}
                >
                  {clock(p.at_ms, state.tz)}
                </span>
              </div>
            );
          })}
        </div>
      )}

      <div className="mt-8 w-full">
        <Primary onClick={() => void completeOnboarding()}>Start using Prayer Times</Primary>
      </div>
    </>
  );
};
