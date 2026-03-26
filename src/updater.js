import { ElLoading, ElMessage, ElMessageBox } from "element-plus"
import { invoke } from "@tauri-apps/api/core"
import { openUrl } from "@tauri-apps/plugin-opener"
import { check } from "@tauri-apps/plugin-updater"

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

function isTauriDesktop() {
  return typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__)
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
    ElMessage.warning("更新已安装，请手动重启应用")
  }
}

function getErrorMessage(error) {
  return String(error?.message || error || "未知错误")
}

function isUpdaterNotReady(error) {
  const message = getErrorMessage(error).toLowerCase()
  return UPDATER_CONFIGURATION_HINTS.some((hint) => message.includes(hint.toLowerCase()))
}

export async function checkForAppUpdateWithPrompt() {
  if (!isTauriDesktop()) return
  let loading = null
  let manualDownloadUrl = ""
  try {
    const update = await check()
    if (!update) return
    manualDownloadUrl = resolveManualDownloadUrl(String(update.rawJson?.url || ""))

    await ElMessageBox.confirm(`检测到新版本 ${update.version}，是否立即下载并安装？`, "发现新版本", {
      confirmButtonText: "立即更新",
      cancelButtonText: "稍后",
      type: "info",
      closeOnClickModal: false,
    })

    loading = ElLoading.service({
      lock: true,
      text: "正在下载更新...",
      background: "rgba(0, 0, 0, 0.45)",
    })

    await update.download(() => {
      loading?.setText("下载完成，正在安装...")
    })
    await update.install()
    ElMessage.success("更新已安装，正在自动重启...")
    await restartApp()
  } catch (error) {
    if (error === "cancel" || error === "close") return
    if (isUpdaterNotReady(error)) {
      console.warn("[updater] updater is not configured correctly, skip startup check:", error)
      return
    }
    const message = getErrorMessage(error)
    await ElMessageBox.confirm(`检查或安装更新失败：${message}\n\n是否打开手动下载链接？`, "更新失败", {
      confirmButtonText: "手动下载",
      cancelButtonText: "取消",
      type: "error",
      closeOnClickModal: false,
    })
    await openUrl(resolveManualDownloadUrl(manualDownloadUrl))
  } finally {
    loading?.close()
  }
}
