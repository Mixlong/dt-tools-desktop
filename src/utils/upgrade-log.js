function getUpgradeTargetLabel(kind) {
  switch (String(kind || "").toLowerCase()) {
    case "boot":
      return "BOOT"
    case "ui":
      return "UI"
    case "config":
      return "CFG"
    case "app":
    default:
      return "APP"
  }
}

function isUpgradeErrorText(message) {
  const normalized = String(message || "").toLowerCase()
  return ["失败", "错误", "超时", "断开", "异常", "error", "timeout", "disconnect", "cancel"].some((keyword) => normalized.includes(keyword))
}

export function shouldUseVerboseUpgradeLogs(enabled = false) {
  return import.meta.env.DEV || Boolean(enabled)
}

export function formatCompactUpgradeErrorMessage(message, kind) {
  const targetLabel = getUpgradeTargetLabel(kind)
  const normalized = String(message || "").toLowerCase()

  if (!normalized) {
    return `${targetLabel}升级失败，请重试`
  }
  if (normalized.includes("取消")) {
    return `${targetLabel}升级已取消`
  }
  if (normalized.includes("断开")) {
    return `${targetLabel}升级失败：设备连接已断开`
  }
  if (normalized.includes("超时") || normalized.includes("timeout")) {
    return `${targetLabel}升级失败：设备响应超时`
  }
  if (normalized.includes("未确认")) {
    return `${targetLabel}升级失败：设备未确认写入`
  }
  if (normalized.includes("连接")) {
    return `${targetLabel}升级失败：请检查设备连接`
  }

  return `${targetLabel}升级失败，请重试`
}

export function formatDisplayedUpgradeProgressLog(payload, verboseEnabled = false) {
  const stage = String(payload?.stage || "").trim()
  const rawLog = String(payload?.log || "").trim()

  if (!shouldUseVerboseUpgradeLogs(verboseEnabled)) {
    const targetLabel = getUpgradeTargetLabel(payload?.kind)
    const fileProgress = Math.max(0, Math.min(100, Number(payload?.fileProgress ?? payload?.progress ?? 0)))
    const sourceText = rawLog || stage

    if (!sourceText) {
      return ""
    }
    if (isUpgradeErrorText(sourceText)) {
      return formatCompactUpgradeErrorMessage(sourceText, payload?.kind)
    }
    if (stage.includes("开始处理")) {
      return `${targetLabel}开始升级`
    }
    if (stage.includes("初始化")) {
      return `${targetLabel}初始化中`
    }
    if (stage.includes("切换实时界面")) {
      return `${targetLabel}正在切换设备界面`
    }
    if (stage.includes("等待仪表")) {
      return `${targetLabel}正在等待设备进入升级状态`
    }
    if (stage.includes("预处理")) {
      return `${targetLabel}正在准备升级环境`
    }
    if (stage.includes("擦除")) {
      return `${targetLabel}正在擦除旧数据`
    }
    if (stage.includes("分包")) {
      return `${targetLabel}升级包校验完成`
    }
    if (stage.includes("写入")) {
      return `${targetLabel}写入中（${fileProgress}%）`
    }
    if (stage.includes("完成")) {
      return `${targetLabel}升级完成`
    }

    return `${targetLabel}升级进行中（${fileProgress}%）`
  }

  return rawLog
}
