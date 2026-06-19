import { useEffect, useRef, useState } from "react";
import {
  dismissFocus,
  engageFocus,
  onFocusDismiss,
  onFocusEngage,
  type FocusCue,
} from "../../lib/ipc";

const SNOOZE_MS = 10 * 60 * 1000;

// Backdrop blur radius + dark-overlay alpha per intensity. The overlay sits over
// the live desktop, so the blur softens what's behind; "opaque" fully covers it.
const BLUR: Record<FocusCue["blur"], { px: number; alpha: number }> = {
  low: { px: 8, alpha: 0.72 },
  medium: { px: 18, alpha: 0.84 },
  high: { px: 32, alpha: 0.92 },
  opaque: { px: 48, alpha: 1 },
};

const mmss = (total: number) => {
  const t = Math.max(0, total);
  return `${Math.floor(t / 60)}:${String(t % 60).padStart(2, "0")}`;
};

/** Full-screen Focus Mode overlay. A discipline aid: "I've prayed" always exits;
 *  Esc exits when emergency exit is enabled; otherwise it clears after the timer. */
export const Focus = () => {
  const [cue, setCue] = useState<FocusCue | null>(null);
  const [secondsLeft, setSecondsLeft] = useState(0);
  const tick = useRef<ReturnType<typeof setInterval>>(undefined);
  const snooze = useRef<ReturnType<typeof setTimeout>>(undefined);

  const clear = () => {
    clearInterval(tick.current);
    setCue(null);
  };
  const dismiss = () => {
    clear();
    void dismissFocus();
  };

  useEffect(() => {
    const engaged = onFocusEngage((next) => {
      clearTimeout(snooze.current);
      clearInterval(tick.current);
      setCue(next);
      setSecondsLeft(next.durationMinutes * 60);
      tick.current = setInterval(() => {
        setSecondsLeft((s) => {
          if (s <= 1) {
            dismiss();
            return 0;
          }
          return s - 1;
        });
      }, 1000);
    });
    // Another overlay dismissed → clear this one without re-invoking the command.
    const dismissed = onFocusDismiss(clear);
    return () => {
      clearInterval(tick.current);
      clearTimeout(snooze.current);
      void engaged.then((fn) => fn());
      void dismissed.then((fn) => fn());
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    if (!cue?.emergencyExit) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") dismiss();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [cue?.emergencyExit]);

  if (!cue) return null;

  const blur = BLUR[cue.blur];
  const onSnooze = () => {
    const prayer = cue.prayer;
    clear();
    void dismissFocus();
    snooze.current = setTimeout(() => void engageFocus(prayer), SNOOZE_MS);
  };

  return (
    <div
      className="focus-overlay fixed inset-0 flex flex-col items-center justify-center text-content"
      style={{
        backdropFilter: `blur(${blur.px}px)`,
        background: `radial-gradient(120% 120% at 50% 30%, rgba(22,24,28,${blur.alpha}), rgba(11,12,14,${blur.alpha}))`,
      }}
    >
      <style>{`
        @keyframes focus-breathe { from { transform: scale(1); opacity: .85 } to { transform: scale(1.06); opacity: 1 } }
        .focus-breathe { animation: focus-breathe 4s cubic-bezier(.32,.72,0,1) infinite alternate; }
        .focus-overlay { animation: focus-in .6s ease both; }
        @keyframes focus-in { from { opacity: 0 } to { opacity: 1 } }
      `}</style>

      <div className="absolute right-8 top-7 font-mono text-[11px] tracking-[0.16em] text-content-subtle">
        {cue.emergencyExit ? "PRESS ESC ANYTIME TO DISMISS" : `FOCUS · ${cue.prayer.toUpperCase()}`}
      </div>

      <div className="focus-breathe relative mb-12 flex h-[200px] w-[200px] items-center justify-center rounded-full border border-accent-ring">
        <div className="absolute inset-6 rounded-full border border-accent-ring/60" />
        <svg width="40" height="40" viewBox="0 0 100 100" aria-hidden="true" style={{ filter: "drop-shadow(0 0 28px rgba(200,169,104,.55))" }}>
          <defs>
            <mask id="focus-crescent">
              <rect width="100" height="100" fill="#000" />
              <circle cx="48" cy="50" r="42" fill="#fff" />
              <circle cx="63" cy="40" r="35" fill="#000" />
            </mask>
          </defs>
          <rect width="100" height="100" fill="var(--c-accent)" mask="url(#focus-crescent)" />
        </svg>
      </div>

      <div className="font-display text-[18px] italic text-content-muted">It's time for</div>
      <div className="font-display text-[64px] leading-none">{cue.prayer}</div>
      <div className="mt-5 text-[15px] text-accent">
        Your screen will gently clear in {mmss(secondsLeft)}
      </div>

      <div className="mt-12 flex flex-col items-center gap-3">
        <button
          type="button"
          onClick={dismiss}
          className="rounded-[11px] bg-accent px-8 py-3 text-[15px] font-semibold text-accent-on transition-colors hover:bg-accent-emphasis"
        >
          I've prayed
        </button>
        <button
          type="button"
          onClick={onSnooze}
          className="rounded-[11px] border border-border px-6 py-2 text-[13.5px] text-content-muted transition-colors hover:text-content"
        >
          Snooze 10 min
        </button>
      </div>

      <div className="absolute bottom-7 max-w-[520px] px-6 text-center text-[12px] text-content-subtle/70">
        A discipline aid, not a lock — Force Quit always works, and Focus won't trap you.
      </div>
    </div>
  );
};
