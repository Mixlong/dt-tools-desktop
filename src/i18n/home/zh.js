export default {
  hero: {
    mainTools: "Main Tools",
    title: "常用工具",
    enterTool: "进入工具",
    entryUnavailable: "左侧菜单暂未开放“{title}”入口，请先开放后再进入。",
  },
  info: {
    deviceEyebrow: "Device",
    deviceTitle: "设备与配置",
    deviceDescription: "连接建立后，可以直接进入配置页面读取仪表参数，或进入程序烧录页面执行升级流程。",
    config: "工具配置",
    software: "程序烧录",
    supportEyebrow: "Support",
    supportTitle: "使用说明",
    supportGuide: "查看使用说明",
    supportItems: [
      "先在左侧设备面板选择端口并建立连接。",
      "接线确认后进入串口抓包或 CAN 抓包。",
      "需要参数读写或升级时，再进入对应页面继续操作。",
    ],
  },
  cards: {
    wiringTitle: "接线指引",
    wiringDescription: "查看硬件接线方式和引脚说明，适合调试前确认连接。",
    serialTitle: "串口抓包",
    serialDescription: "用于查看串口收发日志和基础通讯数据。",
    canTitle: "CAN 数据抓包",
    canDescription: "用于捕获和分析 CAN 总线报文。",
  },
}
