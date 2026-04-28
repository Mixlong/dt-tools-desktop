<template>
  <div class="unimaster-about-page">
    <q-card flat bordered class="about-version-list">
      <q-card-section class="about-version-list__head">
        <div>
          <span class="about-version-list__eyebrow">UniMaster</span>
          <h2>固件版本</h2>
          <p>查看设备端 APP / UI 固件版本，并一键升级到最新。</p>
        </div>
      </q-card-section>
      <q-separator />
      <q-card-section class="about-version-list__section">
        <div class="about-version-table">
          <div class="about-version-table__head">
            <span>组件</span>
            <span>{{ t("about.device.current") }}</span>
            <span>{{ t("about.device.latest") }}</span>
            <span>状态</span>
            <span>操作</span>
          </div>

          <div class="about-version-row">
            <div class="about-version-cell about-version-cell--name">
              <div class="about-card__badge">
                <q-icon name="desktop_windows" size="16px" />
                <span>PC</span>
              </div>
            </div>
            <div class="about-version-cell">
              <strong>{{ desktopSummary.currentLabel }}</strong>
            </div>
            <div class="about-version-cell">
              <strong>{{ desktopSummary.latestLabel }}</strong>
            </div>
            <div class="about-version-cell">
              <div class="about-status-pill" :class="`about-status-pill--${desktopSummary.isLatest ? 'primary' : 'positive'}`">
                {{ desktopSummary.isLatest ? t("about.device.status.latest") : t("about.device.status.upgrade") }}
              </div>
            </div>
            <div class="about-version-cell about-version-cell--actions">
              <q-btn
                class="about-action-btn"
                flat
                color="grey-7"
                icon="refresh"
                no-caps
                :loading="refreshing"
                :label="t('about.refreshAll')"
                @click="refreshPageState"
              />
            </div>
          </div>

          <div
            v-for="item in versionItems"
            :key="item.kind"
            class="about-version-row"
          >
            <div class="about-version-cell about-version-cell--name">
              <div class="about-card__badge" :class="`about-card__badge--${item.kind}`">
                <q-icon :name="item.kind === 'app' ? 'memory' : 'dashboard_customize'" size="16px" />
                <span>{{ item.kind.toUpperCase() }}</span>
              </div>
            </div>
            <div class="about-version-cell">
              <strong>{{ item.currentLabel }}</strong>
            </div>
            <div class="about-version-cell">
              <strong>{{ item.latestLabel }}</strong>
            </div>
            <div class="about-version-cell">
              <div class="about-status-pill" :class="`about-status-pill--${item.statusColor}`">
                {{ item.statusText }}
              </div>
            </div>
            <div class="about-version-cell about-version-cell--actions">
              <div class="about-action-stack">
                <q-btn
                  class="about-action-btn"
                  color="primary"
                  icon="system_update_alt"
                  no-caps
                  unelevated
                  :label="t('about.device.actions.upgrade')"
                  :loading="item.upgrading"
                  :disable="!item.canUpgrade"
                  @click="upgradeDeviceKind(item.kind)"
                />
                <div v-if="item.progressVisible" class="about-inline-progress">
                  <q-linear-progress
                    rounded
                    size="6px"
                    :value="item.progress / 100"
                    color="primary"
                    track-color="grey-3"
                  />
                  <span>{{ item.progress }}%</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </q-card-section>
    </q-card>

    <q-card v-if="deviceStore.developerModeEnabled" flat class="panel about-debug-card">
      <q-card-section class="about-debug-card__section">
        <div class="about-card__head">
          <div>
            <div class="about-card__title">{{ t("about.debug.title") }}</div>
            <div class="about-card__subtitle">{{ t("about.debug.description") }}</div>
          </div>
          <div class="about-card__badge">
            {{ t("about.debug.defaultTag") }}
          </div>
        </div>

        <div class="about-debug-card__form">
          <q-input
            v-model="resourceBaseUrlInput"
            outlined
            dense
            :placeholder="t('about.debug.serverPlaceholder')"
            :label="t('about.debug.serverLabel')"
          />
          <div class="about-debug-card__actions">
            <q-btn
              class="about-action-btn"
              color="primary"
              icon="save"
              no-caps
              unelevated
              :label="t('about.debug.save')"
              @click="saveResourceBaseUrl"
            />
            <q-btn
              class="about-action-btn"
              outline
              color="grey-7"
              icon="settings_backup_restore"
              no-caps
              :label="t('about.debug.reset')"
              @click="resetResourceBaseUrl"
            />
          </div>
        </div>
      </q-card-section>
    </q-card>

    <q-card v-if="deviceStore.developerModeEnabled" flat class="panel about-log-card">
      <q-card-section>
        <div class="about-log-card__head">
          <div>
            <div class="about-card__title">{{ t("about.logs.title") }}</div>
            <div class="about-card__subtitle">{{ currentStage || t("about.logs.empty") }}</div>
          </div>
          <q-btn
            flat
            dense
            icon="delete_sweep"
            color="grey-6"
            no-caps
            :disable="!upgradeLogs.length"
            @click="clearLogs"
          />
        </div>

        <div class="about-log-card__body">
          <template v-if="upgradeLogs.length">
            <div
              v-for="(line, index) in upgradeLogs"
              :key="`${index}-${line}`"
              class="about-log-card__line"
            >
              <span class="about-log-card__index">{{ String(index + 1).padStart(2, "0") }}</span>
              <span>{{ line }}</span>
            </div>
          </template>
          <div v-else class="about-log-card__empty">
            {{ t("about.logs.empty") }}
          </div>
        </div>
      </q-card-section>
    </q-card>
  </div>
