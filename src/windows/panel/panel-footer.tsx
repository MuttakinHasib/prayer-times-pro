import type { ReactNode } from "react";
import { checkForUpdates, hidePanel, openSettings, quitApp } from "../../lib/ipc";
import { GearIcon, PowerIcon, RefreshIcon } from "../../components/icons";

/** Settings / Check for Updates / Quit actions. */
export const PanelFooter = () => (
  <div className="flex flex-col p-1.5">
    <FooterButton icon={<GearIcon size={15} />} onClick={() => openSettings().then(hidePanel)}>
      Settings…
    </FooterButton>
    <FooterButton icon={<RefreshIcon size={15} />} onClick={() => checkForUpdates()}>
      Check for Updates…
    </FooterButton>
    <FooterButton icon={<PowerIcon size={15} />} onClick={() => quitApp()}>
      Quit
    </FooterButton>
  </div>
);

const FooterButton = ({
  icon,
  children,
  onClick,
}: {
  icon: ReactNode;
  children: ReactNode;
  onClick: () => void;
}) => (
  <button
    type="button"
    onClick={onClick}
    className="flex w-full items-center gap-2.5 rounded-md px-2.5 py-1.5 text-left text-[12.5px] text-content hover:bg-surface-hover [&>svg]:text-content-muted"
  >
    {icon}
    {children}
  </button>
);
