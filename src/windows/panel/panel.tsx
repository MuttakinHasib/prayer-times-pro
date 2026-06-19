import { useEffect } from "react";
import { cn } from "../../lib/cn";
import { usePanelAutoSize } from "../../hooks/use-panel-auto-size";
import { initPrayerStore, usePrayerStore } from "../../stores/prayer.store";
import { PanelHeader } from "./panel-header";
import { PrayerList } from "./prayer-list";
import { PanelSummary } from "./panel-summary";
import { PanelFooter } from "./panel-footer";

const SHELL =
  "w-[312px] overflow-hidden rounded-[12px] border-[0.5px] border-white/10 bg-surface text-content backdrop-blur-[30px] backdrop-saturate-[1.8]";

/** The menu-bar dropdown: a dark glass card composed of header / list / summary / footer. */
export const Panel = () => {
  const ref = usePanelAutoSize();
  const state = usePrayerStore((s) => s.state);

  useEffect(() => {
    // initPrayerStore returns its unsubscribe fn → becomes the effect cleanup.
    return initPrayerStore();
  }, []);

  if (!state) return <div ref={ref} className={cn(SHELL, "min-h-[200px]")} />;

  return (
    <div ref={ref} className={SHELL}>
      <PanelHeader state={state} />
      <PrayerList state={state} />
      <div className="h-px bg-border" />
      <PanelSummary state={state} />
      <div className="h-px bg-border" />
      <PanelFooter />
    </div>
  );
};
