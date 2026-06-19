import { useState } from "react";
import type { AppSettings, Coordinates, LocationMode } from "../../../lib/settings";
import { useSettingsStore } from "../../../stores/settings.store";
import { Note, Row, Section, Segmented, Stepper } from "../controls";

interface Props {
  settings: AppSettings;
  update: (patch: Partial<AppSettings>) => void;
}

const LOCATION_OPTIONS: { value: LocationMode; label: string }[] = [
  { value: "automatic", label: "Automatic" },
  { value: "manual", label: "Manual" },
];

const TZ_OPTIONS: { value: "system" | "explicit"; label: string }[] = [
  { value: "system", label: "Follow system" },
  { value: "explicit", label: "Pick explicitly" },
];

const DEFAULT_COORDS: Coordinates = { latitude: 23.8103, longitude: 90.4125, elevation: 0 };

const Num = ({
  value,
  step = 0.0001,
  onChange,
}: {
  value: number;
  step?: number;
  onChange: (v: number) => void;
}) => (
  <input
    type="number"
    value={value}
    step={step}
    onChange={(e) => onChange(Number(e.target.value))}
    className="w-[110px] rounded-md border border-white/10 bg-white/10 px-2 py-1 text-right text-[12.5px] tabular-nums text-content"
  />
);

export const LocationTab = ({ settings, update }: Props) => {
  const detect = useSettingsStore((s) => s.detect);
  const [detecting, setDetecting] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const coords = settings.manualCoordinates ?? DEFAULT_COORDS;
  const setCoords = (patch: Partial<Coordinates>) =>
    update({ manualCoordinates: { ...coords, ...patch } });

  const runDetect = async () => {
    setError(null);
    setDetecting(true);
    try {
      await detect();
    } catch (e) {
      // Tauri rejects Result<_, String> with the bare string; also handle Error objects.
      const message = typeof e === "string" ? e : e instanceof Error ? e.message : String(e);
      setError(message || "Couldn't detect location.");
    } finally {
      setDetecting(false);
    }
  };

  return (
    <>
      <Section title="Location">
        <Row label="Mode">
          <Segmented
            value={settings.locationMode}
            options={LOCATION_OPTIONS}
            onChange={(locationMode) => update({ locationMode })}
          />
        </Row>
        <Row label="Latitude">
          <Num value={coords.latitude} onChange={(latitude) => setCoords({ latitude })} />
        </Row>
        <Row label="Longitude">
          <Num value={coords.longitude} onChange={(longitude) => setCoords({ longitude })} />
        </Row>
        <Row label="Elevation (m)">
          <Num value={coords.elevation} step={1} onChange={(elevation) => setCoords({ elevation })} />
        </Row>
        {settings.locationMode === "automatic" && (
          <Row label="Detect" sublabel="Use your IP to fill coordinates and timezone.">
            <button
              type="button"
              onClick={runDetect}
              disabled={detecting}
              className="rounded-md bg-accent px-3 py-1 text-[12.5px] font-medium text-accent-on transition-colors hover:bg-accent-emphasis disabled:opacity-50"
            >
              {detecting ? "Detecting…" : "Detect location"}
            </button>
          </Row>
        )}
      </Section>
      {settings.locationMode === "automatic" && (
        <Note>
          {error
            ? error
            : settings.autoDetectMethod
              ? "Detect fills coordinates, timezone, and a regional calculation method."
              : "Detect fills coordinates and timezone. Enable method auto-detect in Calculation."}
        </Note>
      )}

      <Section title="Timezone">
        <Row label="Master timezone">
          <Segmented
            value={settings.timezoneOverride == null ? "system" : "explicit"}
            options={TZ_OPTIONS}
            onChange={(mode) =>
              update({ timezoneOverride: mode === "system" ? null : "Asia/Dhaka" })
            }
          />
        </Row>
        {settings.timezoneOverride != null && (
          <Row label="IANA identifier">
            <input
              type="text"
              value={settings.timezoneOverride}
              onChange={(e) => update({ timezoneOverride: e.target.value })}
              className="w-[160px] rounded-md border border-white/10 bg-white/10 px-2 py-1 text-[12.5px] text-content"
            />
          </Row>
        )}
      </Section>

      <Section title="Hijri date">
        <Row label="Day adjustment" sublabel="Shift the displayed Hijri date by whole days.">
          <Stepper
            value={settings.hijriDayAdjustment}
            min={-3}
            max={3}
            format={(v) => (v > 0 ? `+${v}` : `${v}`)}
            onChange={(hijriDayAdjustment) => update({ hijriDayAdjustment })}
          />
        </Row>
      </Section>
    </>
  );
};
