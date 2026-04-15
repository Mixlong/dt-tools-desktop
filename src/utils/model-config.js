import {
  UPGRADE_CAN_BAUD_OPTIONS,
  UPGRADE_POWER_VOLTAGE_OPTIONS,
  UPGRADE_PROTOCOL_TYPE_OPTIONS,
  UPGRADE_UART_BAUD_OPTIONS,
  UPGRADE_VLK5V_OPTIONS,
  buildUpgradeCqCode,
} from "@/utils/upgrade-cq"

export const MODEL_SOURCE_TYPE_LABELS = {
  product: "大货",
  sample: "送样",
}

export const MODEL_COMM_TYPE_LABELS = {
  0: "3.3V 串口",
  1: "5V 串口",
  2: "CAN",
}

export const MODEL_FRAME_TYPE_LABELS = {
  0: "串口默认",
  1: "标准帧",
  2: "扩展帧",
}

export const MODEL_CQ_FRAME_ID_LABELS = {
  1: "默认帧ID",
  2: "自定义帧ID",
}

export const MODEL_BURN_FILE_TYPE_LABELS = {
  0: "BOOT",
  1: "APP",
  2: "UI",
  3: "CFG",
}

function extractOptionAlias(option) {
  const label = String(option?.label || "")
  const parts = label.split("/")
  return String(parts[parts.length - 1] || label).trim().toLowerCase()
}

function getOptionLabel(options, value, fallback = "--") {
  const normalized = Number(value)
  return options.find((item) => Number(item.value) === normalized)?.label || fallback
}

function resolveBaudCode(commType, value, fallback) {
  const options = Number(commType) === 2 ? UPGRADE_CAN_BAUD_OPTIONS : UPGRADE_UART_BAUD_OPTIONS
  const normalized = Number(value)

  if (options.some((item) => Number(item.value) === normalized)) {
    return normalized
  }

  const aliases = [
    String(value ?? "").trim().toLowerCase(),
    Number.isFinite(normalized) ? String(normalized) : "",
  ].filter(Boolean)

  const matchedOption = options.find((option) => {
    const optionAlias = extractOptionAlias(option)
    return aliases.some((alias) => {
      if (alias === optionAlias) {
        return true
      }

      if (Number(commType) === 2) {
        const compactAlias = alias.replace(/khz|k/gi, "")
        const compactOption = optionAlias.replace(/khz|k/gi, "")
        return compactAlias === compactOption
      }

      return false
    })
  })

  return Number(matchedOption?.value ?? fallback)
}

export function getModelSourceTypeLabel(value) {
  return MODEL_SOURCE_TYPE_LABELS[String(value || "").trim()] || "--"
}

export function getModelCommTypeLabel(value) {
  return MODEL_COMM_TYPE_LABELS[Number(value)] || "--"
}

export function getModelBaudRateLabel(commType, value) {
  return Number(commType) === 2
    ? getOptionLabel(UPGRADE_CAN_BAUD_OPTIONS, value)
    : getOptionLabel(UPGRADE_UART_BAUD_OPTIONS, value)
}

export function getModelFrameTypeLabel(value) {
  return MODEL_FRAME_TYPE_LABELS[Number(value)] || "--"
}

export function getModelPowerVoltageLabel(value) {
  return getOptionLabel(UPGRADE_POWER_VOLTAGE_OPTIONS, value)
}

export function getModelVlk5vLabel(value) {
  return getOptionLabel(UPGRADE_VLK5V_OPTIONS, value)
}

export function getModelProtocolTypeLabel(value) {
  return getOptionLabel(UPGRADE_PROTOCOL_TYPE_OPTIONS, value)
}

export function getModelBurnFileTypeLabel(value) {
  return MODEL_BURN_FILE_TYPE_LABELS[Number(value)] || "--"
}

export function getModelCqFrameIdLabel(value) {
  return MODEL_CQ_FRAME_ID_LABELS[Number(value)] || "--"
}

export function buildUpgradeCqStateFromModelConfig(config, options = {}) {
  if (!config) {
    return null
  }

  const {
    transportProfile = "app",
    burnFileType,
  } = options
  const commType = Number(config.serialLevel)
  const isCan = commType === 2
  const useBootTransport = transportProfile === "boot"

  return {
    commType,
    baudCode: isCan
      ? resolveBaudCode(commType, useBootTransport ? config.bootCanRate : config.canRate, 0x08)
      : resolveBaudCode(commType, useBootTransport ? config.bootBaudRate : config.baudRate, 0x0B),
    frameType: isCan ? Number(useBootTransport ? config.bootMsgType : config.msgType) : 0,
    powerVoltage: Number(config.cqPowerVoltage),
    vlk5vEnabled: Number(config.cqVlk5vSwitch),
    protocolType: Number(config.cqUpgradeProtocolType),
    fileFormat: 0x01,
    frameId: isCan ? Number(config.cqFrameId) : 0x00,
    specialFrameValue: config.cqCustomFrameId || "",
    burnFileType: Number(burnFileType ?? config.cqBurnFileType),
  }
}

export function buildCqCodeFromModelConfig(config, options = {}) {
  if (!config) {
    return ""
  }

  const cqState = buildUpgradeCqStateFromModelConfig(config, options)
  return buildUpgradeCqCode(cqState, Number(cqState.burnFileType))
}
