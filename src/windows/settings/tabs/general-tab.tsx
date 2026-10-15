import type {
  AppSettings,
  AppearanceTheme,
  MenuBarStyle,
  MenuBarCountdownMode,
} from "../../../lib/settings";
import { openOnboarding } from "../../../lib/ipc";
import { Note, Row, Section, Segmented, SelectField, Toggle } from "../controls";

interface Props {
  settings: AppSettings;
  update: (patch: Partial<AppSettings>) => void;
}

const STYLE_OPTIONS: { value: MenuBarStyle; label: string }[] = [
  { value: "iconOnly", label: "Icon only" },
  { value: "countdownOnly", label: "Countdown only" },
  { value: "iconCountdown", label: "Icon + countdown" },
  { value: "nextPrayerCountdown", label: "Name + countdown" },
  { value: "iconNameCountdown", label: "Icon + name + countdown" },
  { value: "nextPrayerClock", label: "Name + time" },
  { value: "iconNameClock", label: "Icon + name + time" },
];

const COUNTDOWN_OPTIONS: { value: MenuBarCountdownMode; label: string }[] = [
  { value: "nextPrayer", label: "Next prayer" },
  { value: "currentWaqt", label: "Time left in current prayer" },
];

const APPEARANCE_OPTIONS: { value: AppearanceTheme; label: string }[] = [
  { value: "dark", label: "Dark" },
  { value: "light", label: "Light" },
  { value: "auto", label: "Auto" },
];

export const GeneralTab = ({ settings, update }: Props) => (
  <>
    <Section title="Appearance">
      <Row label="Theme" sublabel="Auto follows your system setting.">
        <Segmented
          value={settings.appearance}
          options={APPEARANCE_OPTIONS}
          onChange={(appearance) => update({ appearance })}
        />
      </Row>
    </Section>

    <Section title="Startup">
      <Row label="Launch at login" sublabel="Open Prayer Times when you sign in.">
        <Toggle
          checked={settings.launchAtLogin}
          onChange={(launchAtLogin) => update({ launchAtLogin })}
        />
      </Row>
    </Section>

    <Section title="Menu bar">
      <Row label="Label style">
        <SelectField
          value={settings.menuBarStyle}
          options={STYLE_OPTIONS}
          onChange={(menuBarStyle) => update({ menuBarStyle })}
        />
      </Row>
      <Row label="Countdown shows">
        <SelectField
          value={settings.menuBarCountdownMode}
          options={COUNTDOWN_OPTIONS}
          onChange={(menuBarCountdownMode) => update({ menuBarCountdownMode })}
        />
      </Row>
    </Section>

    <Section title="Panel">
      <Row label="Show Ishraq time">
        <Toggle
          checked={settings.showIshraqTime}
          onChange={(showIshraqTime) => update({ showIshraqTime })}
        />
      </Row>
      <Row label="Show Hijri date">
        <Toggle
          checked={settings.showHijriDate}
          onChange={(showHijriDate) => update({ showHijriDate })}
        />
      </Row>
    </Section>

    <Section title="Setup">
      <Row label="Setup wizard" sublabel="Re-run the first-launch flow.">
        <button
          type="button"
          onClick={() => void openOnboarding()}
          className="rounded-md border border-border px-3 py-1 text-[12.5px] font-medium text-content transition-colors hover:bg-surface-hover"
        >
          Run setup again
        </button>
      </Row>
    </Section>

    <Section title="Updates">
      <Row label="Check for updates automatically">
        <Toggle
          checked={settings.autoUpdateEnabled}
          onChange={(autoUpdateEnabled) => update({ autoUpdateEnabled })}
        />
      </Row>
    </Section>
    <Note>The updater itself ships with packaging; this preference is honoured then.</Note>
  </>
);
