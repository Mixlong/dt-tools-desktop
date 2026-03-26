<template>
  <div class="software-page">
    <section class="software-head">
      <div>
        <span class="section-eyebrow">Firmware Workspace</span>
        <h2>程序烧录</h2>
        <p>保留版本管理，同时将旧项目中的实时升级、离线烧录和配置文件烧录整合到统一工作台。</p>
      </div>
      <el-tag :type="deviceStore.connectionStatus === 'CONNECTED' ? 'success' : 'info'">
        {{ deviceStore.connectionStatus === "CONNECTED" ? deviceStore.onlineStatus : "未连接适配器" }}
      </el-tag>
    </section>

    <div class="software-tabbar">
      <button
        v-for="item in tabs"
        :key="item.value"
        :class="['software-tab', { 'software-tab--active': activeTab === item.value }]"
        type="button"
        @click="activeTab = item.value"
      >
        {{ item.label }}
      </button>
    </div>

    <div v-if="activeTab === 'version'" class="grid-two">
      <div class="page-card">
        <div class="section-toolbar">
          <div>
            <h3>版本与标志区</h3>
            <p>命令覆盖 `0xA0 / 0xB0 / 0xB1 / 0xB2 / 0xB3 / 0xB4`。</p>
          </div>
          <el-button type="primary" :loading="versionLoading" @click="loadSnapshot">刷新快照</el-button>
        </div>

        <div class="version-summary">
          <div class="summary-item">
            <span>APP 版本</span>
            <strong>{{ snapshot.appVersion || "--" }}</strong>
          </div>
          <div class="summary-item">
            <span>UI 版本</span>
            <strong>{{ snapshot.uiVersion || "--" }}</strong>
          </div>
        </div>

        <el-table :data="snapshot.versionItems" height="320">
          <el-table-column prop="label" label="类型" width="140" />
          <el-table-column prop="value" label="值" min-width="220" show-overflow-tooltip />
          <el-table-column prop="rawHex" label="原始 HEX" min-width="180" show-overflow-tooltip />
        </el-table>
      </div>

      <div class="page-card">
        <h3>写入版本信息 / 标志位</h3>
        <el-form label-position="top">
          <el-form-item label="版本类型">
            <el-select v-model="versionForm.code">
              <el-option
                v-for="item in VERSION_TYPE_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </el-form-item>
          <el-form-item :label="needsHexInput ? 'HEX 内容（4 字节）' : '文本内容'">
            <el-input
              v-model="versionForm.value"
              :placeholder="needsHexInput ? '例如 12 34 56 78' : '例如 APP_BC280_V1.000'"
            />
          </el-form-item>
        </el-form>
        <div class="actions">
          <el-button type="primary" :loading="writeLoading" @click="submitVersion">写入版本信息</el-button>
          <el-button :loading="languageLoading" @click="changeLanguage(0)">切中文</el-button>
          <el-button :loading="languageLoading" @click="changeLanguage(1)">切英文</el-button>
        </div>

        <div class="flag-panel">
          <div class="flag-head">
            <h4>标志区</h4>
            <el-button text @click="refreshFlags">刷新</el-button>
          </div>
          <el-table :data="snapshot.flags" height="220">
            <el-table-column prop="index" label="位置" width="72" />
            <el-table-column prop="label" label="名称" width="120" />
            <el-table-column prop="value" label="值" width="120" />
            <el-table-column prop="hex" label="HEX" min-width="140" />
          </el-table>
          <div class="flag-write">
            <div class="flag-write__field">
              <span class="flag-write__label">位置</span>
              <el-input-number v-model="flagForm.position" :min="0" :max="15" />
            </div>
            <div class="flag-write__field">
              <span class="flag-write__label">HEX 值</span>
              <el-input v-model="flagForm.hexValue" placeholder="4 字节 HEX，如 00 00 00 01" />
            </div>
            <div class="flag-write__field flag-write__field--switch">
              <span class="flag-write__label">写入后关机</span>
              <el-switch v-model="flagForm.shutdownAfterWrite" />
            </div>
            <div class="flag-write__actions">
              <el-button :loading="flagLoading" @click="submitFlag">写入标志位</el-button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else-if="activeTab === 'realtime'" class="grid-two">
      <div class="page-card">
        <div class="section-toolbar">
          <div>
            <h3>实时升级</h3>
            <p>按文件逐个初始化烧录类型，兼容旧项目的 BOOT / APP / UI / 配置文件工作流。</p>
          </div>
          <el-button :loading="realtimeInitLoading" @click="initRealtimeOnly">仅初始化当前参数</el-button>
        </div>

        <el-form label-position="top" class="form-grid">
          <el-form-item label="通讯类型">
            <el-select v-model="realtimeForm.commType">
              <el-option
                v-for="item in REALTIME_COMM_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </el-form-item>
          <el-form-item label="波特率">
            <el-select v-model="realtimeForm.baudCode">
              <el-option
                v-for="item in realtimeBaudOptions"
                :key="item.code"
                :label="item.label"
                :value="item.code"
              />
            </el-select>
          </el-form-item>
          <el-form-item label="帧类型">
            <el-select v-model="realtimeForm.frameType">
              <el-option v-for="item in FRAME_TYPE_OPTIONS" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="供电电压">
            <el-select v-model="realtimeForm.powerVoltage">
              <el-option v-for="item in POWER_OPTIONS" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="协议类型">
            <el-select v-model="realtimeForm.protocolType">
              <el-option v-for="item in PROTOCOL_OPTIONS" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="VLK5V">
            <el-switch v-model="realtimeForm.vlk5vEnabled" />
          </el-form-item>
        </el-form>

        <div class="upload-grid upload-grid--wide">
          <label
            v-for="item in uploadMetas"
            :key="`realtime-${item.key}`"
            class="upload-card"
          >
            <div class="upload-card__head">
              <span>{{ item.label }}</span>
              <small>{{ item.tip }}</small>
            </div>
            <input :accept="item.accept" type="file" @change="onUploadChange($event, realtimeFiles, item.key)" />
            <strong>{{ getUploadedName(realtimeFiles[item.key]) }}</strong>
            <p v-if="item.key === 'config'" class="upload-card__summary">
              {{ getConfigSummary(realtimeFiles.config) }}
            </p>
          </label>
        </div>

        <div class="actions">
          <el-button :loading="accessLoading" @click="checkAccess">检测接入状态</el-button>
          <el-button type="primary" :loading="realtimeLoading" @click="runRealtimeUpgrade">开始实时升级</el-button>
        </div>
      </div>

      <div class="page-card page-card--log">
        <div class="section-toolbar section-toolbar--compact">
          <div>
            <h3>实时升级日志</h3>
            <p>按文件独立初始化，便于追踪旧项目里的逐项烧录流程。</p>
          </div>
          <span class="metric-chip">
            <span class="metric-chip__dot" />
            {{ realtimeResult.stage || "未开始" }}
          </span>
        </div>
        <el-progress :percentage="realtimeResult.progress" :stroke-width="14" />
        <div ref="realtimeLogRef" class="dark-box dark-box--fill">
          <div v-for="(item, index) in realtimeResult.logs" :key="`rt-${index}`">{{ item }}</div>
        </div>
      </div>
    </div>

    <div v-else class="grid-two">
      <div class="page-card">
        <div class="section-toolbar">
          <div>
            <h3>离线烧录包准备</h3>
            <p>保留现有离线准备链路，并补上配置文件写入能力。</p>
          </div>
        </div>

        <el-form label-position="top" class="form-grid">
          <el-form-item label="供电电压">
            <el-select v-model="offlineForm.powerVoltage">
              <el-option v-for="item in POWER_OPTIONS" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="通讯类型">
            <el-select v-model="offlineForm.commType">
              <el-option v-for="item in OFFLINE_COMM_OPTIONS" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="配置波特率通讯类型">
            <el-select v-model="offlineForm.configCommType">
              <el-option
                v-for="item in OFFLINE_BAUD_COMM_OPTIONS"
                :key="item.value"
                :label="item.label"
                :value="item.value"
              />
            </el-select>
          </el-form-item>
          <el-form-item label="波特率编码">
            <el-select v-model="offlineForm.configBaudCode">
              <el-option
                v-for="item in offlineBaudOptions"
                :key="item.code"
                :label="item.label"
                :value="item.code"
              />
            </el-select>
          </el-form-item>
          <el-form-item label="帧类型">
            <el-select v-model="offlineForm.configFrameType">
              <el-option v-for="item in FRAME_TYPE_OPTIONS" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="UI 版本号">
            <el-input v-model="offlineForm.uiVersion" placeholder="可选，最长 32 字节" />
          </el-form-item>
        </el-form>

        <div class="upload-grid upload-grid--wide">
          <label
            v-for="item in uploadMetas"
            :key="`offline-${item.key}`"
            class="upload-card"
          >
            <div class="upload-card__head">
              <span>{{ item.label }}</span>
              <small>{{ item.tip }}</small>
            </div>
            <input :accept="item.accept" type="file" @change="onUploadChange($event, offlineFiles, item.key)" />
            <strong>{{ getUploadedName(offlineFiles[item.key]) }}</strong>
            <p v-if="item.key === 'config'" class="upload-card__summary">
              {{ getConfigSummary(offlineFiles.config) }}
            </p>
          </label>
        </div>

        <div class="actions">
          <el-button type="primary" :loading="offlineLoading" @click="runOfflinePrepare">同步离线烧录包</el-button>
        </div>
      </div>

      <div class="page-card page-card--log">
        <div class="section-toolbar section-toolbar--compact">
          <div>
            <h3>离线烧录日志</h3>
            <p>输出 0x16 / 0x17 / 0x35 / 0x36 / 0x31~0x34 / 0x14 的执行结果。</p>
          </div>
          <span class="metric-chip">
            <span class="metric-chip__dot" />
            {{ offlineResult.stage || "未开始" }}
          </span>
        </div>
        <el-progress :percentage="offlineResult.progress" :stroke-width="14" />
        <div ref="offlineLogRef" class="dark-box dark-box--fill">
          <div v-for="(item, index) in offlineResult.logs" :key="`offline-${index}`">{{ item }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ElMessage } from "element-plus"
