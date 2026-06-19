import { create } from "zustand";
import { getPrayerState, onStateChanged, type PrayerState } from "../lib/ipc";

interface PrayerStore {
  state: PrayerState | null;
}

/**
 * Holds the prayer state owned by the Rust clock. It's push-based (Tauri events),
 * so a Zustand store fits better than a request/refetch cache: components select
 * just the slice they render and only re-render when that slice changes.
 */
export const usePrayerStore = create<PrayerStore>(() => ({ state: null }));

/**
 * Hydrate once and subscribe to `prayer://state-changed`. Call from the panel
 * window's root effect; returns a cleanup that unsubscribes.
 */
export const initPrayerStore = (): (() => void) => {
  let active = true;
  getPrayerState()
    .then((state) => {
      if (active) usePrayerStore.setState({ state });
    })
    .catch((err) => console.error("get_prayer_state failed", err));

  const unlisten = onStateChanged((state) =>
    usePrayerStore.setState({ state }),
  );
  return () => {
    active = false;
    unlisten.then((fn) => fn()).catch(() => {});
  };
};
