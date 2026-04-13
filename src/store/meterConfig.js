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
  }),
  actions: {
    resetForm() {
      this.form = createDefaultForm()
      this.importedFileName = ""
      this.lastAction = translate("config.actions.none")
      this.hasLoadedConfig = false
      this.readStatus = "idle"
    },
  },
})
