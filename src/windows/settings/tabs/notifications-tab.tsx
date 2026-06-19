import {
  type AppSettings,
  type NotificationSound,
  type PrayerKey,
  type PrayerNotificationConfig,
} from "../../../lib/settings";
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

  return (
    <>
      <Section title="Notifications">
        <Row label="Enable prayer notifications" sublabel="Master switch for all prayer alerts.">
          <Toggle
            checked={settings.masterNotificationsEnabled}
            onChange={(masterNotificationsEnabled) => update({ masterNotificationsEnabled })}
          />
        </Row>
      </Section>

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
        {ALL_PRAYERS.map((p) => {
          const c = cfg(p);
          const obligatory = p !== "sunrise";
          return (
            <Row key={p} label={PRAYER_NAMES[p]}>
              <div className="flex items-center gap-4">
                <Labeled label="Notify">
                  <Toggle checked={c.notify} onChange={(notify) => setCfg(p, { notify })} />
                </Labeled>
                <Labeled label="Adhan">
                  <Toggle
                    checked={obligatory && c.playFullAdhan}
                    onChange={(playFullAdhan) => obligatory && setCfg(p, { playFullAdhan })}
                  />
                </Labeled>
                <Labeled label="Remind">
                  <Toggle
                    checked={c.earlyReminderEnabled}
                    onChange={(earlyReminderEnabled) => setCfg(p, { earlyReminderEnabled })}
                  />
                </Labeled>
              </div>
            </Row>
          );
        })}
      </Section>
      <Note>Adhan and reminder scheduling is wired up in a later milestone.</Note>
    </>
  );
};

const Labeled = ({ label, children }: { label: string; children: React.ReactNode }) => (
  <div className="flex flex-col items-center gap-1">
    <span className="text-[10px] uppercase tracking-wide text-content-subtle">{label}</span>
    {children}
  </div>
);
