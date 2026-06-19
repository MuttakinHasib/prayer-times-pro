import { memo, useEffect, useState } from "react";
import { Square } from "lucide-react";
import { onAdhanState, stopAdhan } from "../../lib/ipc";

// The audio thread doesn't report natural end, so clear the bar after the longest
// an Adhan could run as a safety net.
const SAFETY_MS = 6 * 60 * 1000;

/** A slim gold bar offering "Stop Adhan" while an Adhan is playing. */
export const AdhanBar = memo(() => {
  const [playing, setPlaying] = useState(false);

  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | undefined;
    const unlisten = onAdhanState((p) => {
      clearTimeout(timer);
      setPlaying(p);
      if (p) timer = setTimeout(() => setPlaying(false), SAFETY_MS);
    });
    return () => {
      clearTimeout(timer);
      void unlisten.then((fn) => fn());
    };
  }, []);

  if (!playing) return null;

  return (
    <button
      type="button"
      onClick={() => {
        void stopAdhan();
        setPlaying(false);
      }}
      className="flex w-full items-center justify-center gap-2 bg-accent-soft py-2 text-[12.5px] font-medium text-accent transition-colors hover:bg-accent/20"
    >
      <Square size={12} fill="currentColor" />
      Stop Adhan
    </button>
  );
});
AdhanBar.displayName = "AdhanBar";