import {
  fileToBytes,
  initRealtimeUpgrade,
  parseHexInput,
  performRealtimeUpgrade,
  prepareOfflineUpgrade,
  readAccessState,
  readFlags,
  readVersionSnapshot,
  switchLanguage,
  writeFlag,
  writeVersionInfo,
} from "@/api/unimaster"
import {
  FILE_KIND_OPTIONS,
  FRAME_TYPE_OPTIONS,
  OFFLINE_BAUD_COMM_OPTIONS,
  OFFLINE_COMM_OPTIONS,
  POWER_OPTIONS,
  PROTOCOL_OPTIONS,
  REALTIME_COMM_OPTIONS,
  VERSION_TYPE_OPTIONS,
  getBaudOptionsByCommType,
  getFileTypeCode,
} from "@/constants/unimaster"
import { useDeviceStore } from "@/store/device"
import { prepareMeterConfigUpgradeFile } from "@/utils/unimaster-config"

const deviceStore = useDeviceStore()

const tabs = [
  { label: "版本管理", value: "version" },
  { label: "实时升级", value: "realtime" },
  { label: "离线烧录", value: "offline" },
]

const uploadMetas = [
  { key: "boot", label: "BOOT 文件", accept: ".bin", tip: "离线与实时均支持 BIN" },
  { key: "app", label: "APP 文件", accept: ".hex,.bin", tip: "支持 HEX / BIN" },
  { key: "ui", label: "UI 文件", accept: ".txt,.bin", tip: "支持 TXT / BIN" },
  { key: "config", label: "配置文件", accept: ".json,.ini", tip: "会自动解析为 54 字节参数块" },
]

