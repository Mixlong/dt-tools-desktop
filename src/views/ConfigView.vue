<template>
  <div class="config-page">
    <section class="page-card toolbar-panel">
      <div class="toolbar-panel__group">
        <span class="toolbar-panel__label">串口</span>
        <div class="transport-toolbar">
          <el-segmented v-model="transportForm.commType" :options="transportTypeOptions" />
          <el-select v-model="transportForm.baudCode" class="transport-toolbar__select">
            <el-option
              v-for="option in transportBaudOptions"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
          <el-select v-if="transportForm.commType === 0x02" v-model="transportForm.frameType" class="transport-toolbar__select">
            <el-option
              v-for="option in METER_CAN_FRAME_OPTIONS"
              :key="option.value"
              :label="option.label"
              :value="option.value"
            />
          </el-select>
          <el-button type="primary" :loading="loading.init" @click="initTransport">初始化配置链路</el-button>
        </div>
        <div class="actions">
          <el-button :loading="loading.read" @click="handleReadConfig">读取配置</el-button>
          <el-button :loading="loading.write" @click="handleWriteConfig">写入配置</el-button>
          <el-button @click="openImportDialog">导入配置</el-button>
          <el-button @click="handleExportConfig">导出配置</el-button>
        </div>
      </div>

      <div class="toolbar-panel__group toolbar-panel__group--transport">
        <span class="metric-chip">
          <span :class="['metric-chip__dot', { 'metric-chip__dot--idle': !linkReady }]" />
          {{ linkReady ? "配置链路已初始化" : "未初始化配置链路" }}
        </span>
        <span class="metric-chip">
          <span :class="['metric-chip__dot', { 'metric-chip__dot--idle': deviceStore.connectionStatus !== 'CONNECTED' }]" />
          {{ deviceStore.connectionStatus === "CONNECTED" ? deviceStore.onlineStatus : "串口未连接" }}
        </span>
      </div>
    </section>

    <div class="config-grid">
      <section class="panel panel--main">
        <div class="panel-header panel-header--between">
          <div>
            <h3>参数编辑区</h3>
            <p>字段按旧项目能力拆成多个功能块，写入时统一编码为 54 字节仪表配置。</p>
          </div>
        </div>

        <input
          ref="importInputRef"
          class="hidden-input"
          type="file"
          accept=".json,.ini"
          @change="handleImportFile"
        />

        <div class="panel-body">
          <section v-for="group in METER_CONFIG_GROUPS" :key="group.key" class="section-block">
            <div class="section-block__header">
              <div>
                <h4>{{ group.title }}</h4>
                <p>{{ group.description }}</p>
              </div>
            </div>

            <div class="form-grid">
              <div v-for="field in group.fields" :key="field.key" class="field-item">
                <label class="field-label">{{ field.label }}</label>

                <template v-if="field.type === 'select'">
                  <div :class="['field-control', { 'field-control--with-unit': field.unit }]">
                    <el-select v-model="form[field.key]" @change="handleFieldChange(field.key)">
                      <el-option
                        v-for="option in field.options"
                        :key="`${field.key}-${option.value}`"
                        :label="option.label"
                        :value="option.value"
                      />
                    </el-select>
                    <span v-if="field.unit" class="field-control__append">{{ field.unit }}</span>
                  </div>
                </template>

                <template v-else-if="field.type === 'number'">
                  <div :class="['field-control', { 'field-control--with-unit': field.unit }]">
                    <el-input-number
                      v-model="form[field.key]"
                      :controls="false"
                      :min="field.min ?? 0"
                      :max="field.max"
                      :precision="field.precision"
                      :step="field.step ?? 1"
                    />
                    <span v-if="field.unit" class="field-control__append">{{ field.unit }}</span>
                  </div>
                </template>

                <template v-else>
                  <el-input
                    v-model="form[field.key]"
                    :maxlength="field.maxlength"
                    @change="handleFieldChange(field.key)"
                  />
                </template>
              </div>
            </div>
          </section>
        </div>
      </section>

      <aside class="side-stack">
        <section class="panel">
          <div class="panel-header">
            <div>
              <h3>当前摘要</h3>
            </div>
          </div>
          <div class="panel-body">
            <ul class="summary-list">
              <li>
                <span>当前机型</span>
                <strong>{{ deviceStore.currentModelLabel }}</strong>
              </li>
              <li>
                <span>最近动作</span>
                <strong>{{ lastAction }}</strong>
              </li>
              <li>
                <span>导入来源</span>
                <strong>{{ importedFileName || "未导入" }}</strong>
              </li>
              <li>
                <span>轮径 / 周长</span>
                <strong>{{ wheelSummary }}</strong>
              </li>
            </ul>
          </div>
        </section>

        <section class="panel">
          <div class="panel-header">
            <div>
              <h3>操作提示</h3>
            </div>
          </div>
          <div class="panel-body">
            <ol class="guide-list">
              <li>先在左侧侧边栏连接适配器，再执行“初始化配置链路”。</li>
              <li>读取成功后再修改字段，避免把默认值直接覆盖到现场设备。</li>
              <li>INI 导入会按旧项目字段映射转换；JSON 支持直接导入 `instrumentModel`。</li>
            </ol>
          </div>
        </section>
      </aside>
    </div>
  </div>
