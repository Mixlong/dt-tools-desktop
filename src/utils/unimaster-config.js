const encodeByte = (value) => Number(value ?? 0) & 0xff

const encodeHighByte = (value) => (Number(value ?? 0) >> 8) & 0xff

const encodeLowByte = (value) => Number(value ?? 0) & 0xff

const readUInt16 = (high, low) => ((Number(high) & 0xff) << 8) | (Number(low) & 0xff)

const stringToAsciiCode = (value, index) => {
  const text = String(value || "").padEnd(index + 1, " ")
  return text.charCodeAt(index) & 0xff
}

const selectOptions = (items) => items.map((item) => ({ label: String(item), value: item }))

export const METER_UART_BAUD_OPTIONS = [
  { label: "1200", value: 0x01 },
  { label: "2400", value: 0x02 },
  { label: "4800", value: 0x03 },
  { label: "9600", value: 0x04 },
  { label: "14400", value: 0x05 },
  { label: "19200", value: 0x06 },
  { label: "38400", value: 0x07 },
  { label: "43000", value: 0x08 },
  { label: "57600", value: 0x09 },
  { label: "76800", value: 0x0a },
  { label: "115200", value: 0x0b },
  { label: "128000", value: 0x0c },
]

export const METER_CAN_BAUD_OPTIONS = [
  { label: "100K", value: 0x01 },
  { label: "125K", value: 0x02 },
  { label: "150K", value: 0x03 },
  { label: "200K", value: 0x04 },
  { label: "250K", value: 0x05 },
  { label: "300K", value: 0x06 },
  { label: "400K", value: 0x07 },
  { label: "500K", value: 0x08 },
  { label: "600K", value: 0x09 },
  { label: "900K", value: 0x0a },
]

export const METER_TRANSPORT_OPTIONS = [
  { label: "UART", value: 0x01 },
  { label: "CAN", value: 0x02 },
]

export const METER_CAN_FRAME_OPTIONS = [
  { label: "标准帧", value: 0x01 },
  { label: "扩展帧", value: 0x02 },
]

export const VOLTAGE_OPTIONS = selectOptions([24, 36, 48, 52, 60, 72])
export const POWER_GEAR_OPTIONS = selectOptions([3, 4, 5, 9])
export const ASSIST_START_MAGNET_OPTIONS = selectOptions(Array.from({ length: 63 }, (_, index) => index + 2))
export const ASSIST_LIMIT_OPTIONS = selectOptions(Array.from({ length: 256 }, (_, index) => index))
export const SLOW_START_OPTIONS = selectOptions([0, 1, 2, 3])
export const SPEED_STEEL_OPTIONS = selectOptions(Array.from({ length: 16 }, (_, index) => index))
export const BATTERY_VOLTAGE_CHANGE_OPTIONS = selectOptions(Array.from({ length: 60 }, (_, index) => index + 1))
export const SMOOTH_LEVEL_OPTIONS = selectOptions(Array.from({ length: 11 }, (_, index) => index))
export const BUS_TIMEOUT_OPTIONS = selectOptions(Array.from({ length: 251 }, (_, index) => index + 5))

export const WHEEL_DIAMETER_OPTIONS = [
  { label: "16", value: 0 },
  { label: "18", value: 1 },
  { label: "20", value: 2 },
  { label: "22", value: 3 },
  { label: "24", value: 4 },
  { label: "26", value: 5 },
  { label: "700C", value: 6 },
  { label: "28", value: 7 },
  ...Array.from({ length: 92 }, (_, index) => {
    const actual = (index + 8) * 0.5
    return {
      label: String(actual),
      value: index + 8,
    }
  }),
]

export const UNIT_OPTIONS = [
  { label: "公制", value: 0 },
  { label: "英制", value: 1 },
]

export const AGREEMENT_OPTIONS = [
  { label: "KM5S", value: 0 },
  { label: "锂电 2 号", value: 1 },
  { label: "八方", value: 2 },
  { label: "J 协议", value: 3 },
]

export const POWER_MODE_OPTIONS = [
  { label: "控制器上报电压，仪表计算电量", value: 0 },
  { label: "控制器上报电量", value: 1 },
  { label: "仪表检测电压，仪表计算电量", value: 2 },
]