</template>

<script setup>
import { listen } from "@tauri-apps/api/event"
import { computed, onBeforeUnmount, onMounted, reactive, ref, watch } from "vue"
import { useI18n } from "vue-i18n"
import tauriConfig from "../../src-tauri/tauri.conf.json"
import { useDeviceStore } from "@/store/device"
import {
  downloadUpgradeFileBytes,
  getLegacyUniMasterBaseUrl,
  performUniMasterVersionUpgrade,
  queryUpgradeResource,
  readUniMasterVersionInfo,
  writeVersionInfo,
} from "@/api/unimaster"
import { requestRemoteVersionRefresh } from "@/services/deviceVersion"
import { notifyError, notifySuccess } from "@/services/ui"
import { getRemoteAppVersion } from "@/updater"
import { loadPreferences, savePreferences } from "@/utils/preferences"
import { buildUniMasterVersionChunks } from "@/utils/unimaster-version-upgrade"
import { compareVersions, normalizeVersion } from "@/utils/version"

const { t } = useI18n()
const deviceStore = useDeviceStore()

const currentAppVersion = String(tauriConfig?.version || "0.1.0")
const defaultResourceBaseUrl = getLegacyUniMasterBaseUrl()
const resourceBaseUrlInput = ref(String(loadPreferences().unimasterAboutBaseUrl || defaultResourceBaseUrl))
const refreshing = ref(false)
const currentStage = ref("")
const upgradeLogs = ref([])
const resourceErrorMessage = ref("")
const versionInfo = reactive({
  appVersion: "",
  uiVersion: "",
})
const desktop = reactive({
  current: currentAppVersion,
  latest: currentAppVersion,
})
const remoteResources = reactive({
  app: createRemoteResource(),
  ui: createRemoteResource(),
})
const progressState = reactive({
  app: createProgressState(),
  ui: createProgressState(),
})
let unlistenUpgradeProgress = null
const VERSION_TYPE_CODE_MAP = {
  app: 4,
  ui: 8,
}
const VERSION_VERIFY_RETRY_DELAYS = [1600, 2200, 3000]

function createRemoteResource() {
  return {
    versionName: "",
    fileUrl: "",
    fileName: "",
    explains: "",
  }
}

function createProgressState() {
  return {
    progress: 0,
    upgrading: false,
  }
}

