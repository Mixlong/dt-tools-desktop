import { defineStore } from "pinia"
import { connectSerial, disconnectSerial, getSerialStatus, listSerialPorts } from "@/api/unimaster"
import { MODEL_OPTIONS } from "@/constants/unimaster"

const ADAPTER_BAUD_RATE = 115200

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
    onlineStatus: "未连接适配器",
    lastHeartbeatAt: "--",
  }),
  getters: {
    currentModelLabel(state) {
      return state.models.find((item) => item.value === state.currentModel)?.label || "Unknown"
    },
  },
  actions: {
    async refreshPorts() {
      const ports = await listSerialPorts()
      this.ports = ports.map((item) => ({ label: item.portName, value: item.portName }))
      const hasSelectedPort = this.ports.some((item) => item.value === this.port)

      if (!hasSelectedPort) {
        this.port = this.ports.length === 1 ? this.ports[0].value : ""
      }
      return this.ports
    },
    async syncStatus() {
      const status = await getSerialStatus()
      this.connectionStatus = status.connected ? "CONNECTED" : "DISCONNECTED"
      this.onlineStatus = status.connected ? `串口已连接 ${status.portName}` : "未连接适配器"
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
    async toggleConnection() {
      if (this.connectionStatus === "CONNECTED") {
        const status = await disconnectSerial()
        this.connectionStatus = status.connected ? "CONNECTED" : "DISCONNECTED"
        this.onlineStatus = "未连接适配器"
        return status
      }

      const ports = await this.refreshPorts()
      if (ports.length === 0) {
        throw new Error("未检测到串口设备，请连接适配器后刷新端口")
      }

      if (!this.port) {
        throw new Error("当前端口列表已变化，请重新选择有效端口")
      }

      this.baudRate = ADAPTER_BAUD_RATE
      const status = await connectSerial(this.port, ADAPTER_BAUD_RATE)
      this.connectionStatus = status.connected ? "CONNECTED" : "DISCONNECTED"
      this.onlineStatus = status.connected ? `串口已连接 ${status.portName}` : "未连接适配器"
      this.lastHeartbeatAt = new Date().toLocaleTimeString()
      return status
    },
  },
})
