export default {
  title: "UniMaster Version Upgrade",
  description: "Check desktop and device versions, then upgrade APP / UI directly.",
  refreshAll: "Refresh Status",
  debug: {
    title: "Debug Resource Entry",
    description: "Uses the legacy production domain by default, but you can switch to a local or LAN IP for debugging.",
    serverLabel: "Resource Server URL",
    serverPlaceholder: "e.g. http://127.0.0.1:8111",
    save: "Save & Refresh",
    reset: "Reset Default",
    saved: "Debug resource URL saved",
    resetDone: "Resource URL reset to default",
    defaultTag: "Legacy default domain",
  },
  desktop: {
    name: "UniMaster PC Helper",
    current: "Current Version",
    latest: "Latest Version",
  },
  device: {
    name: {
      app: "UniMaster APP",
      ui: "UniMaster UI",
    },
    current: "Device Version",
    latest: "Remote Version",
    status: {
      unknown: "Pending",
      latest: "Up to date",
      upgrade: "Upgrade available",
      disconnected: "Device disconnected",
      checking: "Checking",
      upgrading: "Upgrading",
    },
    actions: {
      upgrade: "Upgrade Now",
      retry: "Retry",
    },
  },
  notices: {
    cqRequired: "Set the CQ configuration string in the left device panel first",
    serialRequired: "Connect the serial adapter first",
    remoteMissing: "Remote upgrade resource not found",
  },
  logs: {
    title: "Upgrade Log",
    empty: "Waiting for upgrade logs...",
  },
}
