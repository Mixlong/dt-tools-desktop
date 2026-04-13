export default {
  title: "Runtime & Updates",
  description: "Local machine settings focused on stable startup and upgrade workflows.",
  logDirectory: {
    title: "Default Log Directory",
    description: "Stores capture results, upgrade logs, and error traces.",
    select: "Choose Directory",
    desktopOnly: "The system directory picker is only available in the desktop app",
    updated: "Log directory updated",
  },
  autostart: {
    title: "Launch at Startup",
    description: "Useful for fixed workstations to reduce repeated tool launches.",
    enabled: "Launch at startup enabled",
    disabled: "Launch at startup disabled",
  },
  updates: {
    title: "Check for Updates on Launch",
    description: "Keep upgrade tools and protocol logic on the latest release.",
  },
}