export const LOGO_OPTIONS = [
  { label: "迪太界面", value: 0 },
  { label: "中性界面", value: 1 },
  { label: "客户界面", value: 2 },
  ...Array.from({ length: 7 }, (_, index) => ({
    label: `保留项 ${index + 3}`,
    value: index + 3,
  })),
]

export const EBIKE_NAME_OPTIONS = [
  { label: "Pace500", value: 0 },
  { label: "Pace350", value: 1 },
  { label: "Level", value: 2 },
  { label: "Sinch", value: 3 },
  { label: "Aventure", value: 4 },
  { label: "Pace", value: 5 },
  { label: "Sinch ST", value: 6 },
  { label: "Pace500 V2", value: 7 },
  { label: "Pace350 V2", value: 8 },
  { label: "Soltera", value: 9 },
  { label: "Soltera-7S", value: 10 },
  { label: "Cruiser", value: 11 },
  { label: "Aventure Pro", value: 12 },
]

export const BOOLEAN_YES_NO_OPTIONS = [
  { label: "是", value: 1 },
  { label: "否", value: 0 },
]

export const BOOLEAN_REVERSED_OPTIONS = [
  { label: "是", value: 0 },
  { label: "否", value: 1 },
]

export const ASSIST_DIRECTION_OPTIONS = [
  { label: "助力正", value: 0 },
  { label: "助力反", value: 1 },
]

export const ROTATE_SPEED_LIMIT_OPTIONS = [
  { label: "正常转把", value: 0 },
  { label: "限速 6km/h", value: 1 },
]

export const SERIAL_LEVEL_OPTIONS = [
  { label: "3.3V", value: 0 },
  { label: "5V", value: 1 },
]

export const METER_CONFIG_DEFAULTS = {
  backlightBrightness: 5,
  sleepTime: 10,
  voltage: 48,
  undervoltage: 42,
  assistPercentage: 128,
  currentlimiting: 12,
  showWheelsize: 0,
  tiresSize: 0,
  powerGear: 5,
  assistStartMagnetNumber: 2,
  assistLimit: 25,
  factoryReset: 1,
  cruise: 1,
  rotateHandleSpeedLimit: 0,
  slowStart: 1,
  wheelDiameter: 5,
  perimeter: 0,
  unit: 0,
  agreement: 0,
  power: 0,
  speedSteel: 1,
  batteryVoltageChangeTime: 10,
  smoothLevel: 3,
  allLineErrTimeOut: 10,
  bluetooth: 0,
  rotateHandle: 0,
  turnOnPasswd: 1,
  assist: 0,
  ebikeName: 0,
  carModel: "AA",
  defaultGear: 0,
  logo: 0,
  startupPasswd: 2020,
  highMenuPasswd: 2020,
  motorSys: 0,
  batteryCap: 0,
  highSpeedBuzzerRemind: 0,
  serialLevel: 1,
  driveAssist: 1,
  buzzerSwitch: 0,
  menuPassword: 0,
}