const versionItems = computed(() => {
  return ["app", "ui"].map((kind) => {
    const currentVersion = normalizeVersion(kind === "app" ? versionInfo.appVersion : versionInfo.uiVersion)
    const latestVersion = normalizeVersion(remoteResources[kind].versionName || "")
    const disconnected = deviceStore.connectionStatus !== "CONNECTED"
    const upgrading = progressState[kind].upgrading
    const remoteMissing = !remoteResources[kind].fileUrl
    const hasComparableVersions = Boolean(currentVersion && latestVersion)
    const remoteHigherThanCurrent = hasComparableVersions
      ? compareVersions(currentVersion, latestVersion) < 0
      : false
    const canUpgrade = !disconnected
      && !upgrading
      && !remoteMissing
      && (
        !latestVersion
        || !currentVersion
        || remoteHigherThanCurrent
      )

    let statusText = t("about.device.status.unknown")
    let statusColor = "grey-6"

    if (upgrading) {
      statusText = t("about.device.status.upgrading")
      statusColor = "warning"
    } else if (disconnected) {
      statusText = t("about.device.status.disconnected")
      statusColor = "grey-7"
    } else if (latestVersion && !currentVersion) {
      statusText = t("about.device.status.upgrade")
      statusColor = "positive"
    } else if (!currentVersion || refreshing.value) {
      statusText = t("about.device.status.checking")
      statusColor = "info"
    } else if (remoteHigherThanCurrent) {
      statusText = t("about.device.status.upgrade")
      statusColor = "positive"
    } else if (currentVersion) {
      statusText = t("about.device.status.latest")
      statusColor = "primary"
    }

    return {
      kind,
      label: t(`about.device.name.${kind}`),
      currentLabel: currentVersion ? `v${currentVersion}` : "---",
      latestLabel: latestVersion ? `v${latestVersion}` : "---",
      statusText,
      statusColor,
      progress: progressState[kind].progress,
      progressVisible: upgrading || progressState[kind].progress > 0,
      upgrading,
      canUpgrade,
      resource: remoteResources[kind],
    }
  })
})

watch(
  () => [deviceStore.connectionStatus, deviceStore.port],
  ([status, port], previous) => {
    const [previousStatus, previousPort] = previous || []
    if (status !== "CONNECTED") {
      versionInfo.appVersion = ""
      versionInfo.uiVersion = ""
      return
    }
    if (status === previousStatus && port === previousPort) {
      return
    }
    appendLogs([`[version] 检测到设备连接 (port=${port || "-"})，准备读取版本号`])
    setTimeout(() => {
      if (deviceStore.connectionStatus === "CONNECTED") {
        refreshDeviceSnapshot()
      }
    }, 300)
  },
)

onMounted(async () => {
  unlistenUpgradeProgress = await listen("upgrade-progress", (event) => {
    applyUniMasterUpgradeProgress(event.payload)
  })
  await refreshPageState()
})

onBeforeUnmount(() => {
  unlistenUpgradeProgress?.()
  unlistenUpgradeProgress = null
})

function clearLogs() {
  upgradeLogs.value = []
}

function getResourceBaseUrl() {
  return String(resourceBaseUrlInput.value || defaultResourceBaseUrl).trim() || defaultResourceBaseUrl
}

function saveResourceBaseUrl() {
  const nextBaseUrl = getResourceBaseUrl()
  resourceBaseUrlInput.value = nextBaseUrl
  savePreferences({
    ...loadPreferences(),
    unimasterAboutBaseUrl: nextBaseUrl,
  })
  notifySuccess(t("about.debug.saved"))
  refreshRemoteResources().catch((error) => notifyError(error))
}

function resetResourceBaseUrl() {
  resourceBaseUrlInput.value = defaultResourceBaseUrl
  savePreferences({
    ...loadPreferences(),
    unimasterAboutBaseUrl: defaultResourceBaseUrl,
  })
  notifySuccess(t("about.debug.resetDone"))
  refreshRemoteResources().catch((error) => notifyError(error))
}

