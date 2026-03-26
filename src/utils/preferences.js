const STORAGE_KEY = "dt-tools.preferences"

export const DEFAULT_PREFERENCES = {
  logDirectory: "",
  autostart: false,
  checkUpdatesOnLaunch: true,
  updateChannel: "Stable",
  themeColor: "#1677FF",
  density: "Desktop Default",
}

export function loadPreferences() {
  if (typeof window === "undefined") {
    return { ...DEFAULT_PREFERENCES }
  }

  try {
    const raw = window.localStorage.getItem(STORAGE_KEY)
    if (!raw) {
      return { ...DEFAULT_PREFERENCES }
    }

    return {
      ...DEFAULT_PREFERENCES,
      ...JSON.parse(raw),
    }
  } catch {
    return { ...DEFAULT_PREFERENCES }
  }
}

export function savePreferences(preferences) {
  if (typeof window === "undefined") return
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(preferences))
}

export function shouldCheckUpdatesOnLaunch() {
  return loadPreferences().checkUpdatesOnLaunch
}
