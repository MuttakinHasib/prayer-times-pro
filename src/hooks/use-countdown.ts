import { useEffect, useState } from "react";

/**
 * Seconds remaining until `targetMs`, re-evaluated once per second. Scoped to the
 * smallest component that shows a countdown so a tick re-renders only that node,
 * not the whole panel.
 */
export const useCountdown = (targetMs: number | undefined): number => {
  const remaining = () => (targetMs == null ? 0 : Math.max(0, (targetMs - Date.now()) / 1000));
  const [seconds, setSeconds] = useState(remaining);

  useEffect(() => {
    setSeconds(remaining());
    const id = setInterval(() => setSeconds(remaining()), 1000);
    return () => clearInterval(id);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [targetMs]);

  return seconds;
};