const activeTab = ref("version")

const snapshot = reactive({
  appVersion: "",
  uiVersion: "",
  versionItems: [],
  flags: [],
})

const versionLoading = ref(false)
const writeLoading = ref(false)
const flagLoading = ref(false)
const languageLoading = ref(false)
const accessLoading = ref(false)
const realtimeInitLoading = ref(false)
const realtimeLoading = ref(false)
const offlineLoading = ref(false)

const versionForm = reactive({
  code: 4,
  value: "",
})

const flagForm = reactive({
  position: 0,
  hexValue: "00 00 00 01",
  shutdownAfterWrite: false,
})

const realtimeForm = reactive({
  model: deviceStore.currentModel,
  commType: 0,
  baudCode: 0x0b,
  frameType: 0,
  powerVoltage: 0,
  vlk5vEnabled: true,
  protocolType: 0x01,
  burnFileType: 1,
})

const offlineForm = reactive({
  model: deviceStore.currentModel,
  powerVoltage: 0,
  commType: 0,
  configCommType: 1,
  configBaudCode: 0x0b,
  configFrameType: 0,
  uiVersion: "",
})

const realtimeFiles = reactive({
  boot: null,
  app: null,
  ui: null,
  config: null,
})

const offlineFiles = reactive({
  boot: null,
  app: null,
  ui: null,
  config: null,
})

