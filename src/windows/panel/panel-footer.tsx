import { memo, type ReactNode } from "react";
import { checkForUpdates, hidePanel, openSettings, quitApp } from "../../lib/ipc";

/** Settings / Check for Updates / Quit actions with mono shortcut hints. */
export const PanelFooter = memo(() => (
  <div className="flex flex-col px-3 pb-3 pt-1.5">
    <FooterButton shortcut="⌘," onClick={() => openSettings().then(hidePanel)}>
      Settings
    </FooterButton>
    <FooterButton onClick={() => checkForUpdates()}>Check for Updates…</FooterButton>
    <FooterButton shortcut="⌘Q" onClick={() => quitApp()}>
      Quit
    </FooterButton>
  </div>
));
PanelFooter.displayName = "PanelFooter";

interface FooterButtonProps {
  shortcut?: string;
  children: ReactNode;
  onClick: () => void;
}

const FooterButton = ({ shortcut, children, onClick }: FooterButtonProps) => (
  <button
    type="button"
    onClick={onClick}
    className="flex w-full items-center rounded-lg px-2.5 py-[9px] text-left text-[13px] text-content hover:bg-surface-hover"
  >
    {children}
    {shortcut && (
      <span className="ml-auto font-mono text-[11px] font-medium text-content-subtle">
        {shortcut}
      </span>
    )}
  </button>
);
