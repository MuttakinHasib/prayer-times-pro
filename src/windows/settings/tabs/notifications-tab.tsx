import { useState } from "react";
import {
  type AppSettings,
  type NotificationSound,
  type PrayerKey,
  type PrayerNotificationConfig,
} from "../../../lib/settings";
import { cn } from "../../../lib/cn";
import { sendTestNotification } from "../../../lib/ipc";
import { PRAYER_NAMES } from "../../../components/icons";
import { Note, Row, Section, SelectField, Stepper, Toggle } from "../controls";

interface Props {
  settings: AppSettings;
  update: (patch: Partial<AppSettings>) => void;
}

const ALL_PRAYERS: PrayerKey[] = ["fajr", "sunrise", "dhuhr", "asr", "maghrib", "isha"];

const SOUND_OPTIONS: { value: NotificationSound; label: string }[] = [
  { value: "none", label: "None" },
  { value: "systemDefault", label: "Default" },
  { value: "softChime", label: "Soft chime" },
  { value: "takbir", label: "Takbir" },
  { value: "adhanMakkah", label: "Adhan (Makkah)" },
  { value: "adhanMadinah", label: "Adhan (Madinah)" },
];

// Per-prayer override prepends "(default)" so users can fall back to the global default.
const PER_PRAYER_SOUND_OPTIONS: { value: NotificationSound | "default"; label: string }[] = [
  { value: "default", label: "Default" },
  ...SOUND_OPTIONS.filter((o) => o.value !== "systemDefault").map((o) => ({
    value: o.value,
    label: o.label,
  })),
];

const DEFAULT_CONFIG: PrayerNotificationConfig = {
  notify: true,
  playFullAdhan: false,
  earlyReminderEnabled: false,
  soundOverride: null,
  earlyLeadMinutesOverride: null,
  iqamahOffsetMinutesOverride: null,
};

export const NotificationsTab = ({ settings, update }: Props) => {
  const defaults = settings.notificationDefaults;
  const setDefaults = (patch: Partial<typeof defaults>) =>
    update({ notificationDefaults: { ...defaults, ...patch } });

  const cfg = (p: PrayerKey) => settings.notifications[p] ?? DEFAULT_CONFIG;
  const setCfg = (p: PrayerKey, patch: Partial<PrayerNotificationConfig>) =>
    update({ notifications: { ...settings.notifications, [p]: { ...cfg(p), ...patch } } });

  const [sampleSent, setSampleSent] = useState(false);
  const [sampleError, setSampleError] = useState<string | null>(null);
  const sendSample = async () => {
    setSampleError(null);
    try {
      await sendTestNotification();
      setSampleSent(true);
      setTimeout(() => setSampleSent(false), 2500);
    } catch (err) {
      setSampleError(typeof err === "string" ? err : err instanceof Error ? err.message : "Send failed.");
    }
  };

  return (
    <>
      <Section title="Notifications">
        <Row label="Enable prayer notifications" sublabel="Master switch for all prayer alerts.">
          <Toggle
            checked={settings.masterNotificationsEnabled}
            onChange={(masterNotificationsEnabled) => update({ masterNotificationsEnabled })}
          />
        </Row>
        <Row label="Send a sample" sublabel="Verify notifications are working.">
          <button
            type="button"
            onClick={sendSample}
            className="rounded-md border border-border px-3 py-1 text-[12.5px] font-medium text-content transition-colors hover:bg-surface-hover"
          >
            {sampleSent ? "Sent ✓" : "Send"}
          </button>
        </Row>
      </Section>
      {sampleError && <Note>{sampleError}</Note>}

      <Section title="Defaults">
        <Row label="Default sound">
          <SelectField
            value={defaults.sound}
            options={SOUND_OPTIONS}
            onChange={(sound) => setDefaults({ sound })}
          />
        </Row>
        <Row label="Play full Adhan audio">
          <Toggle
            checked={defaults.playFullAdhan}
            onChange={(playFullAdhan) => setDefaults({ playFullAdhan })}
          />
        </Row>
        <Row label="Early reminder">
          <Stepper
            value={defaults.earlyReminderMinutes}
            min={0}
            max={60}
            format={(v) => (v === 0 ? "Off" : `${v} min`)}
            onChange={(earlyReminderMinutes) => setDefaults({ earlyReminderMinutes })}
          />
        </Row>
        <Row label="Iqamah / jamaat offset">
          <Stepper
            value={defaults.iqamahOffsetMinutes}
            min={0}
            max={60}
            format={(v) => (v === 0 ? "Off" : `${v} min`)}
            onChange={(iqamahOffsetMinutes) => setDefaults({ iqamahOffsetMinutes })}
          />
        </Row>
      </Section>
      <Note>Applied to every prayer unless overridden per prayer below.</Note>

      <Section title="Per prayer">
        {ALL_PRAYERS.map((p) => (
          <PerPrayerRow key={p} prayer={p} config={cfg(p)} onChange={(patch) => setCfg(p, patch)} />
        ))}
      </Section>
    </>
  );
};

