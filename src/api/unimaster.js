import { invoke } from "@tauri-apps/api/core"

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

export function prepareOfflineUpgrade(request) {
  return invoke("prepare_offline_upgrade", { request })
}

export function setMeterConfigTransport(request) {
  return invoke("set_meter_config_transport", { request })
}

export function readMeterConfig() {
  return invoke("read_meter_config")
}

export function writeMeterConfig(bytes) {
  return invoke("write_meter_config", { bytes })
}

export function saveTextFile(path, contents) {
  return invoke("save_text_file", { path, contents })
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
