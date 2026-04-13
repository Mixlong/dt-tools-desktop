export default {
  eyebrow: "Support Center",
  title: "Support Center",
  description: "Keep upgrade entry points, field manuals, and troubleshooting guidance in one workspace so engineers can quickly decide what to do first, where to do it, and how to recover when something fails.",
  heroStats: [
    { value: "2", label: "core support entry points" },
    { value: "4", label: "upgrade execution steps" },
    { value: "3", label: "manual modules" },
  ],
  status: {
    eyebrow: "Workspace Status",
    title: "Field support navigation",
    note: "Check device connection and version info before starting an upgrade. For faster troubleshooting, use the pre-check list and FAQ below first.",
  },
  upgrade: {
    eyebrow: "Upgrade Center",
    title: "Upgrade workflow",
    description: "Organized around check connection, read versions, run upgrade, and verify results, with both online flow and offline package fallback.",
    primaryAction: "Open upgrade page",
    secondaryAction: "Go to version and upgrade tools",
    steps: [
      {
        title: "Confirm the connection",
        description: "Check port, transport mode, and connection state in the left device panel before starting any upgrade.",
      },
      {
        title: "Read the version snapshot",
        description: "Refresh current version details first so app, UI, and device versions can be compared before action.",
      },
      {
        title: "Run upgrade or write",
        description: "Choose realtime upgrade, file upgrade, or version writing based on the field task and keep power stable.",
      },
      {
        title: "Verify after completion",
        description: "Read versions again after the process and verify key functions, language switching, and communication recovery.",
      },
    ],
    highlights: {
      onlineTitle: "Online upgrade",
      onlineDescription: "Read versions and execute upgrades directly after device connection for routine maintenance.",
      offlineTitle: "Offline package fallback",
      offlineDescription: "Use local upgrade files when the main path fails or network access is unavailable.",
      safetyTitle: "Pre-upgrade safety",
      safetyDescription: "Confirm stable power, correct port settings, and reliable wiring before starting.",
    },
  },
  checklist: {
    eyebrow: "Pre Check",
    title: "Before upgrading",
    items: [
      "Make sure the device is powered correctly and serial or CAN communication is available.",
      "Confirm model, transport mode, and baud rate all match the field device.",
      "Record current version information before upgrading for easier comparison later.",
      "Do not switch ports, unplug cables, or close the app during the upgrade process.",
    ],
  },
  entry: {
    eyebrow: "Quick Access",
    title: "Support entry points",
    upgradeTitle: "Upgrade and versions",
    upgradeDescription: "Check version snapshots, run realtime upgrades, and handle offline upgrade packages.",
    manualTitle: "User manual",
    manualDescription: "Open operation guidance grouped by wiring, upgrade, and troubleshooting topics.",
    troubleshootTitle: "Troubleshooting",
    troubleshootDescription: "Prioritize connection errors, failed upgrades, and version mismatch issues first.",
  },
  manual: {
    eyebrow: "Manual",
    title: "User manual",
    description: "The most common field procedures are grouped into modular manuals so the team spends less time searching across documents and pages.",
    primaryAction: "View manual modules",
    sections: [
      {
        title: "Wiring and connection",
        description: "Confirm device connectivity before opening any tool workflow.",
        points: [
          "Choose the correct serial or CAN port and verify baud rate settings.",
          "Open capture, config, or upgrade flows only after wiring is complete.",
          "If no data is returned, check wiring order, power, and connection status first.",
        ],
      },
      {
        title: "Upgrade preparation",
        description: "Read the current version snapshot before deciding the next action.",
        points: [
          "Refresh version information on the software page before writing or upgrading.",
          "When using offline upgrades, make sure the selected file matches the target model.",
          "After completion, read versions again and verify key parameters.",
        ],
      },
      {
        title: "Field troubleshooting",
        description: "Classify frequent failures first, then decide whether to recover or continue.",
        points: [
          "For connection failures, check port occupation, transport mode, and cable contact first.",
          "For interrupted upgrades, keep logs, then decide between offline package fallback or re-init.",
          "If displayed versions look wrong, refresh the snapshot and verify language and flag writes.",
        ],
      },
    ],
  },
  faqEyebrow: "FAQ",
  faqTitle: "Common questions",
  faqItems: [
    {
      question: "Why is there still no device data after connection succeeds?",
      answer: "Confirm that transport mode, baud rate, and port settings match the device, then inspect wiring, power, and whether the device is ready for communication.",
    },
    {
      question: "What should I do first after an upgrade fails?",
      answer: "Do not retry repeatedly right away. Preserve logs, confirm power and connection status, then try an offline package or reinitialize the workflow.",
    },
    {
      question: "What if version information does not match the actual device state?",
      answer: "Refresh the version snapshot first, especially after a write or language switch. If it still does not match, verify flags, config files, and target model selection.",
    },
  ],
  escalation: {
    eyebrow: "Escalation",
    title: "When you need more support",
    description: "If the issue cannot be resolved on site, collect the most useful context first so development or after-sales teams can respond faster.",
    logsTitle: "Keep operation logs",
    logsDescription: "Record upgrade steps, error messages, and the last successful action for easier replay.",
    connectionTitle: "Capture connection parameters",
    connectionDescription: "Include port, transport mode, baud rate, wiring state, and power condition.",
    versionTitle: "Sync version details",
    versionDescription: "Share app version, UI version, device version, and target upgrade package information.",
    action: "Review common questions first",
  },
}
