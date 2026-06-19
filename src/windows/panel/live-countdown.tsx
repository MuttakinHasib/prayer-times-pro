import { memo } from "react";
import { useCountdown } from "../../hooks/use-countdown";
import { countdownLong, shortCountdown } from "../../lib/format";

interface Props {
  /** Target instant in epoch milliseconds. */
  targetMs: number;
  /** `long` → "0:22:45" (hero); `short` → "22m" (row chip). */
  variant: "long" | "short";
  className?: string;
}

/**
 * A self-ticking countdown. It owns its own 1 Hz interval so only this node
 * re-renders each second — the surrounding panel stays static.
 */
export const LiveCountdown = memo(({ targetMs, variant, className }: Props) => {
  const seconds = useCountdown(targetMs);
  return (
    <span className={className}>
      {variant === "long" ? countdownLong(seconds) : shortCountdown(seconds)}
    </span>
  );
});
LiveCountdown.displayName = "LiveCountdown";