export const METER_CONFIG_GROUPS = [
  {
    key: "basic",
    title: "基础显示",
    description: "电压、休眠、背光和轮径等基础参数。",
    fields: [
      {
        key: "backlightBrightness",
        label: "背光亮度",
        type: "select",
        options: [
          { label: "1", value: 1 },
          { label: "2", value: 2 },
          { label: "3", value: 3 },
          { label: "4", value: 4 },
          { label: "5", value: 5 },
          { label: "AUTO", value: 6 },
        ],
      },
      { key: "sleepTime", label: "休眠时间", type: "select", options: selectOptions(Array.from({ length: 11 }, (_, index) => index)), unit: "min" },
      { key: "voltage", label: "系统电压", type: "select", options: VOLTAGE_OPTIONS, unit: "V" },
      { key: "undervoltage", label: "欠压门限", type: "number", min: 0, step: 0.1, precision: 2, unit: "V" },
      { key: "wheelDiameter", label: "轮径代码", type: "select", options: WHEEL_DIAMETER_OPTIONS, unit: "inch" },
      { key: "perimeter", label: "周长", type: "number", min: 0, unit: "mm" },
      { key: "showWheelsize", label: "显示轮径", type: "number", min: 0 },
      { key: "tiresSize", label: "车轮宽度", type: "number", min: 0 },
      { key: "unit", label: "显示单位", type: "select", options: UNIT_OPTIONS },
      { key: "ebikeName", label: "车名", type: "select", options: EBIKE_NAME_OPTIONS },
      { key: "carModel", label: "车型代码", type: "text", maxlength: 2 },
      { key: "logo", label: "LOGO 界面", type: "select", options: LOGO_OPTIONS },
    ],
  },
  {
    key: "assist",
    title: "助力与速度",
    description: "助力方向、助力比例、测速和限速相关参数。",
    fields: [
      { key: "powerGear", label: "助力档位数", type: "select", options: POWER_GEAR_OPTIONS },
      { key: "assist", label: "助力方向", type: "select", options: ASSIST_DIRECTION_OPTIONS },
      { key: "assistStartMagnetNumber", label: "助力开始磁钢数", type: "select", options: ASSIST_START_MAGNET_OPTIONS },
      { key: "assistPercentage", label: "助力比例", type: "number", min: 0, max: 255 },
      { key: "rotateHandle", label: "转把分档", type: "select", options: BOOLEAN_YES_NO_OPTIONS },
      { key: "rotateHandleSpeedLimit", label: "转把限速", type: "select", options: ROTATE_SPEED_LIMIT_OPTIONS },
      { key: "slowStart", label: "缓启动参数", type: "select", options: SLOW_START_OPTIONS },
      { key: "speedSteel", label: "测速磁钢数", type: "select", options: SPEED_STEEL_OPTIONS },
      { key: "currentlimiting", label: "限流门限", type: "number", min: 0, unit: "A" },
      { key: "assistLimit", label: "助力限速门限", type: "select", options: ASSIST_LIMIT_OPTIONS, unit: "km/h" },
      { key: "smoothLevel", label: "速度平滑等级", type: "select", options: SMOOTH_LEVEL_OPTIONS },
      { key: "defaultGear", label: "默认档位", type: "number", min: 0, max: 9 },
    ],
  },
  {
    key: "protocol",
    title: "协议与接口",
    description: "协议、电量策略、蓝牙和串口等级等通信参数。",
    fields: [
      { key: "agreement", label: "协议", type: "select", options: AGREEMENT_OPTIONS },
      { key: "power", label: "电量计算方式", type: "select", options: POWER_MODE_OPTIONS },
      { key: "batteryVoltageChangeTime", label: "电量显示变化时间", type: "select", options: BATTERY_VOLTAGE_CHANGE_OPTIONS, unit: "s" },
      { key: "allLineErrTimeOut", label: "总线故障超时", type: "select", options: BUS_TIMEOUT_OPTIONS, unit: "s" },
      { key: "bluetooth", label: "蓝牙", type: "select", options: BOOLEAN_YES_NO_OPTIONS },
      { key: "serialLevel", label: "串口通讯电平", type: "select", options: SERIAL_LEVEL_OPTIONS },
      { key: "driveAssist", label: "推车助力", type: "select", options: BOOLEAN_YES_NO_OPTIONS },
      { key: "turnOnPasswd", label: "开机密码开关", type: "select", options: BOOLEAN_REVERSED_OPTIONS },
      { key: "menuPassword", label: "菜单密码开关", type: "select", options: BOOLEAN_REVERSED_OPTIONS },
    ],
  },
  {
    key: "advanced",
    title: "高级与安全",
    description: "巡航、蜂鸣器、密码和电池信息等高级配置。",
    fields: [
      { key: "factoryReset", label: "恢复出厂设置", type: "select", options: BOOLEAN_REVERSED_OPTIONS },
      { key: "cruise", label: "定速巡航", type: "select", options: BOOLEAN_REVERSED_OPTIONS },
      { key: "buzzerSwitch", label: "蜂鸣器开关", type: "select", options: BOOLEAN_REVERSED_OPTIONS },
      { key: "highSpeedBuzzerRemind", label: "高速蜂鸣器提醒", type: "number", min: 0, max: 99 },
      { key: "startupPasswd", label: "开机密码", type: "number", min: 0, max: 9999 },
      { key: "highMenuPasswd", label: "高级菜单密码", type: "number", min: 0, max: 9999 },
      { key: "motorSys", label: "电机功率", type: "number", min: 0, unit: "W" },
      { key: "batteryCap", label: "电池容量", type: "number", min: 0 },
    ],
  },
]

export const METER_REQUIRED_KEYS = METER_CONFIG_GROUPS.flatMap((group) => group.fields.map((field) => field.key))

