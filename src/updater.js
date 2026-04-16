import { invoke } from "@tauri-apps/api/core"
import { openUrl } from "@tauri-apps/plugin-opener"
import { check } from "@tauri-apps/plugin-updater"
import { frontendLog } from "@/api/unimaster"
import { translate } from "@/i18n"
import { confirmAction, notifyInfo, withLoading } from "@/services/ui"
import tauriConfig from "../src-tauri/tauri.conf.json"

const UPDATER_CONFIGURATION_HINTS = [
  "REPLACE_WITH_DT_TOOLS_PUBLIC_KEY",
  "signature pubkey",
  "updater key",
  "invalid base64",
  "invalid public key",
  "missing field `pubkey`",
  "missing required key",
  "could not fetch a valid release json from the remote",
  "nosuchkey",
  "status code 404",
]

const APP_VERSION = String(tauriConfig?.version || "0.1.0")

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

function isTauriDesktop() {
  return typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__)
}

function isUpdaterConfigured() {
  const updater = tauriConfig?.plugins?.updater
  const pubkey = String(updater?.pubkey || "").trim()
  const endpoints = Array.isArray(updater?.endpoints) ? updater.endpoints.filter(Boolean) : []

  return Boolean(
    updater?.active
    && endpoints.length
    && pubkey
    && pubkey !== "REPLACE_WITH_DT_TOOLS_PUBLIC_KEY",
  )
}

function resolveManualDownloadUrl(updateUrl) {
  const fallback = "https://bikewise.oss-cn-shenzhen.aliyuncs.com/d-space/dt-tools-desktop/darwin/DT-Tools.tar.gz"
  if (!updateUrl) return fallback
  try {
    const parsed = new URL(updateUrl)
    parsed.search = ""
    parsed.hash = ""
    return parsed.toString()
  } catch {
    return fallback
  }
}

async function restartApp() {
  try {
    await invoke("restart_app")
  } catch {
    notifyInfo(translate("updater.restartManually"))
  }
}

function getErrorMessage(error) {
  return String(error?.message || error || translate("updater.unknownError"))
}

function isUpdaterNotReady(error) {
  const message = getErrorMessage(error).toLowerCase()
  return UPDATER_CONFIGURATION_HINTS.some((hint) => message.includes(hint.toLowerCase()))
}

export async function getRemoteAppVersion() {
  if (!isTauriDesktop() || !isUpdaterConfigured()) {
    return APP_VERSION
  }

  const update = await check()
  return String(update?.version || APP_VERSION)
}

export async function checkForAppUpdateWithPrompt() {
  const startedAt = getPerfNow()

  if (!isTauriDesktop() || !isUpdaterConfigured()) {
    logStartupPerf("updater-skip", {
      reason: !isTauriDesktop() ? "not-tauri" : "not-configured",
    })
    return
  }

  let manualDownloadUrl = ""

  try {
    logStartupPerf("updater-check-start")
    const update = await check()
    if (!update) {
      logStartupPerf("updater-check-none", {
        totalMs: Math.round(getPerfNow() - startedAt),
      })
      return
    }

    logStartupPerf("updater-check-found", {
      totalMs: Math.round(getPerfNow() - startedAt),
      version: update.version,
    })

    manualDownloadUrl = resolveManualDownloadUrl(String(update.rawJson?.url || ""))

    const confirmed = await confirmAction({
      title: translate("updater.availableTitle"),
      message: translate("updater.availableMessage", { version: update.version }),
      ok: translate("updater.updateNow"),
      cancel: translate("updater.later"),
    })

    if (!confirmed) {
      logStartupPerf("updater-declined", {
        totalMs: Math.round(getPerfNow() - startedAt),
      })
      return
    }

    await withLoading(async () => {
      await update.download()
      await update.install()
    }, { message: translate("updater.downloading") })

    logStartupPerf("updater-installed", {
      totalMs: Math.round(getPerfNow() - startedAt),
    })

    notifyInfo(translate("updater.installedRestarting"))
    await restartApp()
  } catch (error) {
    if (isUpdaterNotReady(error)) {
      logStartupPerf("updater-skip", {
        totalMs: Math.round(getPerfNow() - startedAt),
        reason: "not-ready",
      })
      return
    }

    logStartupPerf("updater-fail", {
      totalMs: Math.round(getPerfNow() - startedAt),
      error: getErrorMessage(error),
    })

    const openManual = await confirmAction({
      title: translate("updater.failedTitle"),
      message: translate("updater.failedMessage", { message: getErrorMessage(error) }),
      ok: translate("updater.manualDownload"),
      cancel: translate("common.actions.cancel"),
    })

    if (openManual) {
      await openUrl(resolveManualDownloadUrl(manualDownloadUrl))
    }
  }
}