const realtimeResult = reactive({
  progress: 0,
  stage: "",
  logs: [],
})

const offlineResult = reactive({
  progress: 0,
  stage: "",
  logs: [],
})

const realtimeLogRef = ref(null)
const offlineLogRef = ref(null)

const needsHexInput = computed(() => [5, 6].includes(versionForm.code))
const realtimeBaudOptions = computed(() => getBaudOptionsByCommType(realtimeForm.commType))
const offlineBaudOptions = computed(() => getBaudOptionsByCommType(offlineForm.configCommType, true))

watch(
  () => deviceStore.currentModel,
  (value) => {
    realtimeForm.model = value
    offlineForm.model = value
  },
  { immediate: true },
)

watch(
  () => realtimeForm.commType,
  () => {
    realtimeForm.baudCode = realtimeBaudOptions.value[0]?.code || 0x0b
    realtimeForm.frameType = realtimeForm.commType === 2 ? 1 : 0
  },
)

watch(
  () => offlineForm.configCommType,
  () => {
    offlineForm.configBaudCode = offlineBaudOptions.value[0]?.code || 0x0b
    offlineForm.configFrameType = offlineForm.configCommType === 3 ? 1 : 0
  },
)

watch(
  () => realtimeResult.logs.length,
  async () => {
    await nextTick()
    scrollLogToBottom(realtimeLogRef.value)
  },
)

watch(
  () => offlineResult.logs.length,
  async () => {
    await nextTick()
    scrollLogToBottom(offlineLogRef.value)
  },
)

onMounted(() => {
  loadSnapshot()
})

async function loadSnapshot() {
  versionLoading.value = true
  try {
    const data = await readVersionSnapshot()
    Object.assign(snapshot, data)
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    versionLoading.value = false
  }
}

async function refreshFlags() {
  try {
    snapshot.flags = await readFlags()
  } catch (error) {
    ElMessage.error(String(error))
  }
}

async function submitVersion() {
  writeLoading.value = true
  try {
    const request = needsHexInput.value
      ? { code: versionForm.code, valueHex: parseHexInput(versionForm.value) }
      : { code: versionForm.code, valueText: versionForm.value }
    const result = await writeVersionInfo(request)
    ElMessage[result.success ? "success" : "warning"](result.message)
    await loadSnapshot()
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    writeLoading.value = false
  }
}

async function changeLanguage(language) {
  languageLoading.value = true
  try {
    const result = await switchLanguage(language)
    ElMessage[result.success ? "success" : "warning"](result.message)
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    languageLoading.value = false
  }
}

async function submitFlag() {
  flagLoading.value = true
  try {
    const bytes = parseHexInput(flagForm.hexValue)
    if (bytes.length !== 4) {
      throw new Error("标志位必须是 4 个字节")
    }
    const result = await writeFlag({
      position: flagForm.position,
      value: ((bytes[0] << 24) | (bytes[1] << 16) | (bytes[2] << 8) | bytes[3]) >>> 0,
      shutdownAfterWrite: flagForm.shutdownAfterWrite,
    })
    ElMessage[result.success ? "success" : "warning"](result.message)
    await refreshFlags()
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    flagLoading.value = false
  }
}

async function checkAccess() {
  accessLoading.value = true
  try {
    const result = await readAccessState()
    ElMessage[result.success ? "success" : "warning"](result.message)
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    accessLoading.value = false
  }
}