const INI_FIELD_MAP = {
  rotateHandle: "rotate_handle",
  rotateHandleSpeedLimit: "rotate_handle_speed_limit",
  slowStart: "slow_start",
  speedSteel: "speed_detect_magnet_number",
  undervoltage: "low_voltage_threshold",
  currentlimiting: "current_limit_threshold",
  assistLimit: "assist_speed_limit_threshold",
  wheelDiameter: "wheelsize",
  batteryVoltageChangeTime: "battery_voltage_change_time",
  allLineErrTimeOut: "bus_overtime",
  smoothLevel: "smooth_level",
  unit: "display_unit",
  bluetooth: "has_ble_feature",
  backlightBrightness: "backlight_brightness",
  sleepTime: "sleep_time",
  voltage: "system_voltage",
  powerGear: "assist_level",
  assist: "assist_dir",
  assistStartMagnetNumber: "assist_start_magnet_number",
  assistPercentage: "assist_percentage",
  perimeter: "peri_meter",
  agreement: "uart_protocol",
  power: "electricity_method",
  driveAssist: "push_speed_func",
  defaultGear: "default_assist",
  logo: "logo",
  serialLevel: "uart_level",
  buzzerSwitch: "beep_switch",
  highSpeedBuzzerRemind: "limit_speed_beep",
  cruise: "cruise_enable_switch",
  startupPasswd: "power_on_password",
  highMenuPasswd: "menu_password",
  factoryReset: "res_factory_set",
  ebikeName: "ebike_name",
  motorSys: "motor_sys",
  batteryCap: "battery_cap",
  showWheelsize: "show_wheelsize",
  tiresSize: "tires_size",
  carModel: "car_model",
  turnOnPasswd: "power_password_switch",
  menuPassword: "menu_password_switch",
}

const NUMBER_LIKE_KEYS = new Set([
  "backlightBrightness",
  "currentlimiting",
  "wheelDiameter",
  "perimeter",
  "unit",
  "agreement",
  "speedSteel",
  "ebikeName",
  "batteryCap",
  "bluetooth",
  "driveAssist",
  "factoryReset",
  "rotateHandle",
  "buzzerSwitch",
  "cruise",
  "turnOnPasswd",
  "menuPassword",
  "rotateHandleSpeedLimit",
  "assist",
  "serialLevel",
  "showWheelsize",
  "motorSys",
  "defaultGear",
  "startupPasswd",
  "highMenuPasswd",
  "assistPercentage",
  "highSpeedBuzzerRemind",
  "batteryVoltageChangeTime",
  "allLineErrTimeOut",
  "slowStart",
])

export function getWheelDiameterLabel(value) {
  return WHEEL_DIAMETER_OPTIONS.find((item) => item.value === Number(value))?.label || String(value)
}

export function getDefaultUndervoltage(voltage) {
  const value = Number(voltage)
  switch (value) {
    case 24:
      return 22
    case 36:
      return 31.5
    case 48:
      return 42
    case 52:
      return 45
    case 60:
      return 54
    case 72:
      return 62
    default:
      return METER_CONFIG_DEFAULTS.undervoltage
  }
}

export function estimatePerimeterByWheel(wheelCode) {
  const wheelLabel = getWheelDiameterLabel(wheelCode)
  const diameter = wheelLabel === "700C" ? 27.5 : Number(wheelLabel)
  if (!Number.isFinite(diameter)) {
    return 0
  }
  return Math.trunc(diameter * 25.4 * 3.14)
}

export function normalizeMeterConfig(form) {
  const next = {
    ...METER_CONFIG_DEFAULTS,
    ...form,
  }

  if (next.highMenuPasswd === undefined && next.menuPasswd !== undefined) {
    next.highMenuPasswd = next.menuPasswd
  }

  next.carModel = String(next.carModel || "AA").slice(0, 2).padEnd(2, " ")
  next.undervoltage = Number(next.undervoltage)
  next.perimeter = Number(next.perimeter)
  next.showWheelsize = Number(next.showWheelsize)
  next.batteryCap = Number(next.batteryCap)
  next.motorSys = Number(next.motorSys)
  next.startupPasswd = Number(next.startupPasswd)
  next.highMenuPasswd = Number(next.highMenuPasswd)
  next.defaultGear = Number(next.defaultGear)
  next.assistPercentage = Number(next.assistPercentage)
  next.highSpeedBuzzerRemind = Number(next.highSpeedBuzzerRemind)
  next.tiresSize = Number(next.tiresSize)

  return next
}

