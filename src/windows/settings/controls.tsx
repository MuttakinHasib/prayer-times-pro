import type { ReactNode } from "react";
import { Minus, Plus } from "lucide-react";
import { cn } from "../../lib/cn";

/** A titled group of rows rendered as an inset card. */
export const Section = ({ title, children }: { title?: string; children: ReactNode }) => (
  <section className="mb-5">
    {title && (
      <h2 className="mb-1.5 px-1 text-[13px] font-semibold text-content">{title}</h2>
    )}
    <div className="divide-y divide-border overflow-hidden rounded-lg bg-white/5">{children}</div>
  </section>
);

/** A label + optional sub-label on the left, a control on the right. */
export const Row = ({
  label,
  sublabel,
  children,
}: {
  label: ReactNode;
  sublabel?: ReactNode;
  children?: ReactNode;
}) => (
  <div className="flex min-h-[40px] items-center justify-between gap-3 px-3 py-2">
    <div className="min-w-0">
      <div className="text-[13px] text-content">{label}</div>
      {sublabel && <div className="mt-0.5 text-[11px] text-content-subtle">{sublabel}</div>}
    </div>
    {children}
  </div>
);

/** A small footnote under a section. */
export const Note = ({ children }: { children: ReactNode }) => (
  <p className="mb-5 mt-[-12px] px-1 text-[11px] leading-snug text-content-subtle">{children}</p>
);

export const Toggle = ({
  checked,
  onChange,
}: {
  checked: boolean;
  onChange: (value: boolean) => void;
}) => (
  <button
    type="button"
    role="switch"
    aria-checked={checked}
    onClick={() => onChange(!checked)}
    className={cn(
      "relative h-[22px] w-[38px] shrink-0 rounded-full transition-colors",
      checked ? "bg-accent" : "bg-white/15",
    )}
  >
    <span
      className={cn(
        "absolute top-[2px] h-[18px] w-[18px] rounded-full bg-white shadow transition-transform",
        checked ? "translate-x-[18px]" : "translate-x-[2px]",
      )}
    />
  </button>
);

export const SelectField = <T extends string>({
  value,
  options,
  onChange,
}: {
  value: T;
  options: { value: T; label: string }[];
  onChange: (value: T) => void;
}) => (
  <select
    value={value}
    onChange={(e) => onChange(e.target.value as T)}
    className="max-w-[60%] rounded-md border border-white/10 bg-white/10 px-2 py-1 text-[12.5px] text-content"
  >
    {options.map((o) => (
      <option key={o.value} value={o.value}>
        {o.label}
      </option>
    ))}
  </select>
);

export const Segmented = <T extends string>({
  value,
  options,
  onChange,
}: {
  value: T;
  options: { value: T; label: string }[];
  onChange: (value: T) => void;
}) => (
  <div className="flex shrink-0 rounded-md bg-white/10 p-0.5">
    {options.map((o) => (
      <button
        key={o.value}
        type="button"
        onClick={() => onChange(o.value)}
        className={cn(
          "rounded px-2.5 py-1 text-[12px] transition-colors",
          value === o.value ? "bg-accent text-white" : "text-content-muted hover:text-content",
        )}
      >
        {o.label}
      </button>
    ))}
  </div>
);

export const Stepper = ({
  value,
  min = 0,
  max = 999,
  step = 1,
  format,
  onChange,
}: {
  value: number;
  min?: number;
  max?: number;
  step?: number;
  format?: (v: number) => string;
  onChange: (value: number) => void;
}) => {
  const clamp = (v: number) => Math.min(max, Math.max(min, v));
  return (
    <div className="flex shrink-0 items-center gap-2">
      <span className="min-w-[64px] text-right text-[12.5px] tabular-nums text-content">
        {format ? format(value) : value}
      </span>
      <div className="flex overflow-hidden rounded-md border border-white/10">
        <button
          type="button"
          onClick={() => onChange(clamp(value - step))}
          className="px-1.5 py-1 text-content-muted hover:bg-white/10 hover:text-content"
        >
          <Minus size={13} />
        </button>
        <button
          type="button"
          onClick={() => onChange(clamp(value + step))}
          className="border-l border-white/10 px-1.5 py-1 text-content-muted hover:bg-white/10 hover:text-content"
        >
          <Plus size={13} />
        </button>
      </div>
    </div>
  );
};