function appendLogs(lines) {
  lines
    .map((line) => String(line || "").trim())
    .filter(Boolean)
    .forEach((line) => {
      if (upgradeLogs.value[upgradeLogs.value.length - 1] !== line) {
        upgradeLogs.value.push(line)
      }
    })
}

function applyUniMasterUpgradeProgress(payload) {
  const kind = String(payload?.kind || "").toLowerCase()
  if (!kind || !progressState[kind]?.upgrading) {
    return
  }

  const nextProgress = Math.max(0, Math.min(100, Number(payload?.fileProgress ?? payload?.progress ?? 0)))
  progressState[kind].progress = Math.max(progressState[kind].progress, nextProgress)

  const stage = String(payload?.stage || "").trim()
  if (stage) {
    currentStage.value = stage
  }

  const logLine = String(payload?.log || "").trim()
  if (logLine) {
    appendLogs([logLine])
  }
}

function formatResourceErrorMessage(error) {
  return String(error?.userMessage || "远程升级资源获取失败")
}

function formatResourceDebugMessage(error) {
  const backendMessage = String(error?.backendMessage || "").trim()
  if (!backendMessage) {
    return formatResourceErrorMessage(error)
  }

  return `${formatResourceErrorMessage(error)}：${backendMessage}`
}

async function refreshPageState() {
  refreshing.value = true
  resourceErrorMessage.value = ""
  try {
    const results = await Promise.allSettled([
      refreshDesktopVersion(),
      refreshRemoteResources(),
      refreshDeviceSnapshot(),
    ])
    const firstRejected = results.find((item) => item.status === "rejected")
    if (firstRejected?.reason) {
      notifyError(firstRejected.reason)
    }
  } finally {
    refreshing.value = false
  }
}

async function refreshDesktopVersion() {
  try {
    const latest = String(await getRemoteAppVersion() || currentAppVersion).trim()
    desktop.current = currentAppVersion
    desktop.latest = latest || currentAppVersion
  } catch (error) {
    desktop.current = currentAppVersion
    desktop.latest = currentAppVersion
    appendLogs([String(error?.message || error || "")])
  }
}

async function refreshRemoteResources() {
  const [appResult, uiResult] = await Promise.allSettled([
    queryUpgradeResource("UniMaster_Upgrade_APP", { baseUrl: getResourceBaseUrl() }),
    queryUpgradeResource("UniMaster_Upgrade_UI2", { baseUrl: getResourceBaseUrl() }),
  ])

  const appResource = appResult.status === "fulfilled" ? appResult.value : null
  const uiResource = uiResult.status === "fulfilled" ? uiResult.value : null

  Object.assign(remoteResources.app, {
    versionName: String(appResource?.versionName || ""),
    fileUrl: String(appResource?.fileUrl || ""),
    fileName: extractFileName(appResource?.fileUrl),
    explains: String(appResource?.explains || ""),
  })
  Object.assign(remoteResources.ui, {
    versionName: String(uiResource?.versionName || ""),
    fileUrl: String(uiResource?.fileUrl || ""),
    fileName: extractFileName(uiResource?.fileUrl),
    explains: String(uiResource?.explains || ""),
  })

  const errors = [appResult, uiResult]
    .filter((item) => item.status === "rejected")
    .map((item) => item.reason)
    .filter(Boolean)

  if (errors.length) {
    resourceErrorMessage.value = formatResourceErrorMessage(errors[0])
    appendLogs(errors.map((error) => formatResourceDebugMessage(error)))
  }
}

async function refreshDeviceSnapshot() {
  if (deviceStore.connectionStatus !== "CONNECTED") {
    versionInfo.appVersion = ""
    versionInfo.uiVersion = ""
    return {
      appVersion: "",
      uiVersion: "",
      logs: [],
    }
  }

  try {
    const info = await readUniMasterVersionInfo()
    versionInfo.appVersion = String(info?.appVersion || "")
    versionInfo.uiVersion = String(info?.uiVersion || "")
    if (Array.isArray(info?.logs)) {
      appendLogs(info.logs)
    }
    return info
  } catch (error) {
    versionInfo.appVersion = ""
    versionInfo.uiVersion = ""
    appendLogs([`设备版本读取失败：${String(error?.message || error || "")}`])
    notifyError(error)
    throw error
  }
}

