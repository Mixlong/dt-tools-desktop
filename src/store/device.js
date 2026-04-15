import { defineStore } from "pinia"
import { listen } from "@tauri-apps/api/event"
import { frontendLog } from "@/api/unimaster"
import { translate } from "@/i18n"
import { connectSerial, disconnectSerial, getSerialStatus, listSerialPorts } from "@/api/unimaster"
import { MODEL_OPTIONS } from "@/constants/unimaster"

const ADAPTER_BAUD_RATE = 115200
const DEFAULT_METER_UART_BAUD_CODE = 0x04
const DEFAULT_METER_CAN_BAUD_CODE = 0x08
const DEFAULT_METER_CAN_FRAME_TYPE = 0x01
const ALLOWED_USB_DEVICE_IDS = new Set([
  "1a86:7523",
])
const IGNORED_PORT_TYPES = new Set(["bluetoothport"])
const IGNORED_PORT_PATTERNS = [
  "debug-console",
  "bluetooth-incoming-port",
  "bluetooth",
  "wirelessiap",
  "airpods",
  "buds",
]
const PREFERRED_PORT_PATTERNS = [
  "usbserial",
  "usbmodem",
  "wchusbserial",
  "slab_usbtouart",
  "cp210",
  "ch340",
  "ftdi",
  "uart",
]

function getPortName(port) {
  if (typeof port === "string") {
    return port
  }

  return port?.portName || port?.port_name || ""
}

function getPortType(port) {
  if (typeof port === "string") {
    return ""
  }

  return String(port?.portType || port?.port_type || "").toLowerCase()
}

function getPortUsbId(port) {
  if (typeof port === "string") {
    return ""
  }

  const vid = Number(port?.usbVid ?? port?.usb_vid)
  const pid = Number(port?.usbPid ?? port?.usb_pid)

  if (!Number.isInteger(vid) || !Number.isInteger(pid)) {
    return ""
  }

  return `${vid.toString(16).padStart(4, "0")}:${pid.toString(16).padStart(4, "0")}`
}

function isUsableSerialPort(port) {
  const normalizedName = String(getPortName(port) || "").toLowerCase()
  const normalizedType = getPortType(port)
  const usbId = getPortUsbId(port)

  if (!normalizedName) {
    return false
  }

  if (usbId) {
    return ALLOWED_USB_DEVICE_IDS.has(usbId)
  }

  if (IGNORED_PORT_TYPES.has(normalizedType)) {
    return false
  }

  return !IGNORED_PORT_PATTERNS.some((pattern) => normalizedName.includes(pattern))
}

function getSerialPortPriority(port) {
  const normalizedName = String(getPortName(port) || "").toLowerCase()
  const normalizedType = getPortType(port)
  const usbId = getPortUsbId(port)
  const isUsbPort = normalizedType.includes("usbport")
  const hasPreferredName = PREFERRED_PORT_PATTERNS.some((pattern) => normalizedName.includes(pattern))

  if (usbId && ALLOWED_USB_DEVICE_IDS.has(usbId)) {
    return 0
  }

  if (isUsbPort && hasPreferredName) {
    return 1
  }

  if (isUsbPort) {
    return 2
  }

  if (hasPreferredName) {
    return 3
  }

  return 4
}

function logDevicePerf(message) {
  console.info(message)
  frontendLog("info", message).catch(() => {})
}

function toPortOptions(ports) {
  return ports
    .filter((item) => isUsableSerialPort(item))
    .sort((left, right) => {
      const priorityDiff = getSerialPortPriority(left) - getSerialPortPriority(right)
      if (priorityDiff !== 0) {
        return priorityDiff
      }

      return getPortName(left).localeCompare(getPortName(right))
    })
    .map((item) => {
      const portName = getPortName(item)
      return { label: portName, value: portName }
    })
}