async function initRealtimeOnly() {
  realtimeInitLoading.value = true
  try {
    const burnType = getFirstSelectedKind(realtimeFiles)
    realtimeForm.burnFileType = getRealtimeBurnType(burnType)
    const result = await initRealtimeUpgrade({ ...realtimeForm })
    ElMessage[result.success ? "success" : "warning"](result.message)
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    realtimeInitLoading.value = false
  }
}

async function runRealtimeUpgrade() {
  realtimeLoading.value = true
  Object.assign(realtimeResult, {
    progress: 0,
    stage: "准备中",
    logs: [],
  })

  try {
    const files = await collectFiles(realtimeFiles)
    if (files.length === 0) {
      throw new Error("请至少选择一个实时升级文件")
    }

    for (const [index, file] of files.entries()) {
      appendUpgradeLog(realtimeResult, `开始处理 ${file.fileName}`)
      const result = await performRealtimeUpgrade({
        init: {
          ...realtimeForm,
          burnFileType: getRealtimeBurnType(file.kind),
        },
        files: [file],
      })

      realtimeResult.logs.push(...result.logs)
      realtimeResult.progress = Math.round(((index + 1) / files.length) * 100)
      realtimeResult.stage = result.stage

      if (!result.success) {
        throw new Error(result.stage)
      }
    }

    realtimeResult.stage = "实时升级完成"
    ElMessage.success("实时升级完成")
  } catch (error) {
    realtimeResult.stage = "实时升级失败"
    ElMessage.error(String(error))
  } finally {
    realtimeLoading.value = false
  }
}

async function runOfflinePrepare() {
  offlineLoading.value = true
  Object.assign(offlineResult, {
    progress: 0,
    stage: "准备中",
    logs: [],
  })

  try {
    const files = await collectFiles(offlineFiles)
    if (files.length === 0) {
      throw new Error("请至少选择一个离线烧录文件")
    }

    const result = await prepareOfflineUpgrade({
      ...offlineForm,
      bootFileType: getResolvedFileType(offlineFiles.boot),
      appFileType: getResolvedFileType(offlineFiles.app),
      uiFileType: getResolvedFileType(offlineFiles.ui),
      configFileType: getResolvedFileType(offlineFiles.config),
      files,
    })
    Object.assign(offlineResult, result)
    ElMessage[result.success ? "success" : "warning"](result.stage)
  } catch (error) {
    offlineResult.stage = "离线烧录失败"
    ElMessage.error(String(error))
  } finally {
    offlineLoading.value = false
  }
}

async function collectFiles(container) {
  const result = []

  for (const item of FILE_KIND_OPTIONS) {
    const current = container[item.value]
    if (!current) {
      continue
    }

    if (item.value === "config") {
      result.push({
        kind: "config",
        fileName: current.fileName,
        data: current.data,
      })
      continue
    }

    result.push({
      kind: item.value,
      fileName: current.name,
      data: await fileToBytes(current),
    })
  }

  return result
}

async function onUploadChange(event, container, key) {
  const [file] = event.target.files || []
  if (!file) {
    container[key] = null
    return
  }

  try {
    if (key === "config") {
      container.config = await prepareMeterConfigUpgradeFile(file)
      ElMessage.success("配置文件解析成功")
    } else {
      container[key] = file
    }
  } catch (error) {
    container[key] = null
    ElMessage.error(String(error))
  } finally {
    event.target.value = ""
  }
}

function getUploadedName(entry) {
  if (!entry) {
    return "未选择"
  }
  return entry.fileName || entry.name || "未命名文件"
}

function getConfigSummary(entry) {
  if (!entry) {
    return "支持 JSON / INI，导入后会编码为仪表参数块。"
  }
  return `已解析 ${entry.data.length} 字节，可直接参与烧录。`
}

function getResolvedFileType(entry) {
  if (!entry) {
    return 0xff
  }
  return getFileTypeCode(entry.fileName || entry.name || "")
}

function getFirstSelectedKind(container) {
  return FILE_KIND_OPTIONS.find((item) => container[item.value])?.value || "app"
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

function appendUpgradeLog(target, message) {
  target.logs.push(message)
}

function scrollLogToBottom(element) {
  if (!element) {
    return
  }
  element.scrollTop = element.scrollHeight
}
</script>

<style scoped lang="scss">
.software-page {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
  min-height: 0;
}

.software-head,
.page-card {
  background: #fff;
  border: 1px solid #dce5f0;
  border-radius: var(--dt-radius-panel);
  box-shadow: var(--dt-shadow-panel);
}

.software-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 18px 20px;
}

