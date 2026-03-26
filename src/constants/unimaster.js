export const MODEL_OPTIONS = [
  { label: "UniMaster", value: "UniMaster" },
  { label: "UniMaster-Color", value: "UniMaster-Color" },
  { label: "UniMaster-Pro", value: "UniMaster-Pro" },
]

export const FRAME_TYPE_OPTIONS = [
  { label: "串口 / 默认", value: 0 },
  { label: "CAN 标准帧", value: 1 },
  { label: "CAN 扩展帧", value: 2 },
]

export const SERIAL_BAUD_OPTIONS = [
  { code: 0x01, label: "1200" },
  { code: 0x02, label: "2400" },
  { code: 0x03, label: "4800" },
  { code: 0x04, label: "9600" },
  { code: 0x05, label: "14400" },
  { code: 0x06, label: "19200" },
  { code: 0x07, label: "38400" },
  { code: 0x08, label: "43000" },
  { code: 0x09, label: "57600" },
  { code: 0x0a, label: "76800" },
  { code: 0x0b, label: "115200" },
  { code: 0x0c, label: "128000" },
]

export const CAN_BAUD_OPTIONS = [
  { code: 0x01, label: "100k" },
  { code: 0x02, label: "125k" },
  { code: 0x03, label: "150k" },
  { code: 0x04, label: "200k" },
  { code: 0x05, label: "250k" },
  { code: 0x06, label: "300k" },
  { code: 0x07, label: "400k" },
  { code: 0x08, label: "500k" },
  { code: 0x09, label: "600k" },
  { code: 0x0a, label: "900k" },
]

export const REALTIME_COMM_OPTIONS = [
  { label: "3.3V 串口", value: 0 },
  { label: "5V 串口", value: 1 },
  { label: "CAN", value: 2 },
]

export const OFFLINE_COMM_OPTIONS = [
  { label: "串口", value: 0 },
  { label: "CAN", value: 1 },
]

export const OFFLINE_BAUD_COMM_OPTIONS = [
  { label: "3.3V 串口", value: 1 },
  { label: "5V 串口", value: 2 },
  { label: "CAN", value: 3 },
]

export const POWER_OPTIONS = [
  { label: "12V", value: 0 },
  { label: "24V", value: 1 },
]

export const PROTOCOL_OPTIONS = [
  { label: "通用彩屏", value: 0x01 },
  { label: "通用段码屏", value: 0x02 },
  { label: "高标", value: 0x03 },
  { label: "华芯微特", value: 0x04 },
  { label: "开阳", value: 0x05 },
  { label: "LIME", value: 0x06 },
  { label: "SPARROW", value: 0x07 },
  { label: "美的", value: 0x08 },
  { label: "IOT", value: 0x09 },
  { label: "K71U", value: 0x0a },
]

export const FILE_KIND_OPTIONS = [
  { label: "BOOT", value: "boot" },
  { label: "APP", value: "app" },
  { label: "UI", value: "ui" },
  { label: "配置文件", value: "config" },
]

export const VERSION_TYPE_OPTIONS = [
  { label: "DT SN", value: 0 },
  { label: "客户 SN", value: 1 },
  { label: "HW", value: 2 },
  { label: "BOOT", value: 3 },
  { label: "APP", value: 4 },
  { label: "BLE 升级标志", value: 5 },
  { label: "BLE_CRC", value: 6 },
  { label: "QR_CODE", value: 7 },
  { label: "UI", value: 8 },
]

export function getFileTypeCode(fileName) {
  const ext = fileName.split(".").pop()?.toLowerCase()
  switch (ext) {
    case "txt":
      return 0
    case "bin":
      return 1
    case "json":
      return 2
    case "ini":
      return 3
    case "hex":
      return 4
    default:
      return 0xff
  }
}

export function getBaudOptionsByCommType(commType, offline = false) {
  if (offline) {
    return commType === 3 ? CAN_BAUD_OPTIONS : SERIAL_BAUD_OPTIONS
  }
  return commType === 2 ? CAN_BAUD_OPTIONS : SERIAL_BAUD_OPTIONS
}
