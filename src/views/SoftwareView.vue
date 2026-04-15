<template>
  <div class="program-burning-page program-burning-page--local">
    <section class="workspace-panel workspace-panel--local">
      <section class="workspace-panel__content workspace-panel__content--local">
        <div class="file-grid file-grid--local">
          <div
            v-for="item in localFileMetas.filter((item) => localKindOrder.includes(item.kind))"
            :key="`local-${item.kind}`"
            class="file-card file-card--local"
            :class="{ 'file-card--active': localFiles[item.kind].isActive }"
          >
            <div class="file-card__head">
              <div class="file-card__head-main">
                <div class="file-card__labels">
                  <span class="file-card__tag">{{ item.label }}</span>
                  <span class="file-card__format">{{ item.tip }}</span>
                  <span :class="['file-state-chip', { 'file-state-chip--active': localFiles[item.kind].isActive }]">
                    {{ getFileStateLabel(localFiles[item.kind]) }}
                  </span>
                </div>
              </div>
            </div>
            <div class="file-card__dropzone">
              <div class="file-card__dropzone-icon">
                <q-icon :name="localFiles[item.kind].fileName ? 'task_alt' : 'upload_file'" size="22px" />
              </div>
              <div class="file-card__dropzone-copy">
                <strong>{{ localFiles[item.kind].fileName || t("software.local.fileNotImported", { label: item.label }) }}</strong>
                <p>{{ localFiles[item.kind].fileName ? t("software.local.fileReady") : t("software.local.fileSupport", { tip: item.tip }) }}</p>
              </div>
            </div>
            <div class="file-card__progress">
              <q-linear-progress
                rounded
                size="8px"
                :value="localFiles[item.kind].percent / 100"
                color="primary"
              />
              <span>{{ localFiles[item.kind].percent }}%</span>
            </div>
            <div class="file-terminal">
              <div class="file-terminal__head">
                <strong>{{ t("software.local.terminalTitle", { label: item.label }) }}</strong>
                <q-btn
                  flat
                  dense
                  size="12px"
                  color="grey-5"
                  icon="delete_sweep"
                  :disable="!terminalLogs[item.kind]?.length"
                  @click="clearLocalLogs(item.kind)"
                />
              </div>
              <div
                :ref="(element) => setTerminalBodyRef(item.kind, element)"
                class="file-terminal__body"
              >
                <template v-if="terminalLogs[item.kind]?.length">
                  <q-virtual-scroll
                    :ref="(element) => setTerminalRef(item.kind, element)"
                    :items="terminalLogs[item.kind]"
                    :virtual-scroll-item-size="28"
                    class="file-terminal__scroll"
                    separator
                  >
                    <template #default="{ item: line, index }">
                      <div :key="`${item.kind}-${index}-${line}`" class="file-terminal__line">
                        <span class="file-terminal__index">{{ String(index + 1).padStart(2, "0") }}</span>
                        <span>{{ line }}</span>
                      </div>
                    </template>
                  </q-virtual-scroll>
                </template>
                <div v-else class="file-terminal__empty">
                  {{ t("software.local.waitingLogs", { label: item.label }) }}
                </div>
              </div>
            </div>
            <div class="file-card__actions">
              <input
                :ref="(element) => setLocalFileInput(item.kind, element)"
                class="hidden-input"
                type="file"
                :accept="item.accept"
                @change="handleLocalFileChange($event, item.kind)"
              />
              <q-btn push color="primary" :label="t('software.local.selectFile', { label: item.label })" @click="openLocalFile(item.kind)" />
              <q-btn
                outline
                color="negative"
                :label="t('software.local.clear')"
                :disable="!localFiles[item.kind].fileName"
                @click="resetLocalFile(item.kind)"
              />
              <q-btn
                push
                color="positive"
                :label="t('software.local.startUpgrade')"
                :loading="upgradeLoading"
                :disable="!localFiles[item.kind].fileName || !sharedCqCode || !isDeviceConnected"
                @click="handleSingleLocalUpgrade(item.kind)"
              />
            </div>
          </div>
        </div>
      </section>
    </section>
  </div>
</template>

<script setup>
import { listen } from "@tauri-apps/api/event"
import { computed, nextTick, onBeforeUnmount, onMounted, reactive, ref, watch } from "vue"
import { useI18n } from "vue-i18n"
import { useDeviceStore } from "@/store/device"
import { notifyError, notifySuccess } from "@/services/ui"
import {
  fileToBytes,
  loadProgramBurningBundle,
  performRealtimeUpgrade,
  prepareOfflineUpgrade,
} from "@/api/unimaster"
import { prepareMeterConfigUpgradeFile } from "@/utils/unimaster-config"
import {
  buildUpgradeCqCode,
  createDefaultUpgradeCqState,
  getUpgradeBurnFileType,
  parseUpgradeCqCode,
} from "@/utils/upgrade-cq"

const deviceStore = useDeviceStore()
const { t } = useI18n()

const kindOrder = ["boot", "app", "ui", "config"]
const localKindOrder = ["app", "ui"]
const kindLabelMap = {
  boot: "BOOT",
  app: "APP",
  ui: "UI",
  config: "CFG",
}

const localFileMetas = [
  { kind: "boot", label: "BOOT", accept: ".bin", tip: "BIN" },
  { kind: "app", label: "APP", accept: ".hex,.bin", tip: "HEX / BIN" },
  { kind: "ui", label: "UI", accept: ".txt,.bin", tip: "TXT / BIN" },
  { kind: "config", label: "CFG", accept: ".json,.ini", tip: "JSON / INI" },
]

const onlineBundleLoading = ref(false)
const syncLoading = ref(false)
const upgradeLoading = ref(false)
const onlineBundleForm = reactive({
  codeOrSn: "8QeJB9d3c8",
})

const onlineBundle = reactive({
  computerName: "",
})

function createFileState(kind) {
  return {
    kind,
    label: kindLabelMap[kind],
    fileName: "",
    sourceUrl: "",
    data: null,
    isActive: false,
    percent: 0,
  }
}

const onlineFilesState = reactive({
  boot: createFileState("boot"),
  app: createFileState("app"),
  ui: createFileState("ui"),
  config: createFileState("config"),
})

const localFiles = reactive({
  boot: createFileState("boot"),
  app: createFileState("app"),
  ui: createFileState("ui"),
  config: createFileState("config"),
})

const localInputs = reactive({
  boot: null,
  app: null,
  ui: null,
  config: null,
})
const terminalLogs = reactive({
  app: [],
  ui: [],
})
const terminalRefs = reactive({
  app: null,
  ui: null,
})
const terminalBodyRefs = reactive({
  app: null,
  ui: null,
})

const operationState = reactive({
  stage: "",
  logs: [],
})
let currentUpgradeGroup = null
let unlistenUpgradeProgress = null
let currentLogKinds = []

const realtimeForm = reactive({
  model: deviceStore.currentModel,
  ...createDefaultUpgradeCqState(),
  protocolType: 0x01,
  burnFileType: 1,
})

watch(
  () => deviceStore.currentModel,
  (value) => {
    realtimeForm.model = value
  },
  { immediate: true },
)

