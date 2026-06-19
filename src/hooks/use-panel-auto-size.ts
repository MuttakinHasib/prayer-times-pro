import { useLayoutEffect, useRef } from "react";
import { reportContentSize } from "../lib/ipc";

/**
 * Report the panel's measured content size to Rust whenever it changes, so the
 * native window can be sized to its content (the shadow hugs the card). Returns a
 * ref to attach to the panel's root element.
 */
export const usePanelAutoSize = () => {
  const ref = useRef<HTMLDivElement>(null);

  useLayoutEffect(() => {
    const el = ref.current;
    if (!el) return;
    const report = () => void reportContentSize(el.offsetWidth, el.offsetHeight);
    report();
    const observer = new ResizeObserver(report);
    observer.observe(el);
    return () => observer.disconnect();
  }, []);

  return ref;
};
