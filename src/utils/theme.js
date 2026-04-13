import { setCssVar } from "quasar"
import { DEFAULT_PREFERENCES, getStoredThemeModePreference } from "./preferences"

const LIGHT_THEME = {
  mode: "light",
  quasarBrand: {
    primary: "#1A73E8",
    secondary: "#334155",
    accent: "#1A73E8",
    dark: "#0F172A",
    positive: "#10B981",
    negative: "#EF4444",
    info: "#1A73E8",
    warning: "#F59E0B",
  },
  cssVars: {
    "dt-brand-primary": "#1A73E8",
    "dt-brand-primary-hover": "#3B82F6",
    "dt-brand-secondary": "#334155",
    "dt-brand-secondary-hover": "#475569",
    "dt-brand-accent": "#1A73E8",
    "dt-brand-primary-soft": "rgba(26, 115, 232, 0.14)",
    "dt-brand-secondary-soft": "rgba(51, 65, 85, 0.16)",
    "dt-brand-accent-soft": "rgba(26, 115, 232, 0.14)",
    "dt-status-success-soft": "rgba(16, 185, 129, 0.14)",
    "dt-status-warning-soft": "rgba(245, 158, 11, 0.16)",
    "dt-status-danger-soft": "rgba(239, 68, 68, 0.14)",
    "dt-accent": "#1A73E8",
    "dt-accent-hover": "#3B82F6",
    "dt-accent-contrast": "#FFFFFF",
    "dt-success": "#10B981",
    "dt-warning": "#F59E0B",
    "dt-danger": "#EF4444",
    "dt-info": "#1A73E8",
    "dt-bg-app": "#e8edf4",
    "dt-bg-shell": "#edf2f7",
    "dt-bg-sidebar": "#ffffff",
    "dt-bg-panel": "#ffffff",
    "dt-bg-panel-soft": "#f5f7fb",
    "dt-bg-panel-strong": "#ffffff",
    "dt-bg-input": "#ffffff",
    "dt-bg-chip": "rgba(255, 255, 255, 0.98)",
    "dt-bg-readout": "linear-gradient(180deg, #fbfcfe, #f2f6fb)",
    "dt-header-surface": "#ffffff",
    "dt-header-surface-strong": "#ffffff",
    "dt-header-border": "rgba(176, 190, 212, 0.52)",
    "dt-header-muted": "#61738c",
    "dt-header-segment-bg": "#eef2f7",
    "dt-header-segment-active": "linear-gradient(135deg, rgba(26, 115, 232, 0.16), rgba(59, 130, 246, 0.12))",
    "dt-header-field-bg": "#ffffff",
    "dt-header-action-primary": "linear-gradient(135deg, #1A73E8, #3B82F6)",
    "dt-header-action-shadow": "0 10px 24px rgba(26, 115, 232, 0.18)",
    "dt-gloss-surface": "linear-gradient(180deg, rgba(255, 255, 255, 0.99), rgba(247, 250, 254, 0.95))",
    "dt-gloss-surface-soft": "linear-gradient(180deg, rgba(248, 251, 254, 0.98), rgba(239, 244, 250, 0.92))",
    "dt-gloss-surface-ghost": "linear-gradient(180deg, rgba(255, 255, 255, 0.76), rgba(244, 247, 252, 0.58))",
    "dt-gloss-active": "linear-gradient(180deg, #87ccff 0%, #3b95ff 46%, #0c63e8 100%)",
    "dt-gloss-border": "rgba(181, 194, 214, 0.56)",
    "dt-gloss-border-strong": "rgba(132, 156, 196, 0.4)",
    "dt-gloss-shadow": "0 10px 24px rgba(67, 88, 122, 0.08)",
    "dt-gloss-shadow-soft": "0 6px 14px rgba(67, 88, 122, 0.06)",
    "dt-gloss-shadow-strong": "0 12px 24px rgba(29, 106, 220, 0.22)",
    "dt-gloss-inset": "inset 0 1px 0 rgba(255, 255, 255, 0.94), inset 0 -1px 0 rgba(216, 225, 239, 0.54)",
    "dt-gloss-blue-inset": "inset 0 1px 0 rgba(255, 255, 255, 0.42), inset 0 -2px 0 rgba(7, 73, 180, 0.36)",
    "dt-text-primary": "#1e293b",
    "dt-text-secondary": "#64748b",
    "dt-text-muted": "#94a3b8",
    "dt-text-contrast": "#ffffff",
    "dt-border": "rgba(176, 190, 212, 0.44)",
    "dt-border-strong": "rgba(132, 156, 196, 0.38)",
    "dt-shadow-panel": "0 10px 24px rgba(67, 88, 122, 0.08)",
    "dt-shadow-float": "0 16px 32px rgba(67, 88, 122, 0.1)",
    "dt-shadow-window": "0 18px 40px rgba(43, 69, 104, 0.12)",
    "dt-window-border": "rgba(148, 163, 184, 0.24)",
  },
}