</template>

<script setup>
import { save } from "@tauri-apps/plugin-dialog"
import { ElMessage } from "element-plus"
import { readMeterConfig, saveTextFile, setMeterConfigTransport, writeMeterConfig } from "@/api/unimaster"
import { useDeviceStore } from "@/store/device"
import {
  METER_CAN_BAUD_OPTIONS,
  METER_CAN_FRAME_OPTIONS,
  METER_CONFIG_DEFAULTS,
  METER_CONFIG_GROUPS,
  METER_TRANSPORT_OPTIONS,
  METER_UART_BAUD_OPTIONS,
  decodeMeterConfig,
  encodeMeterConfig,
  estimatePerimeterByWheel,
  getDefaultUndervoltage,
  getWheelDiameterLabel,
  parseMeterConfigFile,
  validateMeterConfig,
} from "@/utils/unimaster-config"

const deviceStore = useDeviceStore()

const importInputRef = ref(null)
const linkReady = ref(false)
const importedFileName = ref("")
const lastAction = ref("未执行")

const loading = reactive({
  init: false,
  read: false,
  write: false,
})

const form = reactive({
  ...METER_CONFIG_DEFAULTS,
})

const transportForm = reactive({
  commType: 0x01,
  baudCode: 0x04,
  frameType: 0,
})

const transportTypeOptions = METER_TRANSPORT_OPTIONS.map((item) => ({
  label: item.label,
  value: item.value,
}))

const transportBaudOptions = computed(() =>
  transportForm.commType === 0x02 ? METER_CAN_BAUD_OPTIONS : METER_UART_BAUD_OPTIONS,
)

const wheelSummary = computed(() => `${getWheelDiameterLabel(form.wheelDiameter)} / ${form.perimeter} mm`)

function getTransportErrorMessage(error) {
  const message = String(error)
  if (!message.includes("等待命令 0x37 响应超时")) {
    return message
  }

  const transportLabel = transportForm.commType === 0x02 ? "CAN" : "UART"
  const baudLabel = transportBaudOptions.value.find((item) => item.value === transportForm.baudCode)?.label ?? transportForm.baudCode
  return `配置链路初始化超时。请确认适配器连接使用固定 115200，当前页面选择的是 ${transportLabel} / ${baudLabel}，并检查仪表是否上电、TX/RX/GND 接线是否正确。`
}

watch(
  () => transportForm.commType,
  (value) => {
    linkReady.value = false
    if (value === 0x02) {
      transportForm.baudCode = METER_CAN_BAUD_OPTIONS[0]?.value ?? 0x08
      transportForm.frameType = 0x01
      return
    }

    transportForm.baudCode = METER_UART_BAUD_OPTIONS[3]?.value ?? 0x04
    transportForm.frameType = 0
  },
)

watch(
  () => transportForm.baudCode,
  () => {
    linkReady.value = false
  },
)

watch(
  () => transportForm.frameType,
  () => {
    linkReady.value = false
  },
)

watch(
  () => deviceStore.connectionStatus,
  (status) => {
    if (status !== "CONNECTED") {
      linkReady.value = false
      if (lastAction.value.startsWith("串口连接后自动初始化")) {
        lastAction.value = "未执行"
      }
      return
    }

    if (!linkReady.value) {
      lastAction.value = "适配器已连接，等待初始化配置链路"
    }
  },
  { immediate: true },
)

