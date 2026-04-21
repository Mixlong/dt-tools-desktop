export default {
  title: "UniMaster 版本升级",
  description: "查看桌面端与设备端版本，并直接完成 APP / UI 升级。",
  refreshAll: "刷新状态",
  debug: {
    title: "调试资源入口",
    description: "默认走旧项目正式域名，也可以临时切换到本机或局域网 IP 做联调。",
    serverLabel: "资源服务地址",
    serverPlaceholder: "例如 http://127.0.0.1:8111",
    save: "保存并刷新",
    reset: "恢复默认",
    saved: "调试资源地址已保存",
    resetDone: "已恢复默认资源地址",
    defaultTag: "旧项目默认域名",
  },
  desktop: {
    name: "UniMaster PC 助手",
    current: "当前版本",
    latest: "最新版本",
  },
  device: {
    name: {
      app: "UniMaster APP",
      ui: "UniMaster UI",
    },
    current: "设备版本",
    latest: "远端版本",
    status: {
      unknown: "待检测",
      latest: "已是最新版本",
      upgrade: "可升级",
      disconnected: "未连接设备",
      checking: "检查中",
      upgrading: "升级中",
    },
    actions: {
      upgrade: "立即升级",
      retry: "重新检测",
    },
  },
  notices: {
    cqRequired: "请先在左侧设备面板填写 CQ 配置串",
    serialRequired: "请先连接串口适配器",
    remoteMissing: "未获取到远端升级资源",
  },
  logs: {
    title: "升级日志",
    empty: "等待升级日志输出...",
  },
}
