export default {
  eyebrow: "Support Center",
  title: "支持中心",
  description: "把升级入口、现场说明书和常见排障集中在一个工作台里，工程人员进入页面后就能快速判断先做什么、在哪里做、出了问题怎么回退。",
  heroStats: [
    { value: "2", label: "核心支持入口" },
    { value: "4", label: "升级执行步骤" },
    { value: "3", label: "说明书主题模块" },
  ],
  status: {
    eyebrow: "Workspace Status",
    title: "现场支持导航",
    note: "建议先确认设备连接与版本信息，再进入升级流程；需要快速排障时，优先查看下方 FAQ 和升级前检查。",
  },
  upgrade: {
    eyebrow: "Upgrade Center",
    title: "升级工作流",
    description: "按“检查连接、读取版本、执行升级、验证结果”的节奏组织，既适合在线升级，也保留离线升级包兜底入口。",
    primaryAction: "进入升级页面",
    secondaryAction: "打开版本与升级工具",
    steps: [
      {
        title: "确认连接状态",
        description: "先在左侧设备面板确认端口、通信方式和连接状态，避免在未连通时直接开始升级。",
      },
      {
        title: "读取版本快照",
        description: "进入升级页面先刷新当前版本信息，确认应用版本、界面版本和设备版本是否匹配。",
      },
      {
        title: "执行升级或写入",
        description: "根据现场需求选择实时升级、文件升级或版本写入，并保持供电稳定。",
      },
      {
        title: "完成后复核",
        description: "升级结束后重新读取版本信息，并检查关键功能、语言切换和通讯是否恢复正常。",
      },
    ],
    highlights: {
      onlineTitle: "在线升级",
      onlineDescription: "连接设备后直接读取版本与执行升级，适合日常维护和版本核对。",
      offlineTitle: "离线包兜底",
      offlineDescription: "升级失败或网络不可用时，使用本地升级文件继续处理，减少现场等待。",
      safetyTitle: "升级前检查",
      safetyDescription: "确认供电稳定、端口正确、线束连接可靠，避免中途中断导致重复返工。",
    },
  },
  checklist: {
    eyebrow: "Pre Check",
    title: "升级前检查",
    items: [
      "确认设备已经正确上电，串口或 CAN 连接状态正常。",
      "确认当前机型、通信方式、波特率与现场设备一致。",
      "开始升级前先记录版本信息，便于升级后做结果比对。",
      "升级过程中不要切换端口、拔插线束或关闭上位机。",
    ],
  },
  entry: {
    eyebrow: "Quick Access",
    title: "支持入口",
    upgradeTitle: "升级与版本",
    upgradeDescription: "查看版本快照、执行实时升级、处理离线升级文件。",
    manualTitle: "使用说明书",
    manualDescription: "按接线、升级、排障三个主题快速查看现场操作步骤。",
    troubleshootTitle: "排障参考",
    troubleshootDescription: "优先定位连接异常、升级失败、版本不一致等高频问题。",
  },
  manual: {
    eyebrow: "Manual",
    title: "使用说明书",
    description: "把最常用的现场操作整理成模块化说明书，减少翻找文档和跨页面确认的成本。",
    primaryAction: "查看说明书模块",
    sections: [
      {
        title: "接线与连接",
        description: "开始任何工具操作前，先完成设备连接确认。",
        points: [
          "选择正确的串口或 CAN 端口，并检查波特率设置。",
          "接线完成后再进入抓包、配置或升级功能，减少无效重试。",
          "没有返回数据时，先检查线序、电源和设备连接状态。",
        ],
      },
      {
        title: "升级准备",
        description: "升级前先读取版本快照，确认当前状态和目标操作。",
        points: [
          "进入软件页面先刷新版本信息，再决定是否写入或升级。",
          "离线升级时确认所选文件和目标设备型号一致。",
          "升级完成后建议再次读取版本并验证关键参数。",
        ],
      },
      {
        title: "现场排障",
        description: "把高频异常先分类，再决定是回退还是继续操作。",
        points: [
          "连接失败先排查端口占用、通信模式和线束接触问题。",
          "升级中断优先保留当前日志，再选择离线包或重新初始化。",
          "版本显示异常时重新刷新快照，并核对语言和标志位写入结果。",
        ],
      },
    ],
  },
  faqEyebrow: "FAQ",
  faqTitle: "常见问题",
  faqItems: [
    {
      question: "为什么连接成功后还是读不到设备数据？",
      answer: "先确认选择的通信方式、波特率和端口与设备一致，再检查接线、电源以及设备是否处于可通信状态。",
    },
    {
      question: "升级失败后应该先做什么？",
      answer: "先不要重复多次升级，优先保留日志并确认供电和连接状态，再尝试离线升级文件或重新初始化流程。",
    },
    {
      question: "版本信息和实际设备显示不一致怎么办？",
      answer: "先重新刷新版本快照，确认是否刚完成写入或切换语言；如果仍不一致，再核对标志位、配置文件和目标机型。",
    },
  ],
  escalation: {
    eyebrow: "Escalation",
    title: "需要进一步支持时",
    description: "如果现场无法直接恢复，先整理最关键的信息，再交给开发或售后，可以明显减少来回确认时间。",
    logsTitle: "保留操作日志",
    logsDescription: "记录升级步骤、报错提示和最后一次成功操作，便于复盘问题路径。",
    connectionTitle: "记录连接参数",
    connectionDescription: "保留端口、通信方式、波特率、线束状态和设备供电情况。",
    versionTitle: "同步版本信息",
    versionDescription: "附上应用版本、界面版本、设备版本和目标升级包信息。",
    action: "先查看常见问题",
  },
}
