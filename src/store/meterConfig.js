import { defineStore } from "pinia"
import { translate } from "@/i18n"
import { METER_CONFIG_DEFAULTS } from "@/utils/unimaster-config"

function createDefaultForm() {
  return { ...METER_CONFIG_DEFAULTS }
}

export const useMeterConfigStore = defineStore("meterConfig", {
  state: () => ({
    form: createDefaultForm(),
    importedFileName: "",
    lastAction: translate("config.actions.none"),
    hasLoadedConfig: false,
    readStatus: "idle",
    debugLogs: [],
  }),
  actions: {
    appendDebugLogs(lines) {
      const nextLines = Array.isArray(lines) ? lines.filter(Boolean) : []
      if (!nextLines.length) {
        return
      }
      this.debugLogs.push(...nextLines.map((line) => String(line)))
      if (this.debugLogs.length > 200) {
        this.debugLogs = this.debugLogs.slice(-200)
      }
    },
    clearDebugLogs() {
      this.debugLogs = []
    },
    resetForm() {
      this.form = createDefaultForm()
      this.importedFileName = ""
      this.lastAction = translate("config.actions.none")
      this.hasLoadedConfig = false
      this.readStatus = "idle"
      this.debugLogs = []
    },
  },
})