const Labeled = ({ label, children }: { label: string; children: React.ReactNode }) => (
  <div className="flex flex-col items-center gap-1">
    <span className="text-[10px] uppercase tracking-wide text-content-subtle">{label}</span>
    {children}
  </div>
);

interface PerPrayerRowProps {
  prayer: PrayerKey;
  config: PrayerNotificationConfig;
  onChange: (patch: Partial<PrayerNotificationConfig>) => void;
}

/** One prayer's notify/adhan/remind toggles, with a disclosure for per-prayer overrides. */
const PerPrayerRow = ({ prayer, config: c, onChange }: PerPrayerRowProps) => {
  const obligatory = prayer !== "sunrise";
  const [open, setOpen] = useState(false);
  const hasOverrides =
    c.soundOverride != null || c.earlyLeadMinutesOverride != null || c.iqamahOffsetMinutesOverride != null;

  return (
    <div className="py-2">
      <div className="flex items-center justify-between gap-3">
        <button
          type="button"
          onClick={() => setOpen((o) => !o)}
          className="flex items-center gap-1.5 text-left text-[13.5px] text-content"
        >
          <span className="font-medium">{PRAYER_NAMES[prayer]}</span>
          <span
            className={cn("text-[10px] text-content-subtle transition-transform", {
              "rotate-90": open,
            })}
          >
            ▸
          </span>
          {hasOverrides && !open && (
            <span className="ml-1 rounded-full bg-accent-soft px-1.5 text-[9px] font-semibold text-accent">
              custom
            </span>
          )}
        </button>
        <div className="flex items-center gap-4">
          <Labeled label="Notify">
            <Toggle checked={c.notify} onChange={(notify) => onChange({ notify })} />
          </Labeled>
          <Labeled label="Adhan">
            <Toggle
              checked={obligatory && c.playFullAdhan}
              disabled={!obligatory}
              onChange={(playFullAdhan) => obligatory && onChange({ playFullAdhan })}
            />
          </Labeled>
          <Labeled label="Remind">
            <Toggle
              checked={c.earlyReminderEnabled}
              onChange={(earlyReminderEnabled) => onChange({ earlyReminderEnabled })}
            />
          </Labeled>
        </div>
      </div>

      {open && (
        <div className="mt-3 flex flex-col gap-2 rounded-md border border-divider bg-elevated/60 px-3 py-2.5 text-[12.5px]">
          <OverrideRow label="Sound">
            <SelectField
              value={c.soundOverride ?? "default"}
              options={PER_PRAYER_SOUND_OPTIONS}
              onChange={(v) =>
                onChange({ soundOverride: v === "default" ? null : (v as NotificationSound) })
              }
            />
          </OverrideRow>
          {c.earlyReminderEnabled && (
            <OverrideRow label="Reminder lead">
              <Stepper
                value={c.earlyLeadMinutesOverride ?? 0}
                min={0}
                max={60}
                format={(v) => (v === 0 ? "Use default" : `${v} min`)}
                onChange={(v) => onChange({ earlyLeadMinutesOverride: v === 0 ? null : v })}
              />
            </OverrideRow>
          )}
          {obligatory && (
            <OverrideRow label="Iqamah offset">
              <Stepper
                value={c.iqamahOffsetMinutesOverride ?? 0}
                min={0}
                max={60}
                format={(v) => (v === 0 ? "Use default" : `${v} min`)}
                onChange={(v) => onChange({ iqamahOffsetMinutesOverride: v === 0 ? null : v })}
              />
            </OverrideRow>
          )}
        </div>
      )}
    </div>
  );
};

const OverrideRow = ({ label, children }: { label: string; children: React.ReactNode }) => (
  <div className="flex items-center justify-between gap-3">
    <span className="text-content-muted">{label}</span>
    {children}
  </div>
);