export function encodeMeterConfig(config) {
  const value = normalizeMeterConfig(config)
  const bytes = new Array(54).fill(0)
  const now = new Date()

  bytes[0] = encodeByte(value.backlightBrightness)
  bytes[1] = encodeByte(value.sleepTime)
  bytes[2] = encodeByte(now.getFullYear() - 2000)
  bytes[3] = encodeByte(now.getMonth() + 1)
  bytes[4] = encodeByte(now.getDate())
  bytes[5] = encodeByte(now.getHours())
  bytes[6] = encodeByte(now.getMinutes())
  bytes[7] = encodeByte(value.voltage)
  bytes[8] = encodeByte(value.powerGear)
  bytes[9] = encodeByte(value.assist)
  bytes[10] = encodeByte(value.assistStartMagnetNumber)
  bytes[11] = encodeByte(value.assistPercentage)
  bytes[12] = encodeByte(value.rotateHandle)
  bytes[13] = encodeByte(value.rotateHandleSpeedLimit)
  bytes[14] = encodeByte(value.slowStart)
  bytes[15] = encodeByte(value.speedSteel)
  bytes[16] = encodeHighByte(Math.round(value.undervoltage * 1000))
  bytes[17] = encodeLowByte(Math.round(value.undervoltage * 1000))
  bytes[18] = encodeByte(value.currentlimiting)
  bytes[19] = encodeByte(value.assistLimit)
  bytes[20] = encodeByte(value.wheelDiameter)
  bytes[21] = encodeByte(value.batteryVoltageChangeTime)
  bytes[22] = encodeByte(value.allLineErrTimeOut)
  bytes[23] = encodeByte(value.smoothLevel)
  bytes[24] = encodeByte(value.unit)
  bytes[25] = encodeByte(value.bluetooth)
  bytes[26] = encodeHighByte(value.perimeter)
  bytes[27] = encodeLowByte(value.perimeter)
  bytes[28] = encodeByte(value.agreement)
  bytes[29] = encodeByte(value.power)
  bytes[30] = encodeByte(value.driveAssist)
  bytes[31] = encodeByte(value.defaultGear)
  bytes[32] = encodeByte(value.logo)
  bytes[33] = encodeByte(value.serialLevel)
  bytes[34] = encodeByte(value.buzzerSwitch)
  bytes[35] = encodeByte(value.highSpeedBuzzerRemind)
  bytes[36] = encodeByte(value.cruise)
  bytes[37] = encodeHighByte(value.startupPasswd)
  bytes[38] = encodeLowByte(value.startupPasswd)
  bytes[39] = encodeHighByte(value.highMenuPasswd)
  bytes[40] = encodeLowByte(value.highMenuPasswd)
  bytes[41] = encodeByte(value.factoryReset)
  bytes[42] = encodeByte(value.ebikeName)
  bytes[43] = encodeHighByte(value.motorSys)
  bytes[44] = encodeLowByte(value.motorSys)
  bytes[45] = encodeHighByte(value.batteryCap)
  bytes[46] = encodeLowByte(value.batteryCap)
  bytes[47] = encodeHighByte(value.showWheelsize)
  bytes[48] = encodeLowByte(value.showWheelsize)
  bytes[49] = encodeByte(value.tiresSize)
  bytes[50] = stringToAsciiCode(value.carModel, 0)
  bytes[51] = stringToAsciiCode(value.carModel, 1)
  bytes[52] = encodeByte(value.turnOnPasswd)
  bytes[53] = encodeByte(value.menuPassword)

  return bytes
}

