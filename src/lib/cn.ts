import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

/** Compose conditional Tailwind classes, de-duplicating conflicts. */
export const cn = (...inputs: ClassValue[]): string => twMerge(clsx(inputs));
