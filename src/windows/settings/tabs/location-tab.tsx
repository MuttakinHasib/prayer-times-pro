import type { AppSettings, Coordinates, LocationMode } from "../../../lib/settings";
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
  const coords = settings.manualCoordinates ?? DEFAULT_COORDS;
  const setCoords = (patch: Partial<Coordinates>) =>
    update({ manualCoordinates: { ...coords, ...patch } });

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
      </Section>
      {settings.locationMode === "automatic" && (
        <Note>Auto-detect arrives in a later milestone; enter coordinates manually for now.</Note>
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
