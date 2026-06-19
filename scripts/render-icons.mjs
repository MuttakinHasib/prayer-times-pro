// Renders the Mihrab brand mark into the app-icon master + menu-bar tray glyphs.
// Run: bun scripts/render-icons.mjs   (then: bunx tauri icon icons/_master.png)
import { Resvg } from "@resvg/resvg-js";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import {
  MIHRAB_ARCH_SOLID,
  MIHRAB_CRESCENT,
  MIHRAB_FRAME_INNER,
  MIHRAB_FRAME_OUTER,
  MIHRAB_GOLD_STOPS,
} from "../src/lib/mihrab.ts";

const ICONS = join(dirname(fileURLToPath(import.meta.url)), "..", "src-tauri", "icons");

const goldStops = MIHRAB_GOLD_STOPS.map(
  (s) => `<stop offset="${s.offset}" stop-color="${s.color}"/>`,
).join("");
const GOLD = `<radialGradient id="gold" cx="42%" cy="16%" r="92%">${goldStops}</radialGradient>`;

const { keep, cut } = MIHRAB_CRESCENT;

// Mihrab mark (viewBox 0 0 100 100): niche frame + crescent carved from gold.
const markFull = (fill) => `
  <mask id="fr"><rect width="100" height="100" fill="#000"/>
    <path d="${MIHRAB_FRAME_OUTER}" fill="#fff"/>
    <path d="${MIHRAB_FRAME_INNER}" fill="#000"/></mask>
  <mask id="cr"><rect width="100" height="100" fill="#000"/>
    <circle cx="${keep.cx}" cy="${keep.cy}" r="${keep.r}" fill="#fff"/><circle cx="${cut.cx}" cy="${cut.cy}" r="${cut.r}" fill="#000"/></mask>
  <rect width="100" height="100" fill="${fill}" mask="url(#fr)"/>
  <rect width="100" height="100" fill="${fill}" mask="url(#cr)"/>`;

// Solid archway silhouette for small / monochrome use.
const markSolid = (fill) => `<path d="${MIHRAB_ARCH_SOLID}" fill="${fill}"/>`;

// Full app icon: obsidian rounded-square tile + centered gold mihrab.
const appIcon = (px) => {
  const r = Math.round(px * 0.225);
  const scale = (px * 0.64) / 100;
  const off = (px - px * 0.64) / 2;
  return `<svg width="${px}" height="${px}" viewBox="0 0 ${px} ${px}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <radialGradient id="bg" cx="68%" cy="18%" r="92%">
      <stop offset="0" stop-color="#1f2127"/><stop offset="0.72" stop-color="#0b0c0e"/><stop offset="1" stop-color="#0b0c0e"/>
    </radialGradient>
    ${GOLD}
  </defs>
  <rect width="${px}" height="${px}" rx="${r}" ry="${r}" fill="url(#bg)"/>
  <g transform="translate(${off} ${off}) scale(${scale})">${markFull("url(#gold)")}</g>
</svg>`;
};

// Tray template: solid archway in opaque black; macOS tints via the alpha channel.
const trayGlyph = () =>
  `<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">${markSolid("#000000")}</svg>`;

const renderPng = (svg, width) =>
  new Resvg(svg, { fitTo: { mode: "width", value: width } }).render().asPng();

writeFileSync(join(ICONS, "_master.png"), renderPng(appIcon(1024), 1024));
writeFileSync(join(ICONS, "tray-mihrab.png"), renderPng(trayGlyph(), 22));
writeFileSync(join(ICONS, "tray-mihrab@2x.png"), renderPng(trayGlyph(), 44));
console.log("rendered: _master.png (1024), tray-mihrab.png (22), tray-mihrab@2x.png (44)");