onMounted(async () => {
  unlistenUpgradeProgress = await listen("upgrade-progress", (event) => {
    applyUpgradeProgress(event.payload)
  })
})

onBeforeUnmount(() => {
  unlistenUpgradeProgress?.()
  unlistenUpgradeProgress = null
})

const hasActiveLocalFiles = computed(() => localKindOrder.some((kind) => localFiles[kind].isActive))
const localSelectedCount = computed(() => localKindOrder.filter((kind) => localFiles[kind].isActive).length)
const localReadyCount = computed(() => localKindOrder.filter((kind) => localFiles[kind].fileName).length)
const sharedCqCode = computed(() => String(deviceStore.upgradeCqCode || "").trim().toUpperCase())
const isDeviceConnected = computed(() => deviceStore.connectionStatus === "CONNECTED")

async function refreshConnectionStatus() {
  try {
    await deviceStore.syncStatus()
  } catch (_) {
    // 状态刷新失败时沿用当前状态，避免覆盖主错误提示。
  }
}

async function syncDisconnectedStateIfNeeded(error) {
  const message = String(error?.message ?? error ?? "")
  if (!message.includes("请先连接串口适配器") && !message.includes("串口连接已断开")) {
    return
  }

  await refreshConnectionStatus()
}

function getUpgradeCommType() {
  return deviceStore.meterCommType === 0x02 ? 0x02 : 0x00
}

function buildRealtimeInitRequest(file) {
  const parsedCq = parseUpgradeCqCode(sharedCqCode.value)
  const burnFileType = getUpgradeBurnFileType(file.kind)
  const resolvedCqCode = buildUpgradeCqCode(parsedCq, burnFileType)

  return {
    ...realtimeForm,
    model: onlineBundle.computerName || realtimeForm.model || deviceStore.currentModel || "UniMaster",
    commType: parsedCq.commType,
    baudCode: parsedCq.baudCode,
    frameType: parsedCq.frameType,
    powerVoltage: parsedCq.powerVoltage,
    vlk5vEnabled: Boolean(parsedCq.vlk5vEnabled),
    protocolType: parsedCq.protocolType,
    burnFileType,
    frameId: parsedCq.frameId,
    cqCode: resolvedCqCode,
    fileName: file.fileName,
  }
}

async function loadOnlineBundle() {
  onlineBundleLoading.value = true
  try {
    resetFileGroup(onlineFilesState)
    onlineBundle.computerName = ""
    const result = await loadProgramBurningBundle(onlineBundleForm.codeOrSn)
    onlineBundle.computerName = result.computerName || ""

    for (const file of result.files || []) {
      const normalized = await normalizeBundleFile(file)
      if (!normalized) {
        continue
      }
      Object.assign(onlineFilesState[normalized.kind], normalized, {
        isActive: Boolean(normalized.fileName),
        percent: 0,
      })
    }

    if (onlineBundle.computerName) {
      deviceStore.setModel(onlineBundle.computerName)
    }

    const loadedCount = kindOrder.filter((kind) => onlineFilesState[kind].fileName).length
    if (!loadedCount) {
      appendLogs(["未找到可用在线文件"])
      notifyError("未找到可用在线文件")
      return
    }

    appendLogs([`在线资源加载成功，共 ${loadedCount} 个文件`])
    notifySuccess("在线资源加载成功")
  } catch (error) {
    notifyError(error)
  } finally {
    onlineBundleLoading.value = false
  }
}

function toggleOnlineItem(kind) {
  if (!onlineFilesState[kind].fileName) {
    return
  }
  onlineFilesState[kind].isActive = !onlineFilesState[kind].isActive
}

function toggleLocalItem(kind) {
  if (!localFiles[kind].fileName) {
    return
  }
  localFiles[kind].isActive = !localFiles[kind].isActive
}

function getFileStateLabel(file) {
  if (!file?.fileName) {
    return t("software.local.fileState.empty")
  }
  return file.isActive ? t("software.local.fileState.selected") : t("software.local.fileState.pending")
}

function getSourceMeta(sourceUrl) {
  if (!sourceUrl) {
    return t("software.local.source.onlineBundle")
  }

  try {
    return new URL(sourceUrl).hostname || sourceUrl
  } catch {
    return sourceUrl
  }
}

function setLocalFileInput(kind, element) {
  localInputs[kind] = element
}

function openLocalFile(kind) {
  if (["app", "ui", "boot", "config"].includes(kind)) {
    deviceStore.softwareUpgradeTargetKind = kind
  }
  localInputs[kind]?.click()
}

async function handleLocalFileChange(event, kind) {
  const file = event?.target?.files?.[0]
  event.target.value = ""
  if (!file) {
    return
  }

  try {
    if (["app", "ui", "boot", "config"].includes(kind)) {
      deviceStore.softwareUpgradeTargetKind = kind
    }
    const prepared = await buildLocalFileEntry(file, kind)
    Object.assign(localFiles[kind], prepared, {
      isActive: true,
      percent: 0,
    })
  } catch (error) {
    notifyError(error)
  }
}

function resetLocalFile(kind) {
  Object.assign(localFiles[kind], createFileState(kind))
}

async function handleOnlineSync() {
  await runSync(Object.values(onlineFilesState).filter((item) => item.isActive), onlineFilesState, "在线获取")
}

async function handleLocalSync() {
  await runSync(Object.values(localFiles).filter((item) => item.isActive), localFiles, t("software.local.source.localFile"))
}

async function handleOnlineUpgrade() {
  await runUpgrade(Object.values(onlineFilesState).filter((item) => item.isActive), onlineFilesState, "在线获取")
}

async function handleLocalUpgrade() {
  await runUpgrade(Object.values(localFiles).filter((item) => item.isActive), localFiles, t("software.local.source.localFile"))
}

async function handleSingleLocalUpgrade(kind) {
  const file = localFiles[kind]
  if (!file?.fileName) {
    notifyError(t("software.local.errors.selectFileFirst"))
    return
  }
  await refreshConnectionStatus()
  if (!isDeviceConnected.value) {
    notifyError("请先连接串口适配器")
    return
  }
  deviceStore.softwareUpgradeTargetKind = kind
  if (!sharedCqCode.value) {
    notifyError("请先在左侧连接设备面板填写 CQ 配置串")
    return
  }

  file.isActive = true
  await runUpgrade([file], localFiles, t("software.local.source.localFileWithLabel", { label: file.label }))
}

