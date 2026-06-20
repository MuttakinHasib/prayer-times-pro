import { memo, type ReactNode } from "react";
import { checkForUpdates, hidePanel, openMain, openSettings, quitApp } from "../../lib/ipc";

/** Open Prayer Times / Settings / Check for Updates / Quit, with mono shortcut hints. */
export const PanelFooter = memo(() => (
  <div className="flex flex-col px-2.5 pb-2.5 pt-1">
    <FooterButton shortcut="⌘O" onClick={() => openMain().then(hidePanel)}>
      Open Prayer Times
    </FooterButton>
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
    className="flex w-full items-center rounded-lg px-2.5 py-2 text-left text-[12.5px] text-content hover:bg-surface-hover"
  >
    {children}
    {shortcut && (
      <span className="ml-auto font-mono text-[11px] font-medium text-content-subtle">
        {shortcut}
      </span>
    )}
  </button>
);