async function initTransport(options = {}) {
  const { silent = false, trigger = "手动初始化" } = options

  if (deviceStore.connectionStatus !== "CONNECTED") {
    if (!silent) {
      ElMessage.warning("请先连接串口适配器")
    }
    return false
  }

  if (loading.init) {
    return linkReady.value
  }

  loading.init = true
  try {
    const result = await setMeterConfigTransport({
      commType: transportForm.commType,
      baudCode: transportForm.baudCode,
      frameType: transportForm.commType === 0x02 ? transportForm.frameType : 0,
    })

    linkReady.value = result.success
    lastAction.value = `${trigger}: ${result.message}`
    if (!silent || !result.success) {
      ElMessage[result.success ? "success" : "warning"](result.message)
    }
    return result.success
  } catch (error) {
    linkReady.value = false
    lastAction.value = `${trigger}: 初始化失败`
    if (!silent) {
      ElMessage.error(getTransportErrorMessage(error))
    }
    return false
  } finally {
    loading.init = false
  }
}

async function ensureTransportReady(trigger) {
  if (linkReady.value) {
    return true
  }

  return initTransport({ silent: false, trigger })
}

async function handleReadConfig() {
  if (!(await ensureTransportReady("读取前自动初始化"))) {
    return
  }

  loading.read = true
  try {
    const result = await readMeterConfig()
    Object.assign(form, decodeMeterConfig(result.bytes))
    lastAction.value = "读取配置成功"
    ElMessage.success("读取配置成功")
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    loading.read = false
  }
}

async function handleWriteConfig() {
  if (!validateMeterConfig(form)) {
    ElMessage.warning("仪表配置参数均为必填项")
    return
  }

  if (!(await ensureTransportReady("写入前自动初始化"))) {
    return
  }

  loading.write = true
  try {
    const result = await writeMeterConfig(encodeMeterConfig(form))
    lastAction.value = result.message
    ElMessage[result.success ? "success" : "warning"](result.message)
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    loading.write = false
  }
}

function openImportDialog() {
  importInputRef.value?.click()
}

async function handleImportFile(event) {
  const [file] = event.target.files || []
  if (!file) {
    return
  }

  try {
    const imported = await parseMeterConfigFile(file)
    Object.assign(form, imported)
    importedFileName.value = file.name
    lastAction.value = `已导入 ${file.name}`
    ElMessage.success("导入配置成功")
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    event.target.value = ""
  }
}

async function handleExportConfig() {
  if (!validateMeterConfig(form)) {
    ElMessage.warning("仪表配置参数均为必填项")
    return
  }

  try {
    const path = await save({
      title: "导出配置",
      defaultPath: `配置文件-${Date.now()}.json`,
      filters: [{ name: "JSON", extensions: ["json"] }],
    })

    if (!path || Array.isArray(path)) {
      return
    }

    await saveTextFile(path, JSON.stringify(form, null, 2))
    lastAction.value = `配置已导出到 ${path.split(/[\\\\/]/).pop()}`
    ElMessage.success("导出配置成功")
  } catch (error) {
    ElMessage.error(String(error))
  }
}

function handleFieldChange(fieldKey) {
  if (fieldKey === "voltage") {
    form.undervoltage = getDefaultUndervoltage(form.voltage)
  }

  if (fieldKey === "wheelDiameter") {
    form.perimeter = estimatePerimeterByWheel(form.wheelDiameter)
  }

  if (fieldKey === "carModel") {
    form.carModel = String(form.carModel || "").slice(0, 2).toUpperCase()
  }
}
</script>

<style scoped lang="scss">
.config-page {
  display: flex;
  flex-direction: column;
  gap: 10px;
  height: 100%;
  min-height: 0;
}

.toolbar-panel {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  padding: 10px 14px;
}

