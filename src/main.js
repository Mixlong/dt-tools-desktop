import { createApp } from "vue"
import { Dark, Dialog, Loading, Notify, Quasar } from "quasar"
import "@quasar/extras/material-icons/material-icons.css"
import "quasar/src/css/index.sass"

import App from "./App.vue"
import { frontendLog } from "./api/unimaster"
import { applyLocale, i18n, translate } from "./i18n"
import router from "./router"
import store from "./store"
import "./styles.scss"
import { checkForAppUpdateWithPrompt } from "./updater"
import { applyThemeMode, getStoredThemeMode } from "./utils/theme"
import { shouldCheckUpdatesOnLaunch } from "./utils/preferences"

function getPerfNow() {
  return typeof performance !== "undefined" ? performance.now() : Date.now()
}

function logStartupPerf(stage, details = {}) {
  const serializedDetails = Object.entries(details)
    .map(([key, value]) => `${key}=${value}`)
    .join(" ")

  const message = `[perf][startup][${stage}]${serializedDetails ? ` ${serializedDetails}` : ""}`
  console.info(message)
  frontendLog("info", message).catch(() => {})
}

const appBootstrapStartedAt = getPerfNow()
const app = createApp(App)

app.use(router)
app.use(store)
app.use(i18n)
app.use(Quasar, {
  plugins: {
    Dialog,
    Loading,
    Notify,
  },
  config: {
    notify: {
      position: "top",
      timeout: 2600,
      textColor: "white",
    },
    loading: {
      delay: 120,
      message: translate("common.messages.processing"),
    },
  },
})

applyLocale(i18n.global.locale.value)

const isMac = typeof navigator !== "undefined" && /Mac/i.test(navigator.platform || navigator.userAgent)
const isTauriRuntime = typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__)
const allowMacThemeSwitch = !isMac || import.meta.env.DEV

applyThemeMode(allowMacThemeSwitch ? getStoredThemeMode() : "light", Dark)

if (typeof document !== "undefined") {
  document.documentElement.classList.toggle("platform-mac", isMac)
  document.body.classList.toggle("platform-mac", isMac)
  document.documentElement.classList.toggle("mac-theme-locked", isMac && !allowMacThemeSwitch)
  document.body.classList.toggle("mac-theme-locked", isMac && !allowMacThemeSwitch)
  document.documentElement.classList.toggle("runtime-tauri", isTauriRuntime)
  document.body.classList.toggle("runtime-tauri", isTauriRuntime)
}

app.mount("#app")
logStartupPerf("app-mounted", {
  totalMs: Math.round(getPerfNow() - appBootstrapStartedAt),
})

router.isReady().then(() => {
  logStartupPerf("router-ready", {
    totalMs: Math.round(getPerfNow() - appBootstrapStartedAt),
    checkUpdatesOnLaunch: shouldCheckUpdatesOnLaunch(),
  })

  if (shouldCheckUpdatesOnLaunch()) {
    checkForAppUpdateWithPrompt()
  }
})
