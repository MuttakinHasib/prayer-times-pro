import { useEffect, useState } from "react";
import { Bell, EyeOff, MoonStar, Navigation, Settings as Gear, type LucideIcon } from "lucide-react";
import { cn } from "../../lib/cn";
import type { AppSettings } from "../../lib/settings";
import { useSettingsStore } from "../../stores/settings.store";
import { GeneralTab } from "./tabs/general-tab";
import { LocationTab } from "./tabs/location-tab";
import { CalculationTab } from "./tabs/calculation-tab";
import { NotificationsTab } from "./tabs/notifications-tab";
import { FocusTab } from "./tabs/focus-tab";

type TabProps = { settings: AppSettings; update: (patch: Partial<AppSettings>) => void };

const TABS: {
  id: string;
  label: string;
  Icon: LucideIcon;
  Body: (p: TabProps) => React.ReactNode;
}[] = [
  { id: "general", label: "General", Icon: Gear, Body: GeneralTab },
  { id: "location", label: "Location & Time", Icon: Navigation, Body: LocationTab },
  { id: "calculation", label: "Calculation", Icon: MoonStar, Body: CalculationTab },
  { id: "notifications", label: "Notifications", Icon: Bell, Body: NotificationsTab },
  { id: "focus", label: "Focus Mode", Icon: EyeOff, Body: FocusTab },
];

export const Settings = () => {
  const settings = useSettingsStore((s) => s.settings);
  const hydrate = useSettingsStore((s) => s.hydrate);
  const update = useSettingsStore((s) => s.update);
  const [tabId, setTabId] = useState(TABS[0].id);

  useEffect(() => {
    void hydrate();
  }, [hydrate]);

  const active = TABS.find((t) => t.id === tabId) ?? TABS[0];

  return (
    <div className="flex h-screen w-full flex-col overflow-hidden bg-bg text-content">
      <nav className="flex gap-1 border-b border-border px-3 py-2">
        {TABS.map(({ id, label, Icon }) => (
          <button
            key={id}
            type="button"
            onClick={() => setTabId(id)}
            className={cn(
              "flex flex-1 flex-col items-center gap-1 rounded-lg px-2 py-1.5 text-[11px] transition-colors",
              id === tabId
                ? "bg-accent-soft text-accent"
                : "text-content-muted hover:bg-surface-hover hover:text-content",
            )}
          >
            <Icon size={18} />
            {label}
          </button>
        ))}
      </nav>

      <div className="flex-1 overflow-y-auto px-[30px] py-6">
        {settings && <active.Body settings={settings} update={update} />}
      </div>
    </div>
  );
};
