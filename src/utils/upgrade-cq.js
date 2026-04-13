const CQ_PREFIX = "CQ"
const CQ_BODY_LENGTH = 8

export const UPGRADE_CQ_PASSWORD = "dtkj2026"

export const UPGRADE_COMM_TYPE_OPTIONS = [
  { label: "3.3V 串口", value: 0x00 },
  { label: "5V 串口", value: 0x01 },
  { label: "CAN", value: 0x02 },
]

export const UPGRADE_UART_COMM_TYPE_OPTIONS = UPGRADE_COMM_TYPE_OPTIONS.filter((item) => item.value !== 0x02)

export const UPGRADE_POWER_VOLTAGE_OPTIONS = [
  { label: "12V", value: 0x00 },
  { label: "24V", value: 0x01 },
]

export const UPGRADE_VLK5V_OPTIONS = [
  { label: "关闭", value: 0x00 },
  { label: "打开", value: 0x01 },
]

export const UPGRADE_PROTOCOL_TYPE_OPTIONS = [
  { label: "通用彩屏", value: 0x01 },
  { label: "通用段码屏", value: 0x02 },
  { label: "高标", value: 0x03 },
  { label: "华芯微特", value: 0x04 },
  { label: "开阳", value: 0x05 },
  { label: "LIME", value: 0x06 },
  { label: "SPARROW", value: 0x07 },
  { label: "美的", value: 0x08 },
  { label: "IOT", value: 0x09 },
  { label: "K71U", value: 0x0A },
]

export const UPGRADE_UART_BAUD_OPTIONS = [
  { label: "0x01 / 1200", value: 0x01 },
  { label: "0x02 / 2400", value: 0x02 },
  { label: "0x03 / 4800", value: 0x03 },
  { label: "0x04 / 9600", value: 0x04 },
  { label: "0x05 / 14400", value: 0x05 },
  { label: "0x06 / 19200", value: 0x06 },
  { label: "0x07 / 38400", value: 0x07 },
  { label: "0x08 / 43000", value: 0x08 },
  { label: "0x09 / 57600", value: 0x09 },
  { label: "0x0A / 76800", value: 0x0A },
  { label: "0x0B / 115200", value: 0x0B },
  { label: "0x0C / 128000", value: 0x0C },
]

export const UPGRADE_CAN_BAUD_OPTIONS = [
  { label: "0x01 / 100k", value: 0x01 },
  { label: "0x02 / 125k", value: 0x02 },
  { label: "0x03 / 150k", value: 0x03 },
  { label: "0x04 / 200k", value: 0x04 },
  { label: "0x05 / 250k", value: 0x05 },
  { label: "0x06 / 300k", value: 0x06 },
  { label: "0x07 / 400k", value: 0x07 },
  { label: "0x08 / 500k", value: 0x08 },
  { label: "0x09 / 600k", value: 0x09 },
  { label: "0x0A / 900k", value: 0x0A },
]

export function getUpgradeBaudOptions(commType) {
  return Number(commType) === 0x02 ? UPGRADE_CAN_BAUD_OPTIONS : UPGRADE_UART_BAUD_OPTIONS
}

export function getUpgradeFrameTypeOptions(commType) {
  return Number(commType) === 0x02
    ? [
        { label: "标准帧", value: 0x01 },
        { label: "扩展帧", value: 0x02 },
      ]
    : [
        { label: "串口默认", value: 0x00 },
      ]
}

export function getUpgradeFrameIdOptions(commType) {
  return Number(commType) === 0x02
    ? [
        { label: "默认帧ID", value: 0x01 },
        { label: "特殊帧ID", value: 0x02 },
      ]
    : [
        { label: "默认帧ID", value: 0x01 },
      ]
}

export function createDefaultUpgradeCqState() {
  return {
    commType: 0x00,
    baudCode: 0x0B,
    frameType: 0x00,
    powerVoltage: 0x00,
    vlk5vEnabled: 0x00,
    protocolType: 0x01,
    frameId: 0x01,
    specialFrameValue: "",
  }
}

function normalizeNibble(value, fieldName) {
  const normalized = Number(value)
  if (!Number.isInteger(normalized) || normalized < 0 || normalized > 0x0F) {
    throw new Error(`${fieldName} 超出编码范围`)
  }
  return normalized.toString(16).toUpperCase()
}

function normalizeSpecialFrameValue(value) {
  const normalized = String(value || "").trim().toUpperCase()
  if (!normalized) {
    return ""
  }

  const segments = normalized
    .split("-")
    .map((item) => item.trim().toUpperCase())
    .filter(Boolean)

  if (segments.length === 0) {
    return ""
  }

  if (!segments.every((item) => /^[0-9A-F]+$/.test(item))) {
    throw new Error("特殊帧ID格式不正确")
  }

  if (segments.length === 1) {
    return `${segments[0]}-${segments[0]}`
  }

  return `${segments[0]}-${segments[1]}`
}

export function buildUpgradeCqCode(config, burnFileType) {
  const frameId = Number(config.frameId ?? 0x01)
  const frameType = Number(config.commType) === 0x02 ? Number(config.frameType ?? 0x01) : 0x00
  const body = [
    normalizeNibble(config.commType, "通讯类型"),
    normalizeNibble(config.baudCode, "波特率"),
    normalizeNibble(frameType, "帧类型"),
    normalizeNibble(config.powerVoltage, "供电电压"),
    normalizeNibble(config.vlk5vEnabled, "VLK5V 开关"),
    normalizeNibble(config.protocolType, "升级协议类型"),
    normalizeNibble(burnFileType, "烧录文件类型"),
    normalizeNibble(frameId, "帧ID"),
  ].join("")

  const specialFrameValue = normalizeSpecialFrameValue(config.specialFrameValue)
  if (frameId === 0x02) {
    if (!specialFrameValue) {
      throw new Error("请选择特殊帧ID后填写自定义帧ID")
    }
    return `${CQ_PREFIX}${body}-${specialFrameValue}`
  }

  return `${CQ_PREFIX}${body}`
}

export function parseUpgradeCqCode(input) {
  const normalized = String(input || "").trim().toUpperCase()
  if (!normalized) {
    throw new Error("CQ 配置不能为空")
  }
  if (!normalized.startsWith(CQ_PREFIX)) {
    throw new Error("CQ 配置必须以 CQ 开头")
  }

  const bodyStart = CQ_PREFIX.length
  const bodyEnd = bodyStart + CQ_BODY_LENGTH
  const body = normalized.slice(bodyStart, bodyEnd)
  if (body.length !== CQ_BODY_LENGTH) {
    throw new Error("CQ 配置长度不正确")
  }
  if (!/^[0-9A-F]{8}$/.test(body)) {
    throw new Error("CQ 配置格式不正确")
  }

  const suffixPart = normalized.slice(bodyEnd).replace(/^-/, "")
  const values = body.split("").map((item) => Number.parseInt(item, 16))
  return {
    cqCode: normalized,
    commType: values[0],
    baudCode: values[1],
    frameType: values[2],
    powerVoltage: values[3],
    vlk5vEnabled: values[4],
    protocolType: values[5],
    burnFileType: values[6],
    frameId: values[7],
    specialFrameValue: normalizeSpecialFrameValue(suffixPart),
  }
}

export function getUpgradeBurnFileType(kind) {
  switch (kind) {
    case "boot":
      return 0
    case "app":
      return 1
    case "ui":
      return 2
    case "config":
      return 3
    default:
      return 1
  }
}
