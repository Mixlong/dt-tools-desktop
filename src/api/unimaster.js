import { invoke } from "@tauri-apps/api/core"

const MODEL_QUERY_BASE_URL = "http://192.168.2.114:8111"
const LEGACY_UNIMASTER_BASE_URL = "http://test-pucs.riding-evolved.com"
const REMOTE_ASSET_CODES = new Set([
  "UniMaster_Upgrade_APP",
  "UniMaster_Upgrade_UI2",
])

function normalizeBaseUrl(baseUrl, fallbackBaseUrl = LEGACY_UNIMASTER_BASE_URL) {
  const normalized = String(baseUrl || fallbackBaseUrl || "").trim().replace(/\/+$/, "")
  return normalized || LEGACY_UNIMASTER_BASE_URL
}

async function buildLegacySignedHeaders(pathname) {
  return invoke("build_legacy_signed_headers", {
    pathname: String(pathname || "").trim(),
  })
}

export function listSerialPorts() {
  return invoke("list_serial_ports")
}

export function getSerialStatus() {
  return invoke("serial_status")
}

export function connectSerial(portName, baudRate) {
  return invoke("connect_serial", { portName, baudRate })
}

export function disconnectSerial() {
  return invoke("disconnect_serial")
}

export function sendRawCommand(request) {
  return invoke("send_raw_command", { request })
}

export function readVersionSnapshot() {
  return invoke("read_version_snapshot")
}

export function writeVersionInfo(request) {
  return invoke("write_version_info", { request })
}

export function readFlags() {
  return invoke("read_flags")
}

export function writeFlag(request) {
  return invoke("write_flag", { request })
}

export function switchLanguage(language) {
  return invoke("switch_language", { language })
}

export function setRealtimeScreen(screen) {
  return invoke("set_realtime_screen", { screen })
}

export function readAccessState() {
  return invoke("read_access_state")
}

export function initRealtimeUpgrade(request) {
  return invoke("init_realtime_upgrade", { request })
}

export function performRealtimeUpgrade(request) {
  return invoke("perform_realtime_upgrade", { request })
}

export function performUniMasterVersionUpgrade(request) {
  return invoke("perform_unimaster_version_upgrade", { request })
}

export function cancelRealtimeUpgrade() {
  return invoke("cancel_realtime_upgrade")
}

export function prepareOfflineUpgrade(request) {
  return invoke("prepare_offline_upgrade", { request })
}

export function loadProgramBurningBundle(codeOrSn) {
  return invoke("load_program_burning_bundle", { codeOrSn })
}

export async function queryModelConfigByComputerName(computerName) {
  const normalizedComputerName = String(computerName || "").trim().toUpperCase()
  if (!normalizedComputerName) {
    throw new Error("请输入型号名称")
  }

  const url = `${MODEL_QUERY_BASE_URL}/model/config/computer/name/${encodeURIComponent(normalizedComputerName)}`
  const response = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "application/json",
    },
  })

  if (!response.ok) {
    throw new Error(`型号查询失败，状态码 ${response.status}`)
  }

  const payload = await response.json()
  if (Number(payload?.code) !== 200) {
    throw new Error(String(payload?.msg || "型号查询返回异常"))
  }

  return payload?.data ?? null
}

export async function queryCommonDictType(dictType) {
  const normalizedDictType = String(dictType || "").trim()
  if (!normalizedDictType) {
    throw new Error("请输入字典类型")
  }

  const url = `${MODEL_QUERY_BASE_URL}/common/type/${encodeURIComponent(normalizedDictType)}`
  const response = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "application/json",
    },
  })

  if (!response.ok) {
    throw new Error(`字典查询失败，状态码 ${response.status}`)
  }

  const payload = await response.json()
  if (Number(payload?.code) !== 200) {
    throw new Error(String(payload?.msg || "字典查询返回异常"))
  }

  return Array.isArray(payload?.data) ? payload.data : []
}

export async function queryUpgradeResource(code, options = {}) {
  const normalizedCode = String(code || "").trim()
  if (!REMOTE_ASSET_CODES.has(normalizedCode)) {
    throw new Error("升级资源类型不受支持")
  }

  const baseUrl = normalizeBaseUrl(options.baseUrl)
  const pathname = "/sts/type/update"
  const url = `${baseUrl}${pathname}?code=${encodeURIComponent(normalizedCode)}`
  const signedHeaders = await buildLegacySignedHeaders(pathname)
  const response = await fetch(url, {
    method: "GET",
    headers: {
      Accept: "application/json",
      ...signedHeaders,
    },
  })

  if (!response.ok) {
    let payload = null
    try {
      payload = await response.json()
    } catch {
      payload = null
    }

    const backendMessage = String(payload?.msg || payload?.message || "").trim()
    const error = new Error("远程升级资源获取失败")
    error.statusCode = response.status
    error.backendMessage = backendMessage
    error.userMessage = "远程升级资源获取失败"
    throw error
  }

  const payload = await response.json()
  if (Number(payload?.code) !== 200) {
    const backendMessage = String(payload?.msg || payload?.message || "升级资源查询返回异常").trim()
    const error = new Error("远程升级资源获取失败")
    error.statusCode = 200
    error.backendMessage = backendMessage
    error.userMessage = "远程升级资源获取失败"
    throw error
  }

  return payload?.data ?? null
}

export function resolveRemoteAssetUrl(url, options = {}) {
  const normalizedUrl = String(url || "").trim()
  if (!normalizedUrl) {
    return ""
  }

  if (/^https?:\/\//i.test(normalizedUrl)) {
    return normalizedUrl
  }

  const baseUrl = normalizeBaseUrl(options.baseUrl)
  return `${baseUrl}${normalizedUrl.startsWith("/") ? "" : "/"}${normalizedUrl}`
}

export async function downloadUpgradeFileBytes(url, options = {}) {
  const resolvedUrl = resolveRemoteAssetUrl(url, options)
  if (!resolvedUrl) {
    throw new Error("升级文件地址为空")
  }

  const response = await fetch(resolvedUrl)
  if (!response.ok) {
    throw new Error(`升级文件下载失败，状态码 ${response.status}`)
  }

  const buffer = await response.arrayBuffer()
  return Array.from(new Uint8Array(buffer))
}

export function getLegacyUniMasterBaseUrl() {
  return LEGACY_UNIMASTER_BASE_URL
}

export function setMeterConfigTransport(request) {
  return invoke("set_meter_config_transport", { request })
}

export function readMeterConfig(request) {
  return invoke("read_meter_config", { request })
}

export function writeMeterConfig(bytes) {
  return invoke("write_meter_config", { bytes })
}

export function sendMeterConfigHeartbeat(request) {
  return invoke("send_meter_config_heartbeat", { request })
}

export function saveTextFile(path, contents) {
  return invoke("save_text_file", { path, contents })
}

export function frontendLog(level, message) {
  return invoke("frontend_log", { level, message })
}

export async function fileToBytes(file) {
  const buffer = await file.arrayBuffer()
  return Array.from(new Uint8Array(buffer))
}

export function parseHexInput(input) {
  const cleaned = input.replace(/0x/gi, " ").replace(/[^0-9a-f]/gi, " ")
  const tokens = cleaned
    .split(/\s+/)
    .map((item) => item.trim())
    .filter(Boolean)

  return tokens.map((item) => Number.parseInt(item, 16))
}

export function formatHex(bytes) {
  return (bytes || []).map((value) => value.toString(16).padStart(2, "0").toUpperCase()).join(" ")
}