function extractFileName(url) {
  const normalized = String(url || "").split("?")[0]
  return normalized.split("/").pop() || ""
}

function waitForDelay(delayMs) {
  return new Promise((resolve) => {
    window.setTimeout(resolve, delayMs)
  })
}

function getVersionValueByKind(snapshot, kind) {
  return String(kind === "app" ? snapshot?.appVersion || "" : snapshot?.uiVersion || "").trim()
}

function buildVersionSyncText(kind, currentText, targetVersion, explicitText = "") {
  const explicit = String(explicitText || "").trim()
  if (explicit) {
    return explicit
  }

  const normalizedTargetVersion = normalizeVersion(targetVersion)
  if (!normalizedTargetVersion) {
    return ""
  }

  const current = String(currentText || "").trim()
  if (current) {
    const replaced = current.replace(/([vV]?)(\d+(?:\.\d+)+)$/i, (_, versionPrefix) => {
      if (versionPrefix === "V") {
        return `V${normalizedTargetVersion}`
      }
      if (versionPrefix === "v") {
        return `v${normalizedTargetVersion}`
      }
      return normalizedTargetVersion
    })
    if (replaced !== current && normalizeVersion(replaced) === normalizedTargetVersion) {
      return replaced
    }
  }

  return `${kind.toUpperCase()}_V${normalizedTargetVersion}`
}

async function verifyVersionKind(kind, targetVersion, phaseLabel) {
  const normalizedTargetVersion = normalizeVersion(targetVersion)
  let snapshot = await refreshDeviceSnapshot()
  let currentValue = getVersionValueByKind(snapshot, kind)

  if (!normalizedTargetVersion || normalizeVersion(currentValue) === normalizedTargetVersion) {
    return {
      snapshot,
      currentValue,
      matched: true,
      matchedBy: phaseLabel,
    }
  }

  for (const [index, delayMs] of VERSION_VERIFY_RETRY_DELAYS.entries()) {
    appendLogs([
      `[upgrade][${kind}][verify] ${phaseLabel}后等待设备生效（第 ${index + 2} 次校验，${delayMs}ms）`,
    ])
    await waitForDelay(delayMs)
    snapshot = await refreshDeviceSnapshot()
    currentValue = getVersionValueByKind(snapshot, kind)
    if (normalizeVersion(currentValue) === normalizedTargetVersion) {
      return {
        snapshot,
        currentValue,
        matched: true,
        matchedBy: phaseLabel,
      }
    }
  }

  return {
    snapshot,
    currentValue,
    matched: false,
    matchedBy: phaseLabel,
  }
}

async function syncVersionType(kind, targetText) {
  const versionTypeCode = VERSION_TYPE_CODE_MAP[kind]
  if (!versionTypeCode || !targetText) {
    return false
  }

  appendLogs([
    `[upgrade][${kind}][version-sync] 发送 0xB0 版本同步：type=${versionTypeCode} value=${targetText}`,
  ])
  const result = await writeVersionInfo({
    code: versionTypeCode,
    valueText: targetText,
  })
  appendLogs([
    `[upgrade][${kind}][version-sync] ${result?.message || "版本同步完成"}`,
  ])

  if (!result?.success) {
    throw new Error(`${kind.toUpperCase()} 版本同步失败`)
  }

  return true
}

