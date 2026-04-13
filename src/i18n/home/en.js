export default {
  hero: {
    mainTools: "Main Tools",
    title: "Common Tools",
    enterTool: "Open Tool",
    entryUnavailable: "The \"{title}\" entry is not available in the left menu yet. Open it there before continuing.",
  },
  info: {
    deviceEyebrow: "Device",
    deviceTitle: "Device & Config",
    deviceDescription: "After connecting, you can open the config page to read meter parameters or go to software flashing to run upgrade flows.",
    config: "Tool Config",
    software: "Software Flash",
    supportEyebrow: "Support",
    supportTitle: "User Guide",
    supportGuide: "Open Guide",
    supportItems: [
      "Select the port in the left device panel and connect first.",
      "After wiring is confirmed, open Serial Capture or CAN Capture.",
      "When you need parameter read/write or upgrades, continue on the corresponding page.",
    ],
  },
  cards: {
    wiringTitle: "Wiring Guide",
    wiringDescription: "Review hardware wiring and pin definitions before debugging.",
    serialTitle: "Serial Capture",
    serialDescription: "Inspect serial TX/RX logs and basic communication data.",
    canTitle: "CAN Capture",
    canDescription: "Capture and analyze CAN bus messages.",
  },
}