export function decodeMeterConfig(bytes) {
  if (!Array.isArray(bytes) || bytes.length < 52) {
    throw new Error("配置字节长度不足")
  }

  return normalizeMeterConfig({
    backlightBrightness: Number(bytes[0]),
    sleepTime: Number(bytes[1]),
    voltage: Number(bytes[7]),
    powerGear: Number(bytes[8]),
    assist: Number(bytes[9]),
    assistStartMagnetNumber: Number(bytes[10]),
    assistPercentage: Number(bytes[11]),
    rotateHandle: Number(bytes[12]),
    rotateHandleSpeedLimit: Number(bytes[13]),
    slowStart: Number(bytes[14]),
    speedSteel: Number(bytes[15]),
    undervoltage: readUInt16(bytes[16], bytes[17]) / 1000,
    currentlimiting: Number(bytes[18]),
    assistLimit: Number(bytes[19]),
    wheelDiameter: Number(bytes[20]),
    batteryVoltageChangeTime: Number(bytes[21]),
    allLineErrTimeOut: Number(bytes[22]),
    smoothLevel: Number(bytes[23]),
    unit: Number(bytes[24]),
    bluetooth: Number(bytes[25]),
    perimeter: readUInt16(bytes[26], bytes[27]),
    agreement: Number(bytes[28]),
    power: Number(bytes[29]),
    driveAssist: Number(bytes[30]),
    defaultGear: Number(bytes[31]),
    logo: Number(bytes[32]),
    serialLevel: Number(bytes[33]),
    buzzerSwitch: Number(bytes[34]),
    highSpeedBuzzerRemind: Number(bytes[35]),
    cruise: Number(bytes[36]),
    startupPasswd: readUInt16(bytes[37], bytes[38]),
    highMenuPasswd: readUInt16(bytes[39], bytes[40]),
    factoryReset: Number(bytes[41]),
    ebikeName: Number(bytes[42]),
    motorSys: readUInt16(bytes[43], bytes[44]),
    batteryCap: readUInt16(bytes[45], bytes[46]),
    showWheelsize: readUInt16(bytes[47], bytes[48]),
    tiresSize: Number(bytes[49]),
    carModel: String.fromCharCode(bytes[50] || 32) + String.fromCharCode(bytes[51] || 32),
    turnOnPasswd: Number(bytes[52] ?? 1),
    menuPassword: Number(bytes[53] ?? 0),
  })
}

export function validateMeterConfig(config) {
  const data = normalizeMeterConfig(config)
  return METER_REQUIRED_KEYS.every((key) => data[key] !== "" && data[key] !== null && data[key] !== undefined)
}

function parseIniText(text) {
  const result = {}

  for (const line of text.split(/\r\n|\r|\n/)) {
    if (!line || /^\s*;/.test(line) || /^\s*\[/.test(line)) {
      continue
    }

    const match = line.match(/^\s*([\w.\-_]+)\s*=\s*(.*?)\s*$/)
    if (match) {
      result[match[1]] = match[2]
    }
  }

  return result
}

function convertIniValue(key, rawValue) {
  if (rawValue === "") {
    return rawValue
  }

  switch (key) {
    case "undervoltage":
      return (Number(rawValue) + 20000) / 1000
    case "voltage":
      return VOLTAGE_OPTIONS[Number(rawValue)]?.value ?? Number(rawValue)
    case "assistStartMagnetNumber":
      return ASSIST_START_MAGNET_OPTIONS[Number(rawValue)]?.value ?? Number(rawValue)
    case "sleepTime":
      return Number(rawValue)
    case "powerGear":
      return POWER_GEAR_OPTIONS[Number(rawValue)]?.value ?? Number(rawValue)
    case "assistLimit":
      return Number(rawValue)
    case "batteryVoltageChangeTime":
      return BATTERY_VOLTAGE_CHANGE_OPTIONS[Number(rawValue)]?.value ?? Number(rawValue)
    case "allLineErrTimeOut":
      return BUS_TIMEOUT_OPTIONS[Number(rawValue)]?.value ?? Number(rawValue)
    default:
      return NUMBER_LIKE_KEYS.has(key) ? Number(rawValue) : rawValue
  }
}

function importFromIniMap(data) {
  const mapped = {}

  for (const [key, iniKey] of Object.entries(INI_FIELD_MAP)) {
    if (!(iniKey in data)) {
      continue
    }
    mapped[key] = convertIniValue(key, data[iniKey])
  }

  return normalizeMeterConfig({
    ...METER_CONFIG_DEFAULTS,
    ...mapped,
  })
}

export async function parseMeterConfigFile(file) {
  const text = await file.text()
  const extension = file.name.split(".").pop()?.toLowerCase()

  if (extension === "ini") {
    return importFromIniMap(parseIniText(text))
  }

  const data = JSON.parse(text)
  return normalizeMeterConfig(data.instrumentModel ?? data)
}

export async function prepareMeterConfigUpgradeFile(file) {
  const config = await parseMeterConfigFile(file)
  return {
    fileName: file.name,
    config,
    data: encodeMeterConfig(config),
  }
}
