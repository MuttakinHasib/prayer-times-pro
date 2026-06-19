import { memo } from "react";
import type { PrayerState } from "../../lib/ipc";

interface PanelSummaryProps {
  state: PrayerState;
}

/** Active method + resolved location/timezone, set quietly in tertiary text. */
export const PanelSummary = memo(({ state }: PanelSummaryProps) => (
  <div className="px-[22px] py-[14px] text-[11.5px] leading-relaxed text-content-subtle">
    <div>{state.method_name}</div>
    <div>
      {state.latitude.toFixed(2)}, {state.longitude.toFixed(2)} · {state.tz}
    </div>
  </div>
));
PanelSummary.displayName = "PanelSummary";