async function runSync(selectedFiles, groupState, sourceLabel) {
  if (!selectedFiles.length) {
    notifyError(t("software.local.errors.selectFileFirst"))
    return
  }

  currentLogKinds = selectedFiles.map((item) => item.kind)
  syncLoading.value = true
  resetProgress(groupState)
  operationState.stage = t("software.local.sync.inProgress")
  operationState.logs = [t("software.local.sync.started", { source: sourceLabel })]
  appendLogs([t("software.local.sync.started", { source: sourceLabel })], currentLogKinds)

  try {
    const files = await toUpgradeFiles(selectedFiles)
    const commType = getUpgradeCommType()
    const result = await prepareOfflineUpgrade({
      model: onlineBundle.computerName || deviceStore.currentModel || "UniMaster",
      powerVoltage: 0xff,
      commType,
      bootFileType: 0xff,
      appFileType: 0xff,
      uiFileType: 0xff,
      configFileType: 0xff,
      configCommType: commType === 0x02 ? 2 : 1,
      configBaudCode: Number(deviceStore.meterBaudCode ?? 0x0b),
      configFrameType: commType === 0x02 ? Number(deviceStore.meterFrameType ?? 0) : 0,
      uiVersion: "",
      files,
    })

    setSelectedProgress(groupState, 100)
    appendLogs(result.logs || [], currentLogKinds)
    operationState.stage = result.stage || t("software.local.sync.completed")
    notifySuccess(result.stage || t("software.local.sync.completed"))
  } catch (error) {
    operationState.stage = t("software.local.sync.failed")
    appendLogs([String(error)], currentLogKinds)
    notifyError(error)
  } finally {
    syncLoading.value = false
  }
}

async function runUpgrade(selectedFiles, groupState, sourceLabel) {
  if (!selectedFiles.length) {
    notifyError(t("software.local.errors.selectFileFirst"))
    return
  }
  await refreshConnectionStatus()
  if (!isDeviceConnected.value) {
    notifyError("请先连接串口适配器")
    return
  }

  currentLogKinds = selectedFiles.map((item) => item.kind)
  upgradeLoading.value = true
  deviceStore.setUpgradeInProgress(true)
  currentUpgradeGroup = groupState
  resetProgress(groupState)
  operationState.stage = t("software.local.upgrade.inProgress")
  operationState.logs = [t("software.local.upgrade.started", { source: sourceLabel })]
  appendLogs([t("software.local.upgrade.started", { source: sourceLabel })], currentLogKinds)

  try {
    const files = await toUpgradeFiles(selectedFiles)
    files.forEach((file) => {
      const request = buildRealtimeInitRequest(file)
      appendLogs([`使用 CQ 配置：${request.cqCode}`], [file.kind])
    })

    for (const [index, file] of files.entries()) {
      const result = await performRealtimeUpgrade({
        init: buildRealtimeInitRequest(file),
        files: [file],
      })

      if (result.success) {
        setProgressForKind(groupState, file.kind, 100)
      }
      operationState.stage = result.stage || t("software.local.upgrade.inProgress")

      if (!result.success) {
        throw new Error(result.stage || t("software.local.upgrade.fileFailed", { fileName: file.fileName }))
      }

      if (index === files.length - 1) {
        operationState.stage = t("software.local.upgrade.completed")
      }
    }

    notifySuccess(operationState.stage || t("software.local.upgrade.completed"))
  } catch (error) {
    await syncDisconnectedStateIfNeeded(error)
    operationState.stage = t("software.local.upgrade.failed")
    appendLogs([String(error)], currentLogKinds)
    notifyError(error)
  } finally {
    currentUpgradeGroup = null
    deviceStore.setUpgradeInProgress(false)
    upgradeLoading.value = false
  }
}

async function toUpgradeFiles(selectedFiles) {
  const result = []
  for (const item of selectedFiles) {
    result.push({
      kind: item.kind,
      fileName: item.fileName,
      data: Array.isArray(item.data) ? item.data : [],
    })
  }
  return result
}

async function normalizeBundleFile(file) {
  if (!file?.kind || !kindOrder.includes(file.kind)) {
    return null
  }

  if (file.kind === "config") {
    const prepared = await prepareMeterConfigUpgradeFile({
      name: file.fileName || "config.json",
      text: async () => String(file.text || ""),
    })
    return {
      kind: file.kind,
      label: kindLabelMap[file.kind],
      fileName: prepared.fileName || file.fileName || "config.json",
      sourceUrl: file.sourceUrl || "",
      data: prepared.data,
    }
  }

  return {
    kind: file.kind,
    label: kindLabelMap[file.kind],
    fileName: file.fileName || `${file.kind}.bin`,
    sourceUrl: file.sourceUrl || "",
    data: Array.isArray(file.bytes) ? file.bytes : [],
  }
}

async function buildLocalFileEntry(file, kind) {
  if (kind === "config") {
    const prepared = await prepareMeterConfigUpgradeFile(file)
    return {
      kind,
      label: kindLabelMap[kind],
      fileName: prepared.fileName || file.name,
      sourceUrl: "",
      data: prepared.data,
    }
  }

  return {
    kind,
    label: kindLabelMap[kind],
    fileName: file.name,
    sourceUrl: "",
    data: await fileToBytes(file),
  }
}

function resetFileGroup(group) {
  kindOrder.forEach((kind) => {
    Object.assign(group[kind], createFileState(kind))
  })
}

function resetProgress(group) {
  kindOrder.forEach((kind) => {
    if (group[kind]) {
      group[kind].percent = group[kind].isActive ? 0 : group[kind].percent
    }
  })
}

function getRealtimeBurnType(kind) {
  switch (kind) {
    case "boot":
      return 0
    case "app":
      return 1
    case "ui":
      return 2
    case "config":
      return 3
    default:
      return 1
  }
}

function setProgressForKind(group, kind, value) {
  if (group[kind]) {
    group[kind].percent = value
  }
}

function setSelectedProgress(group, value) {
  kindOrder.forEach((kind) => {
    if (group[kind]?.isActive) {
      group[kind].percent = value
    }
  })
}

function applyUpgradeProgress(payload) {
  if (!payload) {
    return
  }

  operationState.stage = payload.stage || operationState.stage

  if (currentUpgradeGroup && payload.kind) {
    setProgressForKind(
      currentUpgradeGroup,
      payload.kind,
      Number(payload.fileProgress ?? payload.progress ?? currentUpgradeGroup[payload.kind]?.percent ?? 0),
    )
  }

  if (payload.log) {
    appendLogs([payload.log], payload.kind ? [payload.kind] : currentLogKinds)
  }
}

function appendLogs(logs, targetKinds = []) {
  operationState.logs.push(...logs)
  nextTick(() => {
    targetKinds
      .filter((kind) => Array.isArray(terminalLogs[kind]))
      .forEach((kind) => {
        terminalLogs[kind].push(...logs)
        scrollTerminalToBottom(kind)
      })
  })
}

function setTerminalRef(kind, element) {
  terminalRefs[kind] = element
}

function setTerminalBodyRef(kind, element) {
  terminalBodyRefs[kind] = element
}

function scrollTerminalToBottom(kind, retry = 0) {
  const lastIndex = terminalLogs[kind].length - 1
  const virtualScroll = terminalRefs[kind]
  const body = terminalBodyRefs[kind]

  if (virtualScroll?.scrollTo && lastIndex >= 0) {
    virtualScroll.scrollTo(lastIndex, "end")
  }

  const applyDomScroll = () => {
    const scrollElement = body?.querySelector?.(".q-virtual-scroll") || virtualScroll?.$el || body
    if (scrollElement) {
      scrollElement.scrollTop = scrollElement.scrollHeight
    }

    if (retry < 2) {
      requestAnimationFrame(() => scrollTerminalToBottom(kind, retry + 1))
    }
  }

  requestAnimationFrame(applyDomScroll)
}

