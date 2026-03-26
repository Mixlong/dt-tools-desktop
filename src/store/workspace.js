import { defineStore } from "pinia"

export const useWorkspaceStore = defineStore("workspace", {
  state: () => ({
    tools: [
      { id: "serial", name: "串口抓包", route: "/tools/serial" },
      { id: "can", name: "CAN 抓包", route: "/tools/can" },
      { id: "wiring", name: "接线指引", route: "/tools/wiring" },
    ],
  }),
})
