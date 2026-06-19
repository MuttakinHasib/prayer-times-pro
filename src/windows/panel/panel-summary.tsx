import type { PrayerState } from "../../lib/ipc";
import { MoonIcon, PinIcon } from "../../components/icons";

/** Active method + resolved location/timezone. */
export const PanelSummary = ({ state }: { state: PrayerState }) => {
  return (
    <div className="flex flex-col gap-1 px-4 py-2 text-[11px] text-content-muted">
      <div className="flex items-start gap-2">
        <MoonIcon size={13} className="mt-px shrink-0 text-content-subtle" />
        <span>{state.method_name}</span>
      </div>
      <div className="flex items-start gap-2">
        <PinIcon size={13} className="mt-px shrink-0 text-content-subtle" />
        <span>
          {state.latitude.toFixed(4)}, {state.longitude.toFixed(4)} · {state.tz}
        </span>
      </div>
    </div>
  );
};
