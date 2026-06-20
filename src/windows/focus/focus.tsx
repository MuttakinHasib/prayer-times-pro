import { useEffect, useRef, useState } from "react";
import {
  dismissFocus,
  engageFocus,
  onFocusDismiss,
  onFocusEngage,
  type FocusCue,
} from "../../lib/ipc";
import { randomScripture, type Scripture } from "./quotes";

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
  const [quote, setQuote] = useState<Scripture>(randomScripture);
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
      setQuote(randomScripture());
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
        @keyframes focus-breathe { from { transform: scale(1); opacity: .8 } to { transform: scale(1.05); opacity: 1 } }
        @keyframes focus-glow { from { transform: translate(-50%,-50%) scale(.95); opacity: .55 } to { transform: translate(-50%,-50%) scale(1.08); opacity: .9 } }
        @keyframes focus-rise { from { opacity: 0; transform: translateY(8px) } to { opacity: 1; transform: translateY(0) } }
        .focus-breathe { animation: focus-breathe 5s cubic-bezier(.4,0,.6,1) infinite alternate; }
        .focus-glow { animation: focus-glow 5s cubic-bezier(.4,0,.6,1) infinite alternate; }
        .focus-overlay { animation: focus-in .6s ease both; }
        .focus-rise { animation: focus-rise .7s cubic-bezier(.32,.72,0,1) both; }
        @keyframes focus-in { from { opacity: 0 } to { opacity: 1 } }
      `}</style>

      {/* Soft gold radial glow centered behind the ring. */}
      <div
        className="focus-glow pointer-events-none absolute left-1/2 top-[40%] h-[560px] w-[560px] rounded-full"
        style={{ background: "radial-gradient(circle, rgba(200,169,104,.12), transparent 68%)" }}
      />

      <div className="absolute right-8 top-7 font-mono text-[11px] tracking-[0.16em] text-content-subtle">
        {cue.emergencyExit ? "PRESS ESC ANYTIME TO DISMISS" : `FOCUS · ${cue.prayer.toUpperCase()}`}
      </div>

      <div className="focus-breathe relative mb-12 flex h-[208px] w-[208px] items-center justify-center rounded-full border border-accent-ring/70">
        <div className="absolute inset-5 rounded-full border border-accent-ring/40" />
        <div className="absolute inset-10 rounded-full bg-[radial-gradient(circle,rgba(200,169,104,.10),transparent_70%)]" />
        <svg
          width="46"
          height="46"
          viewBox="0 0 100 100"
          aria-hidden="true"
          style={{ filter: "drop-shadow(0 0 34px rgba(200,169,104,.6))" }}
        >
          <defs>
            <radialGradient id="focus-fill" cx="38%" cy="32%" r="75%">
              <stop offset="0" stopColor="#ecd49a" />
              <stop offset="1" stopColor="#c8a968" />
            </radialGradient>
            <mask id="focus-crescent">
              <rect width="100" height="100" fill="#000" />
              <circle cx="48" cy="50" r="42" fill="#fff" />
              <circle cx="63" cy="40" r="35" fill="#000" />
            </mask>
          </defs>
          <rect width="100" height="100" fill="url(#focus-fill)" mask="url(#focus-crescent)" />
        </svg>
      </div>

      <div className="focus-rise flex flex-col items-center" style={{ animationDelay: "120ms" }}>
        <div className="font-display text-[18px] italic text-content-muted">It's time for</div>
        <div className="font-display text-[64px] leading-none tracking-[-0.01em]">{cue.prayer}</div>
        <div className="mt-5 text-[15px] text-accent">
          Your screen will gently clear in <span className="tabular-nums">{mmss(secondsLeft)}</span>
        </div>
      </div>

      <figure
        className="focus-rise mt-10 max-w-[660px] px-8 text-center"
        style={{ animationDelay: "200ms" }}
      >
        <blockquote className="font-display text-[24px] italic leading-snug text-content/90">
          “{quote.text}”
        </blockquote>
        <figcaption className="mt-3.5 text-[14px] text-accent">{quote.source}</figcaption>
      </figure>

      <div
        className="focus-rise mt-11 flex items-center justify-center gap-3"
        style={{ animationDelay: "300ms" }}
      >
        <button
          type="button"
          onClick={dismiss}
          className="rounded-[10px] bg-accent px-7 py-3 text-[14px] font-semibold text-accent-on transition-colors hover:bg-accent-emphasis"
        >
          I've prayed
        </button>
        <button
          type="button"
          onClick={onSnooze}
          className="rounded-[10px] border border-border px-6 py-3 text-[14px] text-content-muted transition-colors hover:border-content-subtle hover:text-content"
        >
          Snooze 10 min
        </button>
      </div>

      <div className="absolute bottom-7 max-w-[520px] px-6 text-center text-[11.5px] text-content-subtle/70">
        A discipline aid, not a lock — Force Quit always works, and Focus won't trap you.
      </div>
    </div>
  );
};