.software-head h2,
.page-card h3,
.page-card h4 {
  margin: 0 0 8px;
  color: #1e3354;
}

.software-head h2 {
  font-size: 18px;
}

.software-head p,
.section-toolbar p,
.upload-card small {
  margin: 0;
  color: #6f7c94;
  font-size: 14px;
  line-height: 1.65;
}

.software-tabbar {
  display: flex;
  gap: 10px;
  padding: 2px 2px 0;
}

.software-tab {
  min-height: 38px;
  padding: 0 16px;
  border: 1px solid #d8e2ee;
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.82);
  color: #55657d;
  font: inherit;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
}

.software-tab--active {
  border-color: #b8d2f8;
  background: #eaf3ff;
  color: #1668dc;
}

.grid-two {
  display: grid;
  grid-template-columns: 1.1fr 0.9fr;
  gap: 16px;
  min-height: 0;
  flex: 1;
}

.page-card {
  display: flex;
  flex-direction: column;
  min-height: 0;
  padding: 20px;
  overflow: auto;
}

.page-card--log {
  overflow: hidden;
}

.section-toolbar {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 16px;
}

.section-toolbar--compact {
  margin-bottom: 12px;
}

.section-toolbar h3 {
  font-size: 16px;
}

.version-summary {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
  margin-bottom: 18px;
}

.summary-item {
  padding: 14px 16px;
  border: 1px solid #e2e9f3;
  border-radius: 12px;
  background: #f6f9fd;
}

.summary-item span {
  display: block;
  margin-bottom: 6px;
  color: #7d8aa0;
  font-size: 13px;
}

.summary-item strong {
  color: #1d3559;
  font-size: 18px;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
  margin-top: 16px;
}

.flag-panel {
  margin-top: 20px;
  padding-top: 18px;
  border-top: 1px solid #e7edf5;
}

.flag-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.flag-write {
  display: grid;
  grid-template-columns: 110px minmax(220px, 1fr) 140px auto;
  gap: 10px;
  align-items: end;
  margin-top: 14px;
}

.flag-write__field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-width: 0;
}

.flag-write__field--switch {
  align-items: flex-start;
}

.flag-write__label {
  color: #6d7b91;
  font-size: 12px;
  font-weight: 700;
  line-height: 1.4;
}

.flag-write__actions {
  display: flex;
  align-items: end;
}

.flag-write :deep(.el-input-number) {
  width: 110px;
}

.flag-write :deep(.el-input) {
  min-width: 0;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0 16px;
}

.upload-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 12px;
  margin-top: 10px;
}

.upload-grid--wide {
  grid-template-columns: repeat(2, minmax(0, 1fr));
}

.upload-card {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 14px;
  border: 1px dashed #cdd7e6;
  border-radius: 12px;
  background: #fbfcfe;
}

.upload-card input {
  width: 100%;
}

.upload-card__head {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.upload-card__head span {
  font-weight: 700;
  color: #20385a;
}

.upload-card strong {
  color: #1d3559;
  font-size: 13px;
  line-height: 1.6;
  word-break: break-all;
}

.upload-card__summary {
  margin: 0;
  color: #5f6f88;
  font-size: 12px;
  line-height: 1.6;
}

.dark-box {
  min-height: 320px;
  margin-top: 12px;
  padding: 16px;
  border-radius: 12px;
  background: linear-gradient(180deg, #0f1828, #162235);
  border: 1px solid rgba(99, 126, 175, 0.24);
  color: #d9ebff;
  line-height: 1.9;
  white-space: pre-wrap;
  overflow: auto;
}

.dark-box--fill {
  flex: 1;
  min-height: 0;
}

@media (max-width: 1200px) {
  .grid-two {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 960px) {
  .software-head,
  .section-toolbar {
    flex-direction: column;
  }

  .form-grid,
  .upload-grid,
  .upload-grid--wide {
    grid-template-columns: 1fr;
  }

  .flag-write {
    grid-template-columns: 1fr;
    align-items: stretch;
  }

  .page-card {
    overflow: visible;
  }
}
</style>