const DARK_THEME = {
  mode: "dark",
  quasarBrand: {
    primary: "#4F8CFF",
    secondary: "#B6B2C8",
    accent: "#4F8CFF",
    dark: "#08060F",
    positive: "#50D78A",
    negative: "#FF7A7A",
    info: "#8AB4FF",
    warning: "#F6C760",
  },
  cssVars: {
    "dt-brand-primary": "#4F8CFF",
    "dt-brand-primary-hover": "#77A8FF",
    "dt-brand-secondary": "#B6B2C8",
    "dt-brand-secondary-hover": "#D2CEE3",
    "dt-brand-accent": "#4F8CFF",
    "dt-brand-primary-soft": "rgba(79, 140, 255, 0.18)",
    "dt-brand-secondary-soft": "rgba(182, 178, 200, 0.14)",
    "dt-brand-accent-soft": "rgba(79, 140, 255, 0.16)",
    "dt-status-success-soft": "rgba(80, 215, 138, 0.16)",
    "dt-status-warning-soft": "rgba(246, 199, 96, 0.18)",
    "dt-status-danger-soft": "rgba(255, 122, 122, 0.18)",
    "dt-accent": "#4FA0FF",
    "dt-accent-hover": "#3A8AE4",
    "dt-accent-contrast": "#ffffff",
    "dt-success": "#50D78A",
    "dt-warning": "#F6C760",
    "dt-danger": "#FF7A7A",
    "dt-info": "#8AB4FF",
    "dt-bg-app": "#08060F",
    "dt-bg-shell": "#0D0A16",
    "dt-bg-sidebar": "rgba(19, 16, 28, 0.98)",
    "dt-bg-panel": "rgba(24, 20, 34, 0.98)",
    "dt-bg-panel-soft": "#211C2D",
    "dt-bg-panel-strong": "#14111D",
    "dt-bg-input": "#1B1727",
    "dt-bg-chip": "rgba(29, 24, 41, 0.94)",
    "dt-bg-readout": "linear-gradient(180deg, #171321, #100D18)",
    "dt-header-surface": "#191522",
    "dt-header-surface-strong": "#120F1A",
    "dt-header-border": "rgba(124, 116, 154, 0.24)",
    "dt-header-muted": "#9A93B2",
    "dt-header-segment-bg": "#231D31",
    "dt-header-segment-active": "linear-gradient(135deg, #322A45, #241D35)",
    "dt-header-field-bg": "rgba(21, 17, 30, 0.9)",
    "dt-header-action-primary": "linear-gradient(135deg, #4F8CFF, #2F6FE3)",
    "dt-header-action-shadow": "0 12px 24px rgba(39, 94, 189, 0.24)",
    "dt-gloss-surface": "linear-gradient(180deg, rgba(24, 20, 34, 0.98), rgba(18, 15, 27, 0.96))",
    "dt-gloss-surface-soft": "linear-gradient(180deg, rgba(28, 23, 40, 0.96), rgba(20, 17, 30, 0.94))",
    "dt-gloss-surface-ghost": "linear-gradient(180deg, rgba(31, 25, 44, 0.72), rgba(15, 12, 22, 0.62))",
    "dt-gloss-active": "linear-gradient(180deg, #79AEFF 0%, #4F8CFF 46%, #2F6FE3 100%)",
    "dt-gloss-border": "rgba(103, 96, 128, 0.42)",
    "dt-gloss-border-strong": "rgba(139, 131, 170, 0.34)",
    "dt-gloss-shadow": "0 18px 36px rgba(0, 0, 0, 0.34)",
    "dt-gloss-shadow-soft": "0 12px 24px rgba(0, 0, 0, 0.24)",
    "dt-gloss-shadow-strong": "0 24px 46px rgba(0, 0, 0, 0.42)",
    "dt-gloss-inset": "inset 0 1px 0 rgba(255, 255, 255, 0.06), inset 0 -1px 0 rgba(87, 76, 119, 0.22)",
    "dt-gloss-blue-inset": "inset 0 1px 0 rgba(255, 255, 255, 0.14), inset 0 -2px 0 rgba(55, 63, 150, 0.4)",
    "dt-text-primary": "#F3F1FA",
    "dt-text-secondary": "#C7C2D9",
    "dt-text-muted": "#8E86A6",
    "dt-text-contrast": "#F7FBFF",
    "dt-border": "rgba(110, 101, 136, 0.34)",
    "dt-border-strong": "rgba(146, 136, 179, 0.4)",
    "dt-shadow-panel": "0 22px 56px rgba(0, 0, 0, 0.3)",
    "dt-shadow-float": "0 34px 74px rgba(0, 0, 0, 0.46)",
    "dt-shadow-window": "0 26px 72px rgba(0, 0, 0, 0.42)",
    "dt-window-border": "rgba(112, 104, 138, 0.26)",
  },
}

const THEMES = {
  light: LIGHT_THEME,
  dark: DARK_THEME,
}

function applyThemeTokens(theme) {
  if (typeof document === "undefined") {
    return
  }

  Object.entries(theme.cssVars).forEach(([name, value]) => {
    document.documentElement.style.setProperty(`--${name}`, value)
    if (document.body) {
      document.body.style.setProperty(`--${name}`, value)
    }
  })

  Object.entries(theme.quasarBrand).forEach(([name, value]) => {
    setCssVar(name, value)
  })
}

export function normalizeThemeMode(mode) {
  return mode === "dark" ? "dark" : DEFAULT_PREFERENCES.themeMode
}

export function getStoredThemeMode() {
  return normalizeThemeMode(getStoredThemeModePreference())
}

export function applyThemeMode(mode, darkPlugin) {
  const resolved = normalizeThemeMode(mode)
  const theme = THEMES[resolved]

  if (typeof document !== "undefined") {
    document.documentElement.dataset.theme = resolved
    document.documentElement.classList.toggle("theme-dark", resolved === "dark")
    document.documentElement.classList.toggle("theme-light", resolved === "light")

    if (document.body) {
      document.body.dataset.theme = resolved
      document.body.classList.toggle("theme-dark", resolved === "dark")
      document.body.classList.toggle("theme-light", resolved === "light")
      document.body.classList.toggle("body--dark", resolved === "dark")
      document.body.classList.toggle("body--light", resolved === "light")
      document.body.style.colorScheme = resolved
    }

    document.documentElement.style.colorScheme = resolved
  }

  applyThemeTokens(theme)

  if (darkPlugin?.set) {
    darkPlugin.set(resolved === "dark")
  }

  return resolved
}
