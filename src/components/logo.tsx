import { useId } from "react";
import { cn } from "../lib/cn";

/**
 * The Mihrab brand mark — a pointed prayer-niche archway framing a crescent.
 * `full` carves the niche frame + crescent out of the gold; `solid` is the
 * filled archway silhouette that stays crisp at small sizes and in the menu bar.
 */
type MarkDetail = "full" | "solid";
type MarkTone = "gold" | "mono" | "light";

// Geometry from the brand spec (viewBox 0 0 100 100).
const FRAME_OUTER = "M27,85 L27,52 C27,31 38,17 50,12 C62,17 73,31 73,52 L73,85 Z";
const FRAME_INNER = "M36,85 L36,54 C36,38 43,27 50,23 C57,27 64,38 64,54 L64,85 Z";
const ARCH_SOLID = "M30,86 L30,52 C30,31 40,17 50,12 C60,17 70,31 70,52 L70,86 Z";

export const MihrabMark = ({
  size = 100,
  detail = "full",
  tone = "gold",
  className,
}: {
  size?: number;
  detail?: MarkDetail;
  tone?: MarkTone;
  className?: string;
}) => {
  const uid = useId().replace(/:/g, "");
  const gradId = `mihrab-g-${uid}`;
  const frameMask = `mihrab-fr-${uid}`;
  const crescentMask = `mihrab-cr-${uid}`;
  const fill = tone === "gold" ? `url(#${gradId})` : tone === "light" ? "#a07f3c" : "#f2f0ec";

  return (
    <svg
      width={size}
      height={size}
      viewBox="0 0 100 100"
      fill="none"
      aria-hidden="true"
      className={className}
    >
      {tone === "gold" && (
        <defs>
          <radialGradient id={gradId} cx="42%" cy="16%" r="92%">
            <stop offset="0" stopColor="#ecd49a" />
            <stop offset="0.6" stopColor="#c8a968" />
            <stop offset="1" stopColor="#a8893f" />
          </radialGradient>
        </defs>
      )}

      {detail === "solid" ? (
        <path d={ARCH_SOLID} fill={fill} />
      ) : (
        <>
          <defs>
            <mask id={frameMask}>
              <rect width="100" height="100" fill="#000" />
              <path d={FRAME_OUTER} fill="#fff" />
              <path d={FRAME_INNER} fill="#000" />
            </mask>
            <mask id={crescentMask}>
              <rect width="100" height="100" fill="#000" />
              <circle cx="51.5" cy="49" r="8.7" fill="#fff" />
              <circle cx="57.5" cy="44.5" r="7.1" fill="#000" />
            </mask>
          </defs>
          <rect width="100" height="100" fill={fill} mask={`url(#${frameMask})`} />
          <rect width="100" height="100" fill={fill} mask={`url(#${crescentMask})`} />
        </>
      )}
    </svg>
  );
};

/** The mark inside the obsidian rounded-square app-icon tile. */
export const AppIconTile = ({
  size = 64,
  className,
}: {
  size?: number;
  className?: string;
}) => (
  <div
    className={cn(
      "flex items-center justify-center rounded-[22.5%] shadow-[inset_0_1px_0_rgba(255,255,255,.06)]",
      className,
    )}
    style={{
      width: size,
      height: size,
      background: "radial-gradient(125% 125% at 68% 18%, #1f2127 0%, #0b0c0e 72%)",
    }}
  >
    <MihrabMark size={size * 0.64} detail={size < 40 ? "solid" : "full"} />
  </div>
);

/** Horizontal wordmark: Newsreader name + outlined mono PRO badge. */
export const Wordmark = ({ markSize = 28 }: { markSize?: number }) => (
  <div className="flex items-center gap-3">
    <AppIconTile size={markSize} className="rounded-[24%]" />
    <div className="flex items-baseline gap-2">
      <span className="font-display text-[15px] font-medium tracking-[-0.01em] text-content">
        Prayer Times
      </span>
      <span className="-translate-y-px rounded-[5px] border border-accent/40 px-1.5 py-0.5 font-mono text-[9px] font-semibold tracking-[0.18em] text-accent">
        PRO
      </span>
    </div>
  </div>
);
