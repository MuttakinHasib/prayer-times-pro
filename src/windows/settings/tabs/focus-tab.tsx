import type { AppSettings, FocusBlurIntensity, FocusTrigger } from "../../../lib/settings";
import { engageFocus } from "../../../lib/ipc";
import { Note, Row, Section, SelectField, Stepper, Toggle } from "../controls";

interface Props {
  settings: AppSettings;
  update: (patch: Partial<AppSettings>) => void;
}

const BLUR_OPTIONS: { value: FocusBlurIntensity; label: string }[] = [
  { value: "low", label: "Low" },
  { value: "medium", label: "Medium" },
  { value: "high", label: "High" },
  { value: "opaque", label: "Opaque" },
];

const TRIGGER_OPTIONS: { value: FocusTrigger; label: string }[] = [
  { value: "obligatory", label: "Obligatory prayers" },
  { value: "all", label: "All prayer times" },
  { value: "fajrIsha", label: "Fajr & Isha only" },
];

export const FocusTab = ({ settings, update }: Props) => (
  <>
    <Section title="Focus Mode">
      <Row label="Enable Focus Mode" sublabel="Covers the screen for a short while at prayer time.">
        <Toggle
          checked={settings.focusModeEnabled}
          onChange={(focusModeEnabled) => update({ focusModeEnabled })}
        />
      </Row>
    </Section>

    <Section title="Behaviour">
      <Row label="Prayer duration">
        <Stepper
          value={settings.focusDurationMinutes}
          min={1}
          max={60}
          format={(v) => `${v} min`}
          onChange={(focusDurationMinutes) => update({ focusDurationMinutes })}
        />
      </Row>
      <Row label="Blur intensity">
        <SelectField
          value={settings.focusBlurIntensity}
          options={BLUR_OPTIONS}
          onChange={(focusBlurIntensity) => update({ focusBlurIntensity })}
        />
      </Row>
      <Row label="Trigger on">
        <SelectField
          value={settings.focusTrigger}
          options={TRIGGER_OPTIONS}
          onChange={(focusTrigger) => update({ focusTrigger })}
        />
      </Row>
      <Row label="Emergency exit" sublabel="Allow ⌘⎋ to exit early.">
        <Toggle
          checked={settings.focusEmergencyExitEnabled}
          onChange={(focusEmergencyExitEnabled) => update({ focusEmergencyExitEnabled })}
        />
      </Row>
      <Row label="Try it" sublabel="Preview the overlay with the next prayer.">
        <button
          type="button"
          onClick={() => void engageFocus()}
          className="rounded-md border border-border px-3 py-1 text-[12.5px] font-medium text-content transition-colors hover:bg-surface-hover"
        >
          Preview
        </button>
      </Row>
    </Section>
    <Note>
      Focus Mode is a discipline aid, not a lock — "I've prayed" always exits, and Force Quit always
      works.
    </Note>
  </>
);