async function finalizeVersionManagementUpgrade(kind, resource, previousVersionText) {
  const targetVersion = normalizeVersion(resource.versionName || "")
  const expectedVersionText = buildVersionSyncText(
    kind,
    previousVersionText,
    targetVersion,
    kind === "ui" ? String(resource.explains || "").trim() : "",
  )

  if (!targetVersion && !expectedVersionText) {
    appendLogs([`[upgrade][${kind}][verify] 未提供目标版本信息，跳过最终校验`])
    return
  }

  const initialVerification = await verifyVersionKind(kind, targetVersion, "升级完成")
  if (initialVerification.matched) {
    appendLogs([
      `[upgrade][${kind}][verify] 版本校验通过：${initialVerification.currentValue || expectedVersionText || targetVersion}`,
    ])
    return
  }

  if (!expectedVersionText) {
    throw new Error(
      `${kind.toUpperCase()} 刷写完成，但未能读回目标版本 ${targetVersion}，且缺少可写入的版本文本`,
    )
  }

  await syncVersionType(kind, expectedVersionText)

  const syncedVerification = await verifyVersionKind(kind, targetVersion, "版本同步")
  if (syncedVerification.matched) {
    appendLogs([
      `[upgrade][${kind}][verify] 版本同步后校验通过：${syncedVerification.currentValue || expectedVersionText}`,
    ])
    return
  }

  throw new Error(
    `${kind.toUpperCase()} 刷写完成，但版本仍为 ${syncedVerification.currentValue || "---"}，目标版本 ${targetVersion || expectedVersionText}`,
  )
}

async function upgradeDeviceKind(kind) {
  if (deviceStore.connectionStatus !== "CONNECTED") {
    notifyError(t("about.notices.serialRequired"))
    return
  }

  const resource = remoteResources[kind]
  if (!resource.fileUrl) {
    notifyError(t("about.notices.remoteMissing"))
    return
  }

  progressState[kind].upgrading = true
  progressState[kind].progress = 0
  const previousVersionText = String(kind === "app" ? versionInfo.appVersion : versionInfo.uiVersion || "").trim()
  currentStage.value = `${t(`about.device.name.${kind}`)} 正在下载升级包`
  appendLogs([
    `${t(`about.device.name.${kind}`)} 开始升级`,
    `[upgrade][${kind}] 开始下载升级文件：${resource.fileName || resource.fileUrl}`,
  ])

  try {
    const bytes = await downloadUpgradeFileBytes(resource.fileUrl, { baseUrl: getResourceBaseUrl() })
    const chunks = buildUniMasterVersionChunks(kind, resource.fileName || `${kind}.bin`, bytes)
    currentStage.value = `${t(`about.device.name.${kind}`)} ${t("about.device.status.upgrading")}`
    appendLogs([
      `[upgrade][${kind}] 升级文件下载完成：${bytes.length} bytes / ${chunks.length} 包`,
    ])
    const result = await performUniMasterVersionUpgrade({
      kind,
      fileName: resource.fileName || `${kind}.bin`,
      data: bytes,
      chunks,
      uiVersion: kind === "ui" ? String(resource.explains || resource.versionName || "").trim() : "",
    })

    appendLogs(result.logs || [])
    currentStage.value = result.stage || currentStage.value
    const txLogMatches = (result.logs || [])
      .map((line) => line.match(/\[UniMaster\]\[A2\]\[tx\]\[(\d+)\/(\d+)\]/))
      .filter(Boolean)
    if (txLogMatches.length) {
      const [, current, total] = txLogMatches[txLogMatches.length - 1]
      progressState[kind].progress = Math.min(
        99,
        Math.floor((Number(current) / Number(total || 1)) * 100),
      )
    }

    if (!result.success) {
      throw new Error(result.stage || `${t(`about.device.name.${kind}`)} 升级失败`)
    }

    progressState[kind].progress = 99
    currentStage.value = `${t(`about.device.name.${kind}`)} 正在校验版本`
    await finalizeVersionManagementUpgrade(kind, resource, previousVersionText)
    progressState[kind].progress = 100
    currentStage.value = `${t(`about.device.name.${kind}`)} 升级完成`
    notifySuccess(`${t(`about.device.name.${kind}`)} 升级并校验完成`)
    requestRemoteVersionRefresh({
      silent: true,
      retries: 4,
      retryDelayMs: 600,
    })
  } catch (error) {
    currentStage.value = String(error?.message || error || "")
    appendLogs([
      `[upgrade][${kind}][error] ${currentStage.value}`,
      currentStage.value,
    ])
    notifyError(error)
  } finally {
    progressState[kind].upgrading = false
  }
}

