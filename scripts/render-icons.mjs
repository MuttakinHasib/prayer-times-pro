// Renders the Mihrab brand mark into the app-icon master + menu-bar tray glyphs.
// Run: bun scripts/render-icons.mjs   (then: bunx tauri icon icons/_master.png)
import { Resvg } from "@resvg/resvg-js";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const ICONS = join(dirname(fileURLToPath(import.meta.url)), "..", "src-tauri", "icons");

const GOLD = `<radialGradient id="gold" cx="42%" cy="16%" r="92%">
    <stop offset="0" stop-color="#ecd49a"/><stop offset="0.6" stop-color="#c8a968"/><stop offset="1" stop-color="#a8893f"/>
  </radialGradient>`;

// Mihrab mark (viewBox 0 0 100 100): niche frame + crescent carved from gold.
const markFull = (fill) => `
  <mask id="fr"><rect width="100" height="100" fill="#000"/>
    <path d="M27,85 L27,52 C27,31 38,17 50,12 C62,17 73,31 73,52 L73,85 Z" fill="#fff"/>
    <path d="M36,85 L36,54 C36,38 43,27 50,23 C57,27 64,38 64,54 L64,85 Z" fill="#000"/></mask>
  <mask id="cr"><rect width="100" height="100" fill="#000"/>
    <circle cx="51.5" cy="49" r="8.7" fill="#fff"/><circle cx="57.5" cy="44.5" r="7.1" fill="#000"/></mask>
  <rect width="100" height="100" fill="${fill}" mask="url(#fr)"/>
  <rect width="100" height="100" fill="${fill}" mask="url(#cr)"/>`;

// Solid archway silhouette for small / monochrome use.
const markSolid = (fill) =>
  `<path d="M30,86 L30,52 C30,31 40,17 50,12 C60,17 70,31 70,52 L70,86 Z" fill="${fill}"/>`;

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
