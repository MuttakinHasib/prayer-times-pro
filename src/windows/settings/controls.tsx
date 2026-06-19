import type { ReactNode } from "react";
import { Minus, Plus } from "lucide-react";
import { cn } from "../../lib/cn";

/** A titled group: a mono uppercase label over a bordered surface card whose
 *  rows are separated by hairline dividers (macOS grouped-list style). */
export const Section = ({ title, children }: { title?: string; children: ReactNode }) => (
  <section className="mb-[22px]">
    {title && (
      <h2 className="mb-2.5 font-mono text-[10px] font-semibold uppercase tracking-[0.12em] text-content-subtle">
        {title}
      </h2>
    )}
    <div className="divide-y divide-divider rounded-xl border border-border bg-surface px-[18px]">
      {children}
    </div>
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
  <div className="flex min-h-[40px] items-center justify-between gap-3 py-[14px]">
    <div className="min-w-0">
      <div className="text-[13.5px] text-content">{label}</div>
      {sublabel && <div className="mt-0.5 text-[11.5px] text-content-subtle">{sublabel}</div>}
    </div>
    {children}
  </div>
);

/** A small footnote under a section. */
export const Note = ({ children }: { children: ReactNode }) => (
  <p className="mb-[22px] mt-[-12px] text-[11.5px] leading-snug text-content-subtle">{children}</p>
);

export const Toggle = ({
  checked,
  disabled = false,
  onChange,
}: {
  checked: boolean;
  disabled?: boolean;
  onChange: (value: boolean) => void;
}) => (
  <button
    type="button"
    role="switch"
    aria-checked={checked}
    disabled={disabled}
    onClick={() => onChange(!checked)}
    className={cn("relative h-[24px] w-[40px] shrink-0 rounded-full transition-colors", {
      "bg-accent": checked,
      "bg-white/20": !checked,
      "cursor-default opacity-40": disabled,
    })}
  >
    <span
      className={cn(
        "absolute top-[2px] left-[2px] h-[20px] w-[20px] rounded-full bg-white shadow-[0_1px_2px_rgba(0,0,0,0.35)] transition-transform",
        { "translate-x-[16px]": checked },
      )}
    />
  </button>
);

/** A borderless dropdown: the value sits right-aligned in muted text with the
 *  platform's native popup chevron — matches the grouped-list rows. */
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
    className="max-w-[62%] cursor-pointer truncate bg-transparent text-right text-[12.5px] text-content-muted transition-colors hover:text-content"
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
  <div className="flex shrink-0 rounded-[7px] bg-elevated p-[3px]">
    {options.map((o) => (
      <button
        key={o.value}
        type="button"
        onClick={() => onChange(o.value)}
        className={cn("rounded-[5px] px-3 py-1 text-[12px] transition-colors", {
          "bg-accent font-semibold text-accent-on": value === o.value,
          "text-content-muted hover:text-content": value !== o.value,
        })}
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
      <div className="flex overflow-hidden rounded-md border border-border">
        <button
          type="button"
          onClick={() => onChange(clamp(value - step))}
          className="px-1.5 py-1 text-content-muted hover:bg-surface-hover hover:text-content"
        >
          <Minus size={13} />
        </button>
        <button
          type="button"
          onClick={() => onChange(clamp(value + step))}
          className="border-l border-border px-1.5 py-1 text-content-muted hover:bg-surface-hover hover:text-content"
        >
          <Plus size={13} />
        </button>
      </div>
    </div>
  );
};