const desktopSummary = computed(() => {
  const current = normalizeVersion(desktop.current)
  const latest = normalizeVersion(desktop.latest)
  return {
    currentLabel: current ? `v${current}` : "---",
    latestLabel: latest ? `v${latest}` : "---",
    isLatest: compareVersions(current, latest) >= 0,
  }
})
</script>

<style scoped lang="scss">
.unimaster-about-page {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.about-hero {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--dt-space-4);
  padding: 16px 18px;
}

.about-hero__copy {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.about-hero h1 {
  margin: 0;
  font-size: 18px;
  line-height: 1.35;
  font-weight: 700;
  color: var(--dt-text-primary);
}

.about-hero p {
  margin: 0;
  max-width: 640px;
  color: var(--dt-text-secondary);
  line-height: 1.7;
}

.about-debug-card__section {
  display: grid;
  gap: 12px;
  padding: 16px 18px;
}

.about-debug-card__form {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 12px;
  align-items: end;
}

.about-debug-card__actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.about-debug-card__actions :deep(.q-btn) {
  font-weight: 700;
}

.about-version-list {
  order: 1;
  background: var(--dt-gloss-surface);
  border: 1px solid var(--dt-border);
  box-shadow: var(--dt-shadow-float);
}

.about-version-list__head {
  padding: 18px 20px 16px;
}

.about-version-list__head h2 {
  margin: 0 0 6px;
  font-size: 20px;
  font-weight: 800;
  line-height: 1.2;
  color: var(--dt-text-primary);
}

.about-version-list__head p {
  margin: 0;
  color: var(--dt-text-secondary);
  font-size: 14px;
  line-height: 1.6;
}

.about-version-list__eyebrow {
  display: inline-block;
  margin-bottom: 10px;
  color: var(--dt-text-muted);
  font-size: 11px;
  font-weight: 800;
  letter-spacing: 0.18em;
  text-transform: uppercase;
}

.about-version-list__section {
  display: grid;
  gap: 12px;
  padding: 18px 20px 20px;
}

.about-version-list :deep(.q-separator) {
  background: var(--dt-border);
}

.about-debug-card {
  order: 2;
}

.about-log-card {
  order: 3;
}

.about-resource-warning {
  padding: 12px 14px;
  border-radius: var(--dt-radius-subtle);
  background: rgba(255, 102, 102, 0.1);
  color: #cf3340;
  font-size: 13px;
  font-weight: 700;
}

.about-version-table {
  display: grid;
  gap: 8px;
}

.about-version-table__head,
.about-version-row {
  display: grid;
  grid-template-columns: minmax(240px, 2fr) minmax(120px, 1fr) minmax(120px, 1fr) minmax(150px, 1.1fr) minmax(140px, 1fr);
  gap: 12px;
  align-items: center;
}

.about-version-table__head {
  padding: 0 6px;
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 700;
}

.about-version-row {
  padding: 10px 12px;
  border-radius: var(--dt-radius-subtle);
  background: var(--dt-bg-panel-soft);
  border: 1px solid rgba(94, 135, 199, 0.14);
}

.about-version-cell {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
  color: var(--dt-text-primary);
}

.about-version-cell strong {
  font-size: 14px;
  font-weight: 700;
}

.about-version-cell--name {
  gap: 12px;
}

.about-version-cell--actions {
  justify-content: flex-end;
}

.about-action-stack {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 6px;
  min-width: 132px;
}

.about-inline-progress {
  width: 132px;
  display: flex;
  align-items: center;
  gap: 8px;
}

.about-inline-progress span {
  min-width: 32px;
  text-align: right;
  font-size: 11px;
  font-weight: 700;
  color: var(--dt-text-secondary);
}

.about-card__section {
  display: grid;
  gap: var(--dt-space-4);
  padding: 20px;
}

.about-card__head,
.about-log-card__head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.about-card__title {
  font-size: 16px;
  font-weight: 700;
  color: var(--dt-text-primary);
}

.about-card__subtitle {
  margin-top: 4px;
  font-size: 12px;
  color: var(--dt-text-secondary);
}

.about-card__badge {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 10px;
  border-radius: var(--dt-radius-button);
  background: var(--dt-bg-panel-soft);
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 700;
}

.about-card__badge--app {
  color: var(--dt-brand-secondary);
}

.about-card__badge--ui {
  color: var(--dt-accent);
}

.about-card__versions {
  display: grid;
  gap: 10px;
}

.about-version {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px 14px;
  border-radius: var(--dt-radius-subtle);
  background: var(--dt-bg-panel-soft);
}

.about-version span {
  font-size: 12px;
  color: var(--dt-text-secondary);
}

.about-version strong {
  font-size: 15px;
  font-weight: 700;
  color: var(--dt-text-primary);
}

.about-card__status {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;
  min-height: 28px;
}

.about-card__file {
  font-size: 12px;
  color: var(--dt-text-secondary);
  text-align: right;
  word-break: break-all;
}

.about-status-pill,
.about-summary-strip {
  display: inline-flex;
  align-items: center;
  min-height: 34px;
  padding: 0 10px;
  border-radius: var(--dt-radius-button);
  background: var(--dt-bg-panel-soft);
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 600;
}

.about-status-pill--primary {
  background: color-mix(in srgb, var(--dt-brand-primary-soft) 70%, var(--dt-bg-panel-soft));
  color: var(--dt-accent);
}

.about-status-pill--positive {
  background: color-mix(in srgb, var(--dt-status-success-soft) 74%, var(--dt-bg-panel-soft));
  color: var(--dt-success);
}

.about-status-pill--warning {
  background: color-mix(in srgb, var(--dt-status-warning-soft) 74%, var(--dt-bg-panel-soft));
  color: var(--dt-warning);
}

.about-status-pill--info {
  background: color-mix(in srgb, var(--dt-brand-primary-soft) 64%, var(--dt-bg-panel-soft));
  color: var(--dt-info);
}

.about-status-pill--grey-6,
.about-status-pill--grey-7 {
  background: var(--dt-bg-panel-soft);
  color: var(--dt-text-secondary);
}

.about-card__progress {
  margin-top: -2px;
}

.about-card__actions {
  display: flex;
  gap: 10px;
}

.about-action-btn {
  min-height: 34px;
  border-radius: var(--dt-radius-button);
  font-size: 13px;
  font-weight: 700;
  padding: 0 14px;
}

.about-card__actions :deep(.q-btn) {
  flex: 1 1 0;
  font-weight: 700;
}

.about-log-card__body {
  margin-top: 10px;
  max-height: 220px;
  overflow: auto;
  padding: 10px 12px;
  border-radius: var(--dt-radius-subtle);
  background: var(--dt-bg-panel-soft);
  color: var(--dt-text-primary);
  font-family: "SFMono-Regular", "Consolas", monospace;
  font-size: 12px;
  line-height: 1.6;
}

.about-log-card__line {
  display: flex;
  gap: 10px;
  align-items: flex-start;
}

.about-log-card__index {
  flex: 0 0 22px;
  color: var(--dt-text-muted);
}

.about-log-card__empty {
  color: var(--dt-text-secondary);
}

@media (max-width: 1200px) {
  .about-debug-card__form {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 720px) {
  .about-hero {
    flex-direction: column;
    align-items: stretch;
  }

  .about-debug-card__form {
    grid-template-columns: 1fr;
  }

  .about-version-table__head {
    display: none;
  }

  .about-version-row {
    grid-template-columns: 1fr;
  }

  .about-version-cell {
    justify-content: space-between;
  }

  .about-version-cell--name,
  .about-version-cell--actions {
    justify-content: flex-start;
  }

  .about-action-stack {
    align-items: flex-start;
  }

  .about-debug-card__actions {
    flex-direction: column;
  }
}
</style>