function clearLocalLogs(kind) {
  if (Array.isArray(terminalLogs[kind])) {
    terminalLogs[kind] = []
  }
}

function clearLogs() {
  operationState.logs = []
  Object.keys(terminalLogs).forEach((kind) => {
    if (Array.isArray(terminalLogs[kind])) {
      terminalLogs[kind] = []
    }
  })
}
</script>

<style scoped lang="scss">
.program-burning-page {
  --software-accent: var(--dt-accent);
  --software-accent-soft: var(--dt-brand-primary-soft);
  --software-accent-strong: color-mix(in srgb, var(--dt-accent) 20%, transparent);
  --software-panel-edge: var(--dt-border);
  --software-panel-shadow: var(--dt-shadow-panel);
  --software-glow: radial-gradient(circle at top right, color-mix(in srgb, var(--dt-accent) 16%, transparent), transparent 34%);
  --software-page-bg: linear-gradient(
    180deg,
    color-mix(in srgb, var(--dt-bg-panel-strong) 58%, transparent),
    color-mix(in srgb, var(--dt-bg-panel-soft) 18%, transparent)
  );
  --software-workspace-bg: var(--dt-gloss-surface);
  --software-card-bg: var(--dt-gloss-surface);
  --software-card-bg-soft: var(--dt-gloss-surface-soft);
  --software-card-bg-ghost: var(--dt-gloss-surface-ghost);
  --software-card-bg-active: var(--dt-gloss-surface-ghost);
  --software-dropzone-bg: linear-gradient(
    180deg,
    color-mix(in srgb, var(--dt-bg-panel) 92%, white 8%),
    color-mix(in srgb, var(--dt-bg-panel-soft) 94%, transparent)
  );
  --software-dropzone-bg-active: linear-gradient(
    180deg,
    color-mix(in srgb, var(--dt-bg-panel) 88%, var(--dt-accent) 12%),
    color-mix(in srgb, var(--dt-bg-panel-soft) 84%, var(--dt-accent) 16%)
  );
  --software-chip-bg: color-mix(in srgb, var(--dt-text-primary) 6%, transparent);
  --software-chip-text: var(--dt-text-secondary);
  --software-tag-bg: color-mix(in srgb, var(--dt-accent) 12%, transparent);
  --software-tag-text: var(--dt-accent);
  --software-icon-bg: linear-gradient(
    135deg,
    color-mix(in srgb, var(--dt-accent) 14%, transparent),
    color-mix(in srgb, var(--dt-accent) 22%, transparent)
  );
  --software-icon-text: var(--dt-accent);
  --software-progress-bg: color-mix(in srgb, var(--dt-text-secondary) 18%, transparent);
  --software-dropzone-border: color-mix(in srgb, var(--dt-border) 80%, var(--dt-accent) 20%);
  --software-card-border: color-mix(in srgb, var(--dt-border) 86%, transparent);
  --software-terminal-border: color-mix(in srgb, var(--dt-border) 72%, var(--dt-accent) 28%);
  --software-terminal-bg:
    linear-gradient(
      180deg,
      color-mix(in srgb, var(--dt-bg-panel) 94%, #08111d 6%),
      color-mix(in srgb, var(--dt-bg-panel-soft) 90%, #102036 10%)
    ),
    radial-gradient(circle at top right, color-mix(in srgb, var(--dt-accent) 14%, transparent), transparent 36%);
  --software-terminal-text: color-mix(in srgb, var(--dt-text-primary) 86%, #d9e6f4 14%);
  --software-terminal-muted: color-mix(in srgb, var(--dt-text-secondary) 78%, #9cb3cd 22%);
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 100%;
  height: 100%;
  padding: 12px;
  box-sizing: border-box;
  overflow: hidden;
}

.program-burning-page--local {
  overflow: hidden;
}

.workspace-panel__head h3,
.feature-panel__head h3,
.action-strip__copy h3,
.log-panel__head strong {
  margin: 0;
  color: var(--dt-text-primary);
  font-size: 18px;
  font-weight: 700;
  line-height: 1.3;
}

.workspace-panel__head p,
.feature-panel__head p {
  margin: 6px 0 0;
  color: var(--dt-text-secondary);
  font-size: 13px;
  line-height: 1.6;
}

.summary-card,
.status-card {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 16px 18px;
  border: 1px solid var(--software-card-border);
  border-radius: 16px;
  background: var(--software-card-bg);
  box-shadow: var(--dt-gloss-inset);
}

.summary-card span,
.status-card span {
  color: var(--dt-text-secondary);
  font-size: 12px;
}

.summary-card strong,
.status-card strong {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  color: var(--dt-text-primary);
  font-size: 15px;
  font-weight: 700;
  line-height: 1.5;
  word-break: break-word;
}

.workspace-panel {
  display: flex;
  flex-direction: column;
  gap: 18px;
  padding: 16px;
  flex: 0 0 auto;
  border: 1px solid var(--software-panel-edge);
  border-radius: 18px;
  background: var(--software-workspace-bg), var(--software-glow);
}

.workspace-panel--local {
  flex: 1 1 auto;
  min-height: 0;
  height: 100%;
  padding: 0;
  background: transparent;
  border: 0;
  border-radius: 0;
  box-shadow: none;
}

.program-burning-page--local .workspace-panel {
  gap: 12px;
}

.workspace-panel__head {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  gap: 0;
  padding: 0;
}

.source-tabs {
  min-width: 220px;
}

.source-tabs :deep(.q-tabs__content) {
  gap: 20px;
}

.source-tabs :deep(.q-tab) {
  min-height: 40px;
  padding: 0;
  color: var(--dt-text-muted);
  font-weight: 700;
  transition: none;
}

.source-tabs :deep(.q-tab__label) {
  font-size: 16px;
  font-weight: 800;
}

.source-tabs :deep(.q-tab:hover) {
  color: var(--dt-text-secondary);
}

.source-tabs :deep(.q-tab--active) {
  color: var(--dt-accent);
}

.source-tabs :deep(.q-tab .q-focus-helper),
.source-tabs :deep(.q-tab:hover .q-focus-helper) {
  opacity: 0 !important;
  background: transparent !important;
}

.source-tabs :deep(.q-tabs__arrow) {
  display: none;
}

.source-tabs :deep(.q-tab__indicator) {
  height: 3px;
  border-radius: 999px;
}

.workspace-panel__content {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.workspace-panel__content--local {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
}

.local-section-head {
  flex: 0 0 auto;
  padding: 2px 2px 0;
}

.workspace-panel__content--local .feature-panel {
  flex: 1 1 auto;
  min-height: 0;
  gap: 14px;
  padding: 16px;
}

.workspace-panel__content--local .feature-panel__head,
.local-section-head .feature-panel__head {
  padding-bottom: 2px;
}

.workspace-panel__content--local .feature-panel__head h3,
.local-section-head .feature-panel__head h3 {
  font-size: 16px;
}

.workspace-panel__content--local .feature-panel__head p,
.local-section-head .feature-panel__head p {
  font-size: 12px;
}

.software-grid {
  display: grid;
  gap: 16px;
}

.software-grid--online {
  grid-template-columns: minmax(340px, 0.84fr) minmax(0, 1.16fr);
}

.feature-panel,
.files-panel,
.status-panel,
.log-panel {
  padding: 18px;
  border-radius: 18px;
}

.feature-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.workspace-panel__content--local .feature-panel {
  flex: 1 1 auto;
  min-height: 0;
}

.feature-panel__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding-bottom: 6px;
}

.panel-badge {
  display: inline-flex;
  align-items: center;
  min-height: 28px;
  padding: 0 12px;
  border-radius: 999px;
  background: var(--software-tag-bg);
  color: color-mix(in srgb, var(--software-tag-text) 74%, var(--dt-text-secondary) 26%);
  font-size: 12px;
  font-weight: 800;
  white-space: nowrap;
}

.panel-badge--soft {
  background: color-mix(in srgb, var(--dt-bg-chip) 72%, transparent);
  border: 1px solid var(--software-card-border);
}

.loader-form {
  display: flex;
  align-items: end;
  gap: 14px;
  padding: 16px;
  border: 1px solid var(--software-card-border);
  border-radius: 16px;
  background: var(--software-card-bg-soft);
}

.field-grow {
  flex: 1;
}

.field-label {
  display: block;
  margin-bottom: 8px;
  color: var(--dt-text-secondary);
  font-size: 13px;
  font-weight: 700;
}

.summary-grid,
.status-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 12px;
}

.status-grid {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.status-card--wide {
  grid-column: 1 / -1;
}

.file-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
}

.file-grid--local {
  flex: 1 1 auto;
  min-height: 0;
  height: 100%;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  grid-auto-rows: minmax(0, 1fr);
  align-content: stretch;
  align-items: stretch;
}

.file-card {
  display: flex;
  flex-direction: column;
  gap: 14px;
  width: 100%;
  min-height: 216px;
  padding: 18px;
  border: 1px solid var(--software-card-border);
  border-radius: var(--dt-radius-subtle);
  background: var(--software-card-bg), var(--software-glow);
  box-sizing: border-box;
  text-align: left;
  color: inherit;
  transition: border-color 0.2s ease, transform 0.2s ease, box-shadow 0.2s ease, background 0.2s ease;
}

.file-card--local {
  min-height: 0;
  height: 100%;
  padding: 16px 18px 16px;
  gap: 14px;
  border-color: color-mix(in srgb, var(--dt-border-strong) 78%, transparent);
  background: var(--software-card-bg-soft), var(--software-glow);
  box-shadow: var(--dt-gloss-inset), var(--dt-shadow-panel);
}

.file-card:not(.file-card--local) {
  cursor: pointer;
}

.file-card:hover {
  border-color: color-mix(in srgb, var(--dt-accent) 28%, var(--dt-border) 72%);
  box-shadow: var(--dt-shadow-float);
}

.file-card--active {
  border-color: color-mix(in srgb, var(--dt-accent) 44%, var(--dt-border) 56%);
  background: var(--software-card-bg-active), var(--software-glow);
  box-shadow: var(--dt-gloss-inset), 0 20px 36px color-mix(in srgb, var(--dt-accent) 14%, transparent);
}

.file-card--local.file-card--active {
  border-color: color-mix(in srgb, var(--dt-accent) 52%, var(--dt-border-strong) 48%);
  background: var(--software-card-bg-active), var(--software-glow);
}

.file-card__head {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 10px;
  padding: 0;
  border: 0;
  background: transparent;
  box-shadow: none;
}

.file-card__head-main {
  display: flex;
  min-width: 0;
  flex: 1;
}

.file-card__labels {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  align-items: center;
}

.file-card__dropzone {
  display: flex;
  align-items: center;
  gap: 14px;
  min-height: 72px;
  padding: 14px 16px;
  border: 1px dashed var(--software-dropzone-border);
  border-radius: var(--dt-radius-subtle);
  background: var(--software-dropzone-bg);
}

.file-card--active .file-card__dropzone {
  border-color: color-mix(in srgb, var(--dt-accent) 58%, var(--dt-border) 42%);
  background: var(--software-dropzone-bg-active);
}

.file-card__dropzone-icon {
  display: grid;
  place-items: center;
  width: 50px;
  height: 50px;
  border-radius: var(--dt-radius-subtle);
  background: var(--software-icon-bg);
  color: var(--software-icon-text);
  box-shadow: var(--dt-gloss-inset);
  flex: 0 0 auto;
}

.file-card__dropzone-copy {
  min-width: 0;
}

.file-card__dropzone-copy strong {
  display: block;
  color: var(--dt-text-primary);
  font-size: 14px;
  font-weight: 800;
  line-height: 1.35;
}

.file-card__dropzone-copy p {
  margin: 6px 0 0;
  color: var(--dt-text-secondary);
  font-size: 12px;
  line-height: 1.55;
}

.file-card__tag {
  display: inline-flex;
  align-items: center;
  min-height: 24px;
  padding: 0 10px;
  border-radius: 999px;
  background: var(--software-tag-bg);
  color: var(--software-tag-text);
  font-size: 11px;
  font-weight: 800;
  letter-spacing: 0.04em;
}

.file-card__format {
  display: inline-flex;
  align-items: center;
  min-height: 22px;
  padding: 0 9px;
  border-radius: 999px;
  background: var(--software-chip-bg);
  color: var(--software-chip-text);
  font-size: 11px;
  font-weight: 700;
}

.file-state-chip,
.log-stage {
  display: inline-flex;
  align-items: center;
  min-height: 24px;
  padding: 0 10px;
  border-radius: 999px;
  border: 1px solid color-mix(in srgb, var(--dt-border) 72%, transparent);
  background: color-mix(in srgb, var(--dt-bg-chip) 68%, transparent);
  color: var(--software-chip-text);
  font-size: 11px;
  font-weight: 800;
  white-space: nowrap;
  box-shadow: none;
  margin-left: auto;
}

.file-state-chip--active {
  border-color: color-mix(in srgb, var(--dt-accent) 22%, transparent);
  background: color-mix(in srgb, var(--dt-accent) 12%, transparent);
  color: var(--dt-accent);
}

.file-card__meta {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.file-card__meta--inline {
  flex-direction: row;
  align-items: center;
  gap: 8px;
  color: var(--dt-text-secondary);
}

.file-card__meta span {
  color: var(--dt-text-secondary);
  font-size: 12px;
}

.file-card__meta p {
  margin: 0;
  color: var(--dt-text-primary);
  font-size: 13px;
  line-height: 1.6;
  word-break: break-all;
}

.file-card__meta--inline p {
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  word-break: normal;
}

.file-card__progress {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--dt-text-secondary);
  font-size: 12px;
  margin-top: auto;
}

.file-card--local .file-card__progress {
  margin-top: 2px;
}

.file-card__progress :deep(.q-linear-progress) {
  flex: 1;
  border-radius: 999px;
  background: var(--software-progress-bg);
}

.file-terminal {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  min-height: 0;
  gap: 8px;
}

.file-terminal__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.file-terminal__head strong {
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 800;
  letter-spacing: 0.04em;
}

.file-terminal__body {
  flex: 1 1 auto;
  min-height: 220px;
  overflow: hidden;
  padding: 0;
  border: 1px solid var(--software-terminal-border);
  border-radius: 10px;
  background: var(--software-terminal-bg);
  color: var(--software-terminal-text);
  box-sizing: border-box;
}

.file-terminal__scroll {
  height: 100%;
  min-height: 220px;
  padding: 16px 18px;
  font-family: "SFMono-Regular", "Menlo", "Monaco", monospace;
  font-size: 12px;
  line-height: 1.7;
  white-space: pre-wrap;
  word-break: break-word;
  box-sizing: border-box;
}

.file-terminal__line {
  display: flex;
  gap: 10px;
  align-items: flex-start;
  padding: 1px 0;
}

.file-terminal__index {
  min-width: 24px;
  color: var(--software-terminal-muted);
  font-variant-numeric: tabular-nums;
  text-align: right;
}

.file-terminal__empty {
  color: var(--software-terminal-muted);
  padding: 16px 18px;
  line-height: 1.7;
}

.file-card__actions {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
  align-items: center;
  gap: 12px;
  padding-top: 4px;
}

.file-card--local .file-card__actions {
  gap: 12px;
  padding-top: 0;
  margin-top: auto;
}

.file-card__actions > :deep(.q-btn) {
  min-height: 38px;
  min-width: 94px;
  padding: 0 18px;
  border-radius: var(--dt-radius-button);
  font-weight: 800;
}

.file-card--local .file-card__actions > :deep(.q-btn) {
  min-height: 40px;
  min-width: 108px;
  padding: 0 18px;
}

.hidden-input {
  display: none;
}

.action-strip {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 14px;
  padding: 16px 18px;
  border: 1px solid var(--software-card-border);
  border-radius: 18px;
  background: var(--software-card-bg-soft), radial-gradient(circle at left center, var(--software-accent-soft), transparent 34%);
}

.workspace-panel__content--local .action-strip {
  flex: 0 0 auto;
  margin-top: auto;
}

.action-strip__copy {
  flex: 0 0 auto;
  display: none;
}

.action-strip__actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.action-strip__actions > :deep(.q-btn) {
  min-height: 40px;
  min-width: 116px;
  border-radius: var(--dt-radius-button);
  font-weight: 800;
}

.empty-state,
.log-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  min-height: 220px;
  border: 1px dashed color-mix(in srgb, var(--dt-border) 80%, var(--dt-accent) 20%);
  border-radius: 18px;
  color: var(--dt-text-secondary);
  background: var(--software-card-bg-ghost);
}

.software-bottom-grid {
  display: grid;
  grid-template-columns: minmax(320px, 0.76fr) minmax(0, 1.24fr);
  gap: 16px;
  flex: 0 0 auto;
  min-height: 0;
}

.status-panel {
  display: flex;
  flex-direction: column;
  gap: 16px;
  min-height: 0;
}

.log-panel {
  display: flex;
  flex-direction: column;
  min-height: 280px;
}

.log-panel__head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.log-panel__body {
  display: flex;
  flex-direction: column;
  gap: 10px;
  flex: 1;
  overflow: auto;
  margin-top: 8px;
  padding: 16px;
  border: 1px solid rgba(82, 107, 145, 0.34);
  border-radius: 18px;
  background:
    linear-gradient(180deg, rgba(11, 20, 34, 0.98), rgba(17, 31, 50, 0.98)),
    radial-gradient(circle at top right, rgba(59, 130, 246, 0.12), transparent 34%);
  color: #d9e6f4;
  font-size: 13px;
  line-height: 1.7;
  white-space: pre-wrap;
  word-break: break-word;
}

.log-line {
  display: flex;
  gap: 12px;
}

.log-line__index {
  min-width: 26px;
  color: rgba(217, 230, 244, 0.54);
  font-variant-numeric: tabular-nums;
}

@media (max-width: 1180px) {
  .file-grid--local,
  .software-bottom-grid {
    grid-template-columns: minmax(0, 1fr);
  }
}

:global(html[data-theme="dark"]) .program-burning-page,
:global(html.theme-dark) .program-burning-page,
:global(body.body--dark) .program-burning-page,
:global(body.theme-dark) .program-burning-page,
:global(body[data-theme="dark"]) .program-burning-page {
  --software-panel-edge: rgba(255, 255, 255, 0.2);
  --software-panel-shadow: 0 26px 56px rgba(0, 0, 0, 0.36);
  --software-glow: radial-gradient(circle at top right, rgba(98, 142, 255, 0.22), transparent 40%);
  --software-page-bg: linear-gradient(180deg, rgba(9, 8, 15, 0.92), rgba(13, 12, 22, 0.84));
  --software-workspace-bg: linear-gradient(180deg, rgba(20, 17, 30, 0.98), rgba(13, 11, 22, 0.98));
  --software-card-bg: linear-gradient(180deg, rgba(31, 26, 44, 0.98), rgba(22, 18, 32, 0.96));
  --software-card-bg-soft: linear-gradient(180deg, rgba(38, 32, 55, 0.98), rgba(25, 21, 37, 0.96));
  --software-card-bg-ghost: linear-gradient(180deg, rgba(28, 24, 41, 0.9), rgba(19, 16, 28, 0.86));
  --software-card-bg-active: linear-gradient(180deg, rgba(35, 49, 82, 0.98), rgba(23, 31, 53, 0.96));
  --software-dropzone-bg: linear-gradient(180deg, rgba(21, 28, 46, 0.94), rgba(15, 21, 35, 0.9));
  --software-dropzone-bg-active: linear-gradient(180deg, rgba(29, 49, 84, 0.98), rgba(20, 35, 63, 0.94));
  --software-chip-bg: rgba(255, 255, 255, 0.1);
  --software-chip-text: #d4d0e3;
  --software-tag-bg: rgba(92, 145, 255, 0.22);
  --software-tag-text: #9fc3ff;
  --software-icon-bg: linear-gradient(135deg, rgba(90, 143, 255, 0.26), rgba(67, 113, 224, 0.36));
  --software-icon-text: #9fc3ff;
  --software-progress-bg: rgba(255, 255, 255, 0.16);
  --software-dropzone-border: rgba(255, 255, 255, 0.28);
  --software-card-border: rgba(255, 255, 255, 0.18);
  --software-terminal-border: rgba(110, 145, 201, 0.34);
  --software-terminal-bg:
    linear-gradient(180deg, rgba(11, 20, 34, 0.98), rgba(17, 31, 50, 0.98)),
    radial-gradient(circle at top right, rgba(59, 130, 246, 0.12), transparent 34%);
  --software-terminal-text: #d9e6f4;
  --software-terminal-muted: rgba(217, 230, 244, 0.6);
  background: var(--software-page-bg), transparent;
}

:global(html[data-theme="dark"]) .program-burning-page .file-state-chip,
:global(html.theme-dark) .program-burning-page .file-state-chip,
:global(body.body--dark) .program-burning-page .file-state-chip,
:global(body.theme-dark) .program-burning-page .file-state-chip,
:global(body[data-theme="dark"]) .program-burning-page .file-state-chip {
  border-color: rgba(144, 177, 229, 0.18);
  background: rgba(255, 255, 255, 0.08);
  color: #d4dceb;
}

:global(html[data-theme="dark"]) .program-burning-page .file-state-chip--active,
:global(html.theme-dark) .program-burning-page .file-state-chip--active,
:global(body.body--dark) .program-burning-page .file-state-chip--active,
:global(body.theme-dark) .program-burning-page .file-state-chip--active,
:global(body[data-theme="dark"]) .program-burning-page .file-state-chip--active {
  border-color: rgba(102, 162, 255, 0.22);
  background: rgba(59, 130, 246, 0.14);
  color: #a8c6ff;
}

:global(html[data-theme="dark"]) .program-burning-page .summary-card,
:global(html[data-theme="dark"]) .program-burning-page .status-card,
:global(html[data-theme="dark"]) .program-burning-page .workspace-panel,
:global(html[data-theme="dark"]) .program-burning-page .file-card,
:global(html[data-theme="dark"]) .program-burning-page .file-card--local,
:global(html[data-theme="dark"]) .program-burning-page .action-strip,
:global(html[data-theme="dark"]) .program-burning-page .empty-state,
:global(html[data-theme="dark"]) .program-burning-page .log-empty,
:global(html.theme-dark) .program-burning-page .summary-card,
:global(html.theme-dark) .program-burning-page .status-card,
:global(html.theme-dark) .program-burning-page .workspace-panel,
:global(html.theme-dark) .program-burning-page .file-card,
:global(html.theme-dark) .program-burning-page .file-card--local,
:global(html.theme-dark) .program-burning-page .action-strip,
:global(html.theme-dark) .program-burning-page .empty-state,
:global(html.theme-dark) .program-burning-page .log-empty,
:global(body.body--dark) .program-burning-page .summary-card,
:global(body.body--dark) .program-burning-page .status-card,
:global(body.body--dark) .program-burning-page .workspace-panel,
:global(body.body--dark) .program-burning-page .file-card,
:global(body.body--dark) .program-burning-page .file-card--local,
:global(body.body--dark) .program-burning-page .action-strip,
:global(body.body--dark) .program-burning-page .empty-state,
:global(body.body--dark) .program-burning-page .log-empty,
:global(body.theme-dark) .program-burning-page .summary-card,
:global(body.theme-dark) .program-burning-page .status-card,
:global(body.theme-dark) .program-burning-page .workspace-panel,
:global(body.theme-dark) .program-burning-page .file-card,
:global(body.theme-dark) .program-burning-page .file-card--local,
:global(body.theme-dark) .program-burning-page .action-strip,
:global(body.theme-dark) .program-burning-page .empty-state,
:global(body.theme-dark) .program-burning-page .log-empty,
:global(body[data-theme="dark"]) .program-burning-page .summary-card,
:global(body[data-theme="dark"]) .program-burning-page .status-card,
:global(body[data-theme="dark"]) .program-burning-page .workspace-panel,
:global(body[data-theme="dark"]) .program-burning-page .file-card,
:global(body[data-theme="dark"]) .program-burning-page .file-card--local,
:global(body[data-theme="dark"]) .program-burning-page .action-strip,
:global(body[data-theme="dark"]) .program-burning-page .empty-state,
:global(body[data-theme="dark"]) .program-burning-page .log-empty {
  border-color: rgba(255, 255, 255, 0.18);
  box-shadow: var(--dt-gloss-inset), var(--software-panel-shadow);
}

:global(html[data-theme="dark"]) .program-burning-page .summary-card,
:global(html[data-theme="dark"]) .program-burning-page .status-card,
:global(html[data-theme="dark"]) .program-burning-page .workspace-panel,
:global(html.theme-dark) .program-burning-page .summary-card,
:global(html.theme-dark) .program-burning-page .status-card,
:global(html.theme-dark) .program-burning-page .workspace-panel,
:global(body.body--dark) .program-burning-page .summary-card,
:global(body.body--dark) .program-burning-page .status-card,
:global(body.body--dark) .program-burning-page .workspace-panel,
:global(body.theme-dark) .program-burning-page .summary-card,
:global(body.theme-dark) .program-burning-page .status-card,
:global(body.theme-dark) .program-burning-page .workspace-panel,
:global(body[data-theme="dark"]) .program-burning-page .summary-card,
:global(body[data-theme="dark"]) .program-burning-page .status-card,
:global(body[data-theme="dark"]) .program-burning-page .workspace-panel {
  background: var(--software-workspace-bg), var(--software-glow);
}

:global(html[data-theme="dark"]) .program-burning-page .panel-badge,
:global(html.theme-dark) .program-burning-page .panel-badge,
:global(body.body--dark) .program-burning-page .panel-badge,
:global(body.theme-dark) .program-burning-page .panel-badge,
:global(body[data-theme="dark"]) .program-burning-page .panel-badge {
  background: rgba(110, 124, 255, 0.16);
  color: #aeb8ff;
}

:global(html[data-theme="dark"]) .program-burning-page .panel-badge--soft,
:global(html.theme-dark) .program-burning-page .panel-badge--soft,
:global(body.body--dark) .program-burning-page .panel-badge--soft,
:global(body.theme-dark) .program-burning-page .panel-badge--soft,
:global(body[data-theme="dark"]) .program-burning-page .panel-badge--soft {
  background: rgba(31, 25, 44, 0.82);
  border-color: rgba(255, 255, 255, 0.18);
  color: var(--dt-text-secondary);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card,
:global(html[data-theme="dark"]) .program-burning-page .file-card--local,
:global(html.theme-dark) .program-burning-page .file-card,
:global(html.theme-dark) .program-burning-page .file-card--local,
:global(body.body--dark) .program-burning-page .file-card,
:global(body.body--dark) .program-burning-page .file-card--local,
:global(body.theme-dark) .program-burning-page .file-card,
:global(body.theme-dark) .program-burning-page .file-card--local,
:global(body[data-theme="dark"]) .program-burning-page .file-card,
:global(body[data-theme="dark"]) .program-burning-page .file-card--local {
  background: var(--software-card-bg), var(--software-glow);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card:hover,
:global(html.theme-dark) .program-burning-page .file-card:hover,
:global(body.body--dark) .program-burning-page .file-card:hover,
:global(body.theme-dark) .program-burning-page .file-card:hover,
:global(body[data-theme="dark"]) .program-burning-page .file-card:hover {
  border-color: rgba(124, 164, 255, 0.5);
  box-shadow: 0 18px 34px rgba(0, 0, 0, 0.34);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card--active,
:global(html.theme-dark) .program-burning-page .file-card--active,
:global(body.body--dark) .program-burning-page .file-card--active,
:global(body.theme-dark) .program-burning-page .file-card--active,
:global(body[data-theme="dark"]) .program-burning-page .file-card--active {
  border-color: rgba(127, 173, 255, 0.66);
  background: var(--software-card-bg-active), var(--software-glow);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.08),
    0 22px 42px rgba(0, 0, 0, 0.38),
    0 0 0 1px rgba(82, 141, 255, 0.16);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card__dropzone,
:global(html.theme-dark) .program-burning-page .file-card__dropzone,
:global(body.body--dark) .program-burning-page .file-card__dropzone,
:global(body.theme-dark) .program-burning-page .file-card__dropzone,
:global(body[data-theme="dark"]) .program-burning-page .file-card__dropzone {
  border-color: var(--software-dropzone-border);
  background: var(--software-dropzone-bg);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card--active .file-card__dropzone,
:global(html.theme-dark) .program-burning-page .file-card--active .file-card__dropzone,
:global(body.body--dark) .program-burning-page .file-card--active .file-card__dropzone,
:global(body.theme-dark) .program-burning-page .file-card--active .file-card__dropzone,
:global(body[data-theme="dark"]) .program-burning-page .file-card--active .file-card__dropzone {
  border-color: rgba(136, 183, 255, 0.78);
  background: var(--software-dropzone-bg-active);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card__dropzone-icon,
:global(html.theme-dark) .program-burning-page .file-card__dropzone-icon,
:global(body.body--dark) .program-burning-page .file-card__dropzone-icon,
:global(body.theme-dark) .program-burning-page .file-card__dropzone-icon,
:global(body[data-theme="dark"]) .program-burning-page .file-card__dropzone-icon {
  background: var(--software-icon-bg);
  color: var(--software-icon-text);
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.08);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card__dropzone-copy strong,
:global(html.theme-dark) .program-burning-page .file-card__dropzone-copy strong,
:global(body.body--dark) .program-burning-page .file-card__dropzone-copy strong,
:global(body.theme-dark) .program-burning-page .file-card__dropzone-copy strong,
:global(body[data-theme="dark"]) .program-burning-page .file-card__dropzone-copy strong {
  color: var(--dt-text-primary);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card__dropzone-copy p,
:global(html.theme-dark) .program-burning-page .file-card__dropzone-copy p,
:global(body.body--dark) .program-burning-page .file-card__dropzone-copy p,
:global(body.theme-dark) .program-burning-page .file-card__dropzone-copy p,
:global(body[data-theme="dark"]) .program-burning-page .file-card__dropzone-copy p {
  color: var(--dt-text-secondary);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card__tag,
:global(html.theme-dark) .program-burning-page .file-card__tag,
:global(body.body--dark) .program-burning-page .file-card__tag,
:global(body.theme-dark) .program-burning-page .file-card__tag,
:global(body[data-theme="dark"]) .program-burning-page .file-card__tag {
  background: var(--software-tag-bg);
  color: var(--software-tag-text);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card__format,
:global(html[data-theme="dark"]) .program-burning-page .file-card .file-state-chip,
:global(html[data-theme="dark"]) .program-burning-page .log-stage,
:global(html.theme-dark) .program-burning-page .file-card__format,
:global(html.theme-dark) .program-burning-page .file-card .file-state-chip,
:global(html.theme-dark) .program-burning-page .log-stage,
:global(body.body--dark) .program-burning-page .file-card__format,
:global(body.body--dark) .program-burning-page .file-card .file-state-chip,
:global(body.body--dark) .program-burning-page .log-stage,
:global(body.theme-dark) .program-burning-page .file-card__format,
:global(body.theme-dark) .program-burning-page .file-card .file-state-chip,
:global(body.theme-dark) .program-burning-page .log-stage,
:global(body[data-theme="dark"]) .program-burning-page .file-card__format,
:global(body[data-theme="dark"]) .program-burning-page .file-card .file-state-chip,
:global(body[data-theme="dark"]) .program-burning-page .log-stage {
  background: var(--software-chip-bg);
  color: var(--software-chip-text);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card__progress,
:global(html.theme-dark) .program-burning-page .file-card__progress,
:global(body.body--dark) .program-burning-page .file-card__progress,
:global(body.theme-dark) .program-burning-page .file-card__progress,
:global(body[data-theme="dark"]) .program-burning-page .file-card__progress {
  color: var(--dt-text-secondary);
}

:global(html[data-theme="dark"]) .program-burning-page .file-card__progress :deep(.q-linear-progress),
:global(html.theme-dark) .program-burning-page .file-card__progress :deep(.q-linear-progress),
:global(body.body--dark) .program-burning-page .file-card__progress :deep(.q-linear-progress),
:global(body.theme-dark) .program-burning-page .file-card__progress :deep(.q-linear-progress),
:global(body[data-theme="dark"]) .program-burning-page .file-card__progress :deep(.q-linear-progress) {
  background: var(--software-progress-bg);
}

:global(html[data-theme="dark"]) .program-burning-page .action-strip,
:global(html.theme-dark) .program-burning-page .action-strip,
:global(body.body--dark) .program-burning-page .action-strip,
:global(body.theme-dark) .program-burning-page .action-strip,
:global(body[data-theme="dark"]) .program-burning-page .action-strip {
  background: var(--software-workspace-bg), radial-gradient(circle at left center, var(--software-accent-soft), transparent 36%);
}

:global(html[data-theme="dark"]) .program-burning-page .empty-state,
:global(html[data-theme="dark"]) .program-burning-page .log-empty,
:global(html.theme-dark) .program-burning-page .empty-state,
:global(html.theme-dark) .program-burning-page .log-empty,
:global(body.body--dark) .program-burning-page .empty-state,
:global(body.body--dark) .program-burning-page .log-empty,
:global(body.theme-dark) .program-burning-page .empty-state,
:global(body.theme-dark) .program-burning-page .log-empty,
:global(body[data-theme="dark"]) .program-burning-page .empty-state,
:global(body[data-theme="dark"]) .program-burning-page .log-empty {
  border-color: rgba(100, 128, 188, 0.46);
  background: var(--software-card-bg-ghost);
}

@media (max-width: 1200px) {
  .software-grid--online,
  .software-bottom-grid {
    grid-template-columns: 1fr;
  }

  .summary-grid,
  .status-grid,
  .file-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .file-grid--local {
    grid-template-rows: none;
  }
}

@media (max-width: 860px) {
  .workspace-panel__head,
  .loader-form,
  .action-strip {
    flex-direction: column;
    align-items: stretch;
  }

  .workspace-panel {
    padding: 14px;
  }

  .source-tabs {
    min-width: 0;
    width: 100%;
  }

  .summary-grid,
  .status-grid,
  .file-grid {
    grid-template-columns: 1fr;
  }

  .file-grid--local {
    grid-template-rows: none;
  }

  .file-card__actions {
    flex-direction: column;
  }

  .action-strip__copy,
  .action-strip__actions {
    width: 100%;
  }

  .action-strip__copy {
    justify-content: space-between;
  }

  .action-strip__actions {
    flex-direction: column;
  }

  .file-card__actions > :deep(.q-btn) {
    width: 100%;
  }

  .action-strip__actions > :deep(.q-btn) {
    width: 100%;
  }
}
</style>
