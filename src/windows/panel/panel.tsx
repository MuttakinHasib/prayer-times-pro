import { useEffect } from "react";
import { cn } from "../../lib/cn";
import { usePanelAutoSize } from "../../hooks/use-panel-auto-size";
import { initPrayerStore, usePrayerStore } from "../../stores/prayer.store";
import { PanelHeader } from "./panel-header";
import { PrayerList } from "./prayer-list";
import { PanelSummary } from "./panel-summary";
import { PanelFooter } from "./panel-footer";

// No outer drop-shadow here: the window is sized to the card, so a CSS shadow
// would paint into the square corner gaps. The native NSPanel shadow handles it.
const SHELL =
  "relative w-[336px] overflow-hidden rounded-[16px] border border-white/[0.08] text-content shadow-[inset_0_1px_0_rgba(255,255,255,.06)]";

// The blur lives on its own static layer behind the content. WKWebView re-rasterizes
// a backdrop-filter whenever a descendant repaints, so keeping the per-second
// countdown/ring out of this layer stops the 1 Hz flicker.
const BACKDROP = "pointer-events-none absolute inset-0 bg-popover backdrop-blur-[40px] backdrop-saturate-[1.4]";

/** The menu-bar dropdown: a dark glass card composed of header / list / summary / footer. */
export const Panel = () => {
  const ref = usePanelAutoSize();
  const state = usePrayerStore((s) => s.state);

  useEffect(() => {
    // initPrayerStore returns its unsubscribe fn → becomes the effect cleanup.
    return initPrayerStore();
  }, []);

  if (!state) {
    return (
      <div ref={ref} className={cn(SHELL, "min-h-[200px]")}>
        <div className={BACKDROP} aria-hidden />
      </div>
    );
  }

  return (
    <div ref={ref} className={SHELL}>
      <div className={BACKDROP} aria-hidden />
      <div className="relative">
        <PanelHeader state={state} />
        <div className="h-px bg-divider" />
        <PrayerList state={state} />
        <div className="h-px bg-divider" />
        <PanelSummary state={state} />
        <div className="h-px bg-divider" />
        <PanelFooter />
      </div>
    </div>
  );
};
