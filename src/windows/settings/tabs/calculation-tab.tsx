import {
  METHODS,
  OBLIGATORY,
  type AppSettings,
  type CalculationMode,
  type HighLatitudeRule,
  type PrayerKey,
} from "../../../lib/settings";
import { PRAYER_NAMES } from "../../../components/icons";
import { Note, Row, Section, Segmented, SelectField, Stepper, Toggle } from "../controls";

interface Props {
  settings: AppSettings;
  update: (patch: Partial<AppSettings>) => void;
}

const SOURCE_OPTIONS: { value: CalculationMode; label: string }[] = [
  { value: "calculated", label: "Calculated" },
  { value: "manual", label: "Manual (fixed)" },
];

const HIGH_LAT_OPTIONS: { value: HighLatitudeRule; label: string }[] = [
  { value: "automatic", label: "Automatic (recommended)" },
  { value: "none", label: "None" },
  { value: "middleOfNight", label: "Middle of the night" },
  { value: "seventhOfNight", label: "One-seventh of the night" },
  { value: "angleBased", label: "Angle-based" },
];

const toClock = (minutes: number) =>
  `${String(Math.floor(minutes / 60)).padStart(2, "0")}:${String(minutes % 60).padStart(2, "0")}`;
const fromClock = (value: string) => {
  const [h, m] = value.split(":").map(Number);
  return (h || 0) * 60 + (m || 0);
};

export const CalculationTab = ({ settings, update }: Props) => {
  const setJamaat = (prayer: PrayerKey, minutes: number) =>
    update({ jamaatTimes: { ...settings.jamaatTimes, [prayer]: minutes } });

  return (
    <>
      <Section title="Source">
        <Row label="Time source">
          <Segmented
            value={settings.calculationMode}
            options={SOURCE_OPTIONS}
            onChange={(calculationMode) => update({ calculationMode })}
          />
        </Row>
      </Section>

      {settings.calculationMode === "calculated" ? (
        <Section title="Method">
          <Row label="Calculation method">
            <SelectField
              value={settings.methodId}
              options={METHODS.map((m) => ({ value: m.id, label: m.name }))}
              onChange={(methodId) => update({ methodId })}
            />
          </Row>
          <Row label="Hanafi Asr (madhab)" sublabel="Later afternoon Asr (shadow ×2).">
            <Toggle checked={settings.hanafiAsr} onChange={(hanafiAsr) => update({ hanafiAsr })} />
          </Row>
          <Row label="High-latitude rule">
            <SelectField
              value={settings.highLatitudeRule}
              options={HIGH_LAT_OPTIONS}
              onChange={(highLatitudeRule) => update({ highLatitudeRule })}
            />
          </Row>
          <Row
            label="Auto-detect method from location"
            sublabel="Pick a regional method when detecting location."
          >
            <Toggle
              checked={settings.autoDetectMethod}
              onChange={(autoDetectMethod) => update({ autoDetectMethod })}
            />
          </Row>
        </Section>
      ) : (
        <>
          <Section title="Jamaat schedule">
            {OBLIGATORY.map((prayer) => (
              <Row key={prayer} label={PRAYER_NAMES[prayer]}>
                <input
                  type="time"
                  value={toClock(settings.jamaatTimes[prayer] ?? 0)}
                  onChange={(e) => setJamaat(prayer, fromClock(e.target.value))}
                  className="rounded-md border border-white/10 bg-white/10 px-2 py-1 text-[12.5px] text-content"
                />
              </Row>
            ))}
          </Section>
          <Section title="Adhan timing">
            <Row label="Adhan before jamaat">
              <Stepper
                value={settings.azanBeforeJamaat}
                min={0}
                max={60}
                format={(v) => `${v} min`}
                onChange={(azanBeforeJamaat) => update({ azanBeforeJamaat })}
              />
            </Row>
            <Row label="Keep waqt for Sunrise & windows" sublabel="Astronomical times for non-jamaat events.">
              <Toggle
                checked={settings.manualKeepWaqt}
                onChange={(manualKeepWaqt) => update({ manualKeepWaqt })}
              />
            </Row>
          </Section>
          <Note>Enter the times your mosque announces; the panel uses them directly.</Note>
        </>
      )}
    </>
  );
};
