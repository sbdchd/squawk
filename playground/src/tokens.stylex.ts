import * as stylex from "@stylexjs/stylex"

export const colors = stylex.defineVars({
  background: "rgb(30, 30, 30)",
  border: "#282828",
  text: "#fff",
  subtleText: "oklch(87.2% 0.01 258.338)",
  hover: "oklch(37.3% 0.034 259.733)",
  accent: "oklch(54.6% 0.245 262.881)",
  info: "oklch(42.4% 0.199 265.638)",
  infoText: "oklch(70.7% 0.165 254.624)",
  bannerText: "oklch(96.8% 0.007 247.896)",
  bannerBorder: "oklch(86.9% 0.022 252.894)",
  warningBackground: "oklch(47.3% 0.137 46.201)",
  warning: "oklch(87.9% 0.169 91.605)",
  error: "oklch(70.4% 0.191 22.216)",
  errorText: "oklch(55.3% 0.195 38.402)",
})

export const breakpoints = stylex.defineConsts({
  md: "@media (min-width: 48rem)",
})

export const transitions = stylex.defineConsts({
  colors: "color, background-color, border-color",
  duration: "150ms",
  easing: "cubic-bezier(0.4, 0, 0.2, 1)",
})