export const useDeviceStore = defineStore("device", {
  state: () => ({
    models: MODEL_OPTIONS,
    ports: [],
    currentModel: MODEL_OPTIONS[0].value,
    transport: "uart",
    port: "",
    baudRate: ADAPTER_BAUD_RATE,
    canBitrate: 500,
    frameType: "standard",
    connectionStatus: "DISCONNECTED",
    onlineStatus: translate("device.status.disconnectedAdapter"),
    lastHeartbeatAt: "--",
    meterCommType: 0x01,
    meterBaudCode: DEFAULT_METER_UART_BAUD_CODE,
    meterFrameType: 0,
    meterUartBaudCode: DEFAULT_METER_UART_BAUD_CODE,
    meterCanBaudCode: DEFAULT_METER_CAN_BAUD_CODE,
    meterCanFrameType: DEFAULT_METER_CAN_FRAME_TYPE,
    meterLinkReady: false,
    upgradeCqCode: "",
    softwareUpgradeTargetKind: "app",
    pendingHotplugPort: "",
    upgradeInProgress: false,
  }),
  getters: {
    currentModelLabel(state) {
      return state.models.find((item) => item.value === state.currentModel)?.label || "Unknown"
    },
  },
  actions: {
    async refreshPorts() {
      const ports = await listSerialPorts()
      this.ports = toPortOptions(ports)
      const hasSelectedPort = this.ports.some((item) => item.value === this.port)

      if (!hasSelectedPort) {
        this.port = this.ports.length === 1 ? this.ports[0].value : ""
      }
      return this.ports
    },
    async syncStatus() {
      const status = await getSerialStatus()
      this.connectionStatus = status.connected ? "CONNECTED" : "DISCONNECTED"
      this.onlineStatus = status.connected
        ? translate("device.status.connectedPort", { port: status.portName })
        : translate("device.status.disconnectedAdapter")
      if (!status.connected) {
        this.meterLinkReady = false
      }
      if (status.portName) {
        this.port = status.portName
      }
      if (status.baudRate) {
        this.baudRate = status.baudRate
      }
      return status
    },
    setModel(value) {
      this.currentModel = value
    },
    setTransport(value) {
      this.transport = value
    },
    setMeterCommType(value) {
      const nextCommType = Number(value) === 0x02 ? 0x02 : 0x01

      if (this.meterCommType === 0x02) {
        this.meterCanBaudCode = Number(this.meterBaudCode ?? this.meterCanBaudCode ?? DEFAULT_METER_CAN_BAUD_CODE)
        this.meterCanFrameType = Number(this.meterFrameType ?? this.meterCanFrameType ?? DEFAULT_METER_CAN_FRAME_TYPE)
      } else {
        this.meterUartBaudCode = Number(this.meterBaudCode ?? this.meterUartBaudCode ?? DEFAULT_METER_UART_BAUD_CODE)
      }

      this.meterCommType = nextCommType
      if (nextCommType === 0x02) {
        this.meterBaudCode = Number(this.meterCanBaudCode ?? DEFAULT_METER_CAN_BAUD_CODE)
        this.meterFrameType = Number(this.meterCanFrameType ?? DEFAULT_METER_CAN_FRAME_TYPE)
      } else {
        this.meterBaudCode = Number(this.meterUartBaudCode ?? DEFAULT_METER_UART_BAUD_CODE)
        this.meterFrameType = 0
      }

      this.meterLinkReady = false
    },
    setMeterBaudCode(value) {
      const nextBaudCode = Number(value)
      this.meterBaudCode = nextBaudCode

      if (this.meterCommType === 0x02) {
        this.meterCanBaudCode = nextBaudCode
      } else {
        this.meterUartBaudCode = nextBaudCode
      }

      this.meterLinkReady = false
    },
    setMeterFrameType(value) {
      const nextFrameType = Number(value)
      this.meterFrameType = nextFrameType

      if (this.meterCommType === 0x02) {
        this.meterCanFrameType = nextFrameType
      }

      this.meterLinkReady = false
    },
    setUpgradeCqCode(value) {
      this.upgradeCqCode = String(value || "").trim().toUpperCase()
    },
    resetMeterLink() {
      this.meterLinkReady = false
    },
    setUpgradeInProgress(value) {
      this.upgradeInProgress = Boolean(value)
      if (this.upgradeInProgress) {
        this.pendingHotplugPort = ""
      }
    },
    async toggleConnection(source = "unknown") {
      if (this.upgradeInProgress) {
        throw new Error("升级进行中，暂不允许切换串口连接")
      }

      logDevicePerf(`[perf][device-store][toggle-connection-start] source=${source} status=${this.connectionStatus} port=${this.port || ""}`)

      if (this.connectionStatus === "CONNECTED") {
        const status = await disconnectSerial()
        this.connectionStatus = status.connected ? "CONNECTED" : "DISCONNECTED"
        this.onlineStatus = translate("device.status.disconnectedAdapter")
        this.meterLinkReady = false
        logDevicePerf(`[perf][device-store][toggle-connection-done] source=${source} status=${this.connectionStatus} port=${status.portName || ""}`)
        return status
      }

      const ports = await this.refreshPorts()
      if (ports.length === 0) {
        throw new Error(translate("device.errors.noPorts"))
      }

      if (!this.port) {
        throw new Error(translate("device.errors.invalidPort"))
      }

      this.baudRate = ADAPTER_BAUD_RATE
      const status = await connectSerial(this.port, ADAPTER_BAUD_RATE)
      this.connectionStatus = status.connected ? "CONNECTED" : "DISCONNECTED"
      this.onlineStatus = status.connected
        ? translate("device.status.connectedPort", { port: status.portName })
        : translate("device.status.disconnectedAdapter")
      this.lastHeartbeatAt = new Date().toLocaleTimeString()
      this.meterLinkReady = false
      logDevicePerf(`[perf][device-store][toggle-connection-done] source=${source} status=${this.connectionStatus} port=${status.portName || ""}`)
      return status
    },
    clearHotplugPending() {
      this.pendingHotplugPort = ""
    },
    queueStartupHotplugPrompt() {
      if (this.upgradeInProgress || this.connectionStatus === "CONNECTED" || this.ports.length === 0) {
        return
      }

      const targetPort = this.port || this.ports[0]?.value || ""
      if (targetPort) {
        this.pendingHotplugPort = targetPort
      }
    },
    async startPortWatcher() {
      const unlisten = await listen("ports-changed", async () => {
        const previousPort = this.port
        const previousUsablePorts = this.ports.map((item) => item.value)

        try {
          const ports = await listSerialPorts()
          const nextPortOptions = toPortOptions(ports)
          const currentUsablePorts = nextPortOptions.map((item) => item.value)
          const usableAdded = currentUsablePorts.filter((name) => !previousUsablePorts.includes(name))
          const usableRemoved = previousUsablePorts.filter((name) => !currentUsablePorts.includes(name))

          this.ports = nextPortOptions

          const hasSelectedPort = this.ports.some((item) => item.value === previousPort)
          if (!hasSelectedPort) {
            this.port = this.ports.length === 1 ? this.ports[0].value : ""
          }

          if (usableAdded.length > 0 && this.connectionStatus !== "CONNECTED" && !this.upgradeInProgress) {
            if (this.ports.length === 1) {
              this.port = this.ports[0].value
            }
            const targetPort = usableAdded[0] || this.port || this.ports[0]?.value || ""
            if (targetPort) {
              this.pendingHotplugPort = targetPort
            }
          }

          if (usableRemoved.length > 0 && this.connectionStatus === "CONNECTED") {
            const connectedPortRemoved = usableRemoved.some((name) => name === previousPort)
            if (connectedPortRemoved) {
              this.connectionStatus = "DISCONNECTED"
              this.onlineStatus = translate("device.status.adapterRemoved")
              this.meterLinkReady = false
              this.port = ""
            }
          }
        } catch (error) {
          console.error("刷新串口热插拔列表失败", error)
        }
      })

      return unlisten
    },
  },
})