.toolbar-panel__group {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

.toolbar-panel__group--transport {
  margin-left: auto;
  flex-wrap: wrap;
  justify-content: flex-end;
}

.toolbar-panel__label {
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 700;
  white-space: nowrap;
}

.transport-toolbar {
  display: flex;
  align-items: center;
  gap: 8px;
  flex-wrap: wrap;
}

.transport-toolbar__select {
  width: 148px;
}

.config-grid {
  display: grid;
  grid-template-columns: minmax(0, 1.52fr) minmax(300px, 0.48fr);
  gap: 12px;
  flex: 1;
  min-height: 0;
}

.panel {
  background: #ffffff;
  border: 1px solid var(--dt-border);
  border-radius: var(--dt-radius-panel);
  box-shadow: var(--dt-shadow-panel);
  padding: 16px;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.panel--main {
  padding: 18px;
}

.panel-header {
  margin-bottom: 12px;
  flex: 0 0 auto;
}

.panel-header--between {
  display: flex;
  justify-content: space-between;
  gap: 16px;
  align-items: flex-start;
}

.panel-header h3,
.section-block h4 {
  margin: 0;
  color: var(--dt-text-primary);
}

.panel-header p,
.section-block__header p {
  margin: 6px 0 0;
  color: var(--dt-text-secondary);
  font-size: 13px;
  line-height: 1.6;
}

.panel-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
  padding-right: 2px;
}

.side-stack {
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-height: 0;
}

.section-block {
  padding: 12px;
  border: 1px solid var(--dt-border);
  border-radius: 12px;
  background: var(--dt-bg-panel-soft);
}

.section-block + .section-block {
  margin-top: 10px;
}

.section-block__header {
  margin-bottom: 10px;
}

.section-block__header h4 {
  font-size: 15px;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 10px 12px;
}

.field-item {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-label {
  color: #4a5b75;
  font-size: 12px;
  font-weight: 600;
  line-height: 1.35;
}

.field-control {
  display: flex;
  align-items: center;
  width: 100%;
}

.field-control :deep(.el-input-number),
.field-control :deep(.el-select) {
  width: 100%;
}

.field-control--with-unit {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: stretch;
  border: 1px solid var(--dt-border);
  border-radius: 8px;
  overflow: hidden;
  background: #fff;
}

.field-control--with-unit :deep(.el-input-number .el-input__wrapper),
.field-control--with-unit :deep(.el-select__wrapper) {
  border: 0;
  border-radius: 0;
  box-shadow: none;
  background: transparent;
}

.field-control__append {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  align-self: stretch;
  min-width: 48px;
  padding: 0 12px;
  border-left: 1px solid var(--dt-border);
  border-radius: 0;
  background: #f5f8fc;
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
}

.form-grid :deep(.el-input__inner),
.form-grid :deep(.el-select__selected-item),
.form-grid :deep(.el-input-number .el-input__inner),
.form-grid :deep(.el-input-number__decrease),
.form-grid :deep(.el-input-number__increase),
.form-grid :deep(.el-input__wrapper),
.form-grid :deep(.el-select__wrapper) {
  text-align: left;
  justify-content: flex-start;
}

.actions {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.summary-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.summary-list li {
  padding: 12px 14px;
  border: 1px solid var(--dt-border);
  border-radius: 10px;
  background: var(--dt-bg-panel-soft);
}

.summary-list span {
  display: block;
  margin-bottom: 4px;
  color: #748399;
  font-size: 12px;
}

.summary-list strong {
  color: var(--dt-text-primary);
  font-size: 14px;
  font-weight: 600;
  line-height: 1.4;
}

.guide-list {
  margin: 0;
  padding-left: 20px;
  color: var(--dt-text-secondary);
}

.guide-list li {
  font-size: 13px;
  line-height: 1.7;
}

.guide-list li + li {
  margin-top: 8px;
}

.hidden-input {
  display: none;
}

.config-page .page-intro {
  padding: 16px 18px;
  margin-bottom: 0;
}

.config-page .page-intro h2 {
  font-size: 22px;
}

.config-page .page-intro p {
  margin-top: 8px;
  font-size: 13px;
  line-height: 1.6;
}

.config-page :deep(.el-form-item) {
  margin-bottom: 0;
}

.config-page :deep(.el-form-item__label) {
  padding-bottom: 4px;
  color: #4a5b75;
  font-size: 12px;
  font-weight: 600;
}

.config-page :deep(.el-button) {
  min-height: 34px;
  padding: 0 14px;
}

.config-page :deep(.el-input__wrapper),
.config-page :deep(.el-select__wrapper),
.config-page :deep(.el-textarea__inner),
.config-page :deep(.el-input-number) {
  min-height: 34px;
}

.metric-chip__dot--idle {
  background: var(--el-color-info);
}

@media (max-width: 1500px) {
  .form-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 1200px) {
  .toolbar-panel {
    flex-direction: column;
    align-items: flex-start;
  }

  .toolbar-panel__group {
    flex-wrap: wrap;
  }

  .toolbar-panel__group--transport {
    margin-left: 0;
    justify-content: flex-start;
  }

  .config-grid {
    grid-template-columns: 1fr;
  }

  .panel,
  .panel--main {
    height: auto;
  }

  .panel-body {
    overflow: visible;
    padding-right: 0;
  }
}

@media (max-width: 720px) {
  .panel-header--between {
    flex-direction: column;
    align-items: flex-start;
  }

  .form-grid {
    grid-template-columns: 1fr;
  }
}
</style>
