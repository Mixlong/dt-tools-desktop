<template>
  <div :class="['config-page', `config-page--${surfaceMode}`]">
    <input
      ref="importInputRef"
      class="hidden-input"
      type="file"
      accept=".json,.ini"
      @change="handleImportFile"
    />

    <div :class="['workspace-grid', { 'workspace-grid--empty': !canDisplayConfig }]">
      <section :class="['panel', 'panel--main', { 'panel--main--empty': !canDisplayConfig }]">
        <div :class="['panel-body', { 'panel-body--empty': !canDisplayConfig }]">
          <section class="panel toolbar-panel toolbar-panel--content">
            <div class="toolbar-panel__group toolbar-panel__group--status">
              <span :class="['header-status-chip', `header-status-chip--${toolbarStatusTone}`]">
                <span :class="['header-status-chip__dot', `header-status-chip__dot--${toolbarStatusTone}`]" />
                {{ toolbarStatusLabel }}
              </span>
            </div>

            <div class="toolbar-panel__group toolbar-panel__group--actions">
              <q-btn class="toolbar-action toolbar-action--read" push color="teal" :loading="loading.read" :disable="!isDeviceConnected || !hasConnectionConfig || loading.init || loading.write" :label="t('config.toolbar.read')" @click="handleReadConfig" />
              <q-btn class="toolbar-action toolbar-action--write" push color="deep-orange" :loading="loading.write" :disable="!isDeviceConnected || !hasLoadedConfig || loading.init || loading.read" :label="t('config.toolbar.write')" @click="handleWriteConfig" />
              <q-btn class="toolbar-action toolbar-action--soft" push color="primary" :label="t('config.toolbar.import')" @click="openImportDialog" />
              <q-btn class="toolbar-action toolbar-action--soft" push color="primary" :disable="!hasLoadedConfig" :label="t('config.toolbar.export')" @click="handleExportConfig" />
            </div>
          </section>

          <template v-if="canDisplayConfig">
            <q-scroll-area class="panel-body__scroll">
              <div class="panel-body__content">
                <section
                  v-for="group in localizedGroups"
                  :key="group.key"
                  :ref="(el) => setSectionRef(group.key, el)"
                  :class="['section-block', `section-block--${group.key}`]"
                >
                  <div class="section-block__header">
                    <div class="section-block__tag">
                      <h4>{{ group.title }}</h4>
                    </div>
                    <div class="section-block__line" />
                  </div>

                  <div class="form-grid">
                    <div v-for="field in group.fields" :key="field.key" class="field-item">
                      <label class="field-label">{{ field.label }}</label>

                      <template v-if="field.type === 'select'">
                        <q-select
                          v-model="form[field.key]"
                          :options="field.options"
                          emit-value
                          map-options
                          outlined
                          dense
                          option-label="label"
                          option-value="value"
                          @update:model-value="handleFieldChange(field.key)"
                        >
                          <template v-if="field.unit" #append>
                            <span class="dt-field__suffix">{{ field.unit }}</span>
                          </template>
                        </q-select>
                      </template>

                      <template v-else-if="field.type === 'number'">
                        <DtNumberField
                          v-model="form[field.key]"
                          :min="field.min ?? 0"
                          :max="field.max"
                          :precision="field.precision"
                          :step="field.step ?? 1"
                          :suffix="field.unit"
                          @update:model-value="handleFieldChange(field.key)"
                        />
                      </template>

                      <template v-else>
                        <q-input
                          v-model="form[field.key]"
                          outlined
                          dense
                          :maxlength="field.maxlength"
                          input-class="dt-field__input dt-field__input--left"
                          @update:model-value="handleFieldChange(field.key)"
                        />
                      </template>
                    </div>
                  </div>
                </section>
              </div>
            </q-scroll-area>
          </template>

          <template v-else>
            <section :class="['config-empty', `config-empty--${emptyState.tone}`]">
              <div class="config-empty__hero">
                <div class="config-empty__visual" aria-hidden="true">
                  <span class="config-empty__halo config-empty__halo--outer" />
                  <span class="config-empty__halo config-empty__halo--inner" />
                  <span class="config-empty__spark config-empty__spark--top" />
                  <span class="config-empty__spark config-empty__spark--right" />
                  <span class="config-empty__spark config-empty__spark--bottom" />
                  <div class="config-empty__device-shell">
                    <div class="config-empty__device-core">
                      <q-icon :name="emptyState.icon" size="40px" />
                    </div>
                  </div>
                  <span class="config-empty__endpoint config-empty__endpoint--left">{{ transportLabel }}</span>
                  <span class="config-empty__endpoint config-empty__endpoint--right">{{ transportBaudLabel }}</span>
                </div>

                <div class="config-empty__content">
                  <h3>{{ emptyState.title }}</h3>
                  <p class="config-empty__description">{{ emptyState.description }}</p>
                </div>
              </div>
            </section>
          </template>
        </div>
      </section>

    </div>
  </div>
</template>

<script setup>
import { useQuasar } from "quasar"
import { useI18n } from "vue-i18n"
import { save } from "@tauri-apps/plugin-dialog"
import DtNumberField from "@/components/DtNumberField.vue"
import { notifyError, notifyInfo, notifySuccess } from "@/services/ui"
import { readMeterConfig, saveTextFile, sendMeterConfigHeartbeat, setMeterConfigTransport, writeMeterConfig } from "@/api/unimaster"
import { useDeviceStore } from "@/store/device"
import { useMeterConfigStore } from "@/store/meterConfig"
import {
  METER_CAN_BAUD_OPTIONS,
  METER_CONFIG_GROUPS,
  METER_UART_BAUD_OPTIONS,
  decodeMeterConfig,
  encodeMeterConfig,
  estimatePerimeterByWheel,
  getDefaultUndervoltage,
  parseMeterConfigFile,
  validateMeterConfig,
} from "@/utils/unimaster-config"

const deviceStore = useDeviceStore()
const meterConfigStore = useMeterConfigStore()
const $q = useQuasar()
const { t } = useI18n()
const openAdapterDialog = inject("openAdapterDialog", null)

const importInputRef = ref(null)
const linkReady = computed({
  get: () => deviceStore.meterLinkReady,
  set: (value) => {
    deviceStore.meterLinkReady = value
  },
})
const importedFileName = computed({
  get: () => meterConfigStore.importedFileName,
  set: (value) => {
    meterConfigStore.importedFileName = value
  },
})
const lastAction = computed({
  get: () => meterConfigStore.lastAction,
  set: (value) => {
    meterConfigStore.lastAction = value
  },
})
const hasLoadedConfig = computed({
  get: () => meterConfigStore.hasLoadedConfig,
  set: (value) => {
    meterConfigStore.hasLoadedConfig = value
  },
})
const readStatus = computed({
  get: () => meterConfigStore.readStatus,
  set: (value) => {
    meterConfigStore.readStatus = value
  },
})
const surfaceMode = computed(() => "ledger")

const loading = reactive({
  init: false,
  read: false,
  write: false,
})
let canHeartbeatTimer = null
let heartbeatInFlight = false

const form = meterConfigStore.form

const transportForm = reactive({
  get commType() {
    return deviceStore.meterCommType
  },
  set commType(value) {
    deviceStore.setMeterCommType(value)
  },
  get baudCode() {
    return deviceStore.meterBaudCode
  },
  set baudCode(value) {
    deviceStore.setMeterBaudCode(value)
  },
  get frameType() {
    return deviceStore.meterFrameType
  },
  set frameType(value) {
    deviceStore.setMeterFrameType(value)
  },
})

const transportBaudOptions = computed(() =>
  transportForm.commType === 0x02 ? METER_CAN_BAUD_OPTIONS : METER_UART_BAUD_OPTIONS,
)

function localizeFieldOptions(field) {
  if (!Array.isArray(field.options)) {
    return field.options
  }

  switch (field.key) {
    case "backlightBrightness":
      return field.options.map((option) => ({
        ...option,
        label: option.value === 6 ? t("config.options.backlightBrightness.auto") : option.label,
      }))
    case "assist":
      return field.options.map((option) => ({
        ...option,
        label: t(`config.options.assistDirection.${option.value}`),
      }))
    case "rotateHandle":
    case "bluetooth":
    case "driveAssist":
      return field.options.map((option) => ({
        ...option,
        label: t(`config.options.booleanYesNo.${option.value}`),
      }))
    case "turnOnPasswd":
    case "menuPassword":
    case "factoryReset":
    case "cruise":
    case "buzzerSwitch":
      return field.options.map((option) => ({
        ...option,
        label: t(`config.options.booleanReversed.${option.value}`),
      }))
    case "rotateHandleSpeedLimit":
      return field.options.map((option) => ({
        ...option,
        label: t(`config.options.rotateHandleSpeedLimit.${option.value}`),
      }))
    case "agreement":
      return field.options.map((option) => ({
        ...option,
        label: t(`config.options.agreement.${option.value}`),
      }))
    case "power":
      return field.options.map((option) => ({
        ...option,
        label: t(`config.options.power.${option.value}`),
      }))
    case "logo":
      return field.options.map((option) => ({
        ...option,
        label: t(`config.options.logo.${option.value}`),
      }))
    case "ebikeName":
      return field.options
    case "serialLevel":
      return field.options.map((option) => ({
        ...option,
        label: t(`config.options.serialLevel.${option.value}`),
      }))
    default:
      return field.options
  }
}

const localizedGroups = computed(() =>
  METER_CONFIG_GROUPS.map((group) => ({
    ...group,
    title: t(`config.groups.${group.key}`),
    fields: group.fields.map((field) => ({
      ...field,
      label: t(`config.fields.${field.key}`),
      options: localizeFieldOptions(field),
    })),
  })),
)

const sectionRefs = new Map()
const hasReadableConfig = computed(() =>
  hasLoadedConfig.value
  || importedFileName.value !== ""
  || lastAction.value === t("config.actions.readSuccess")
  || lastAction.value.startsWith(t("config.actions.imported", { name: "" }).trim()),
)
const canDisplayConfig = computed(() => hasReadableConfig.value)
const isDeviceConnected = computed(() => deviceStore.connectionStatus === "CONNECTED")
const hasConnectionConfig = computed(() => Boolean(String(deviceStore.upgradeCqCode || "").trim()))
const transportLabel = computed(() => (
  transportForm.commType === 0x02 ? t("layout.device.transportCan") : t("layout.device.transportUart")
))
const transportBaudLabel = computed(() => (
  transportBaudOptions.value.find((item) => item.value === transportForm.baudCode)?.label
  ?? String(transportForm.baudCode ?? "--")
))
const readStatusLabel = computed(() => {
  if (readStatus.value === "success") {
    return t("config.status.read")
  }

  if (readStatus.value === "error") {
    return t("config.status.readFailed")
  }

  return t("config.status.unread")
})
const readStatusTone = computed(() => {
  if (readStatus.value === "success") {
    return "success"
  }

  if (readStatus.value === "error") {
    return "danger"
  }

  return "idle"
})
const toolbarStatusTone = computed(() => {
  if (canDisplayConfig.value) {
    return readStatusTone.value
  }

  return isDeviceConnected.value ? "warning" : "idle"
})
const toolbarStatusLabel = computed(() => (
  canDisplayConfig.value ? readStatusLabel.value : emptyState.value?.badge
))
const emptyState = computed(() => {
  if (hasReadableConfig.value) {
    return null
  }

  if (!isDeviceConnected.value) {
    return {
      tone: "offline",
      icon: "cable",
      badge: t("config.empty.notConnected.badge"),
      title: t("config.empty.notConnected.title"),
      description: t("config.empty.notConnected.description"),
    }
  }

  return {
    tone: "standby",
    icon: "settings_ethernet",
    badge: t("config.empty.waitingRead.badge"),
    title: t("config.empty.waitingRead.title"),
    description: t("config.empty.waitingRead.description"),
  }
})

const shouldKeepCanHeartbeat = computed(() => (
  isDeviceConnected.value
  && linkReady.value
  && transportForm.commType === 0x02
  && !loading.init
  && !loading.read
  && !loading.write
))

function stopCanHeartbeat() {
  if (canHeartbeatTimer) {
    clearInterval(canHeartbeatTimer)
    canHeartbeatTimer = null
  }
  heartbeatInFlight = false
}

function startCanHeartbeat() {
  stopCanHeartbeat()

  if (!shouldKeepCanHeartbeat.value) {
    return
  }

  canHeartbeatTimer = setInterval(async () => {
    if (heartbeatInFlight) {
      return
    }

    heartbeatInFlight = true
    try {
      await sendMeterConfigHeartbeat({ commType: transportForm.commType })
    } catch (_) {
      // 心跳失败不直接弹窗，避免高频保活打断用户操作。
    } finally {
      heartbeatInFlight = false
    }
  }, 200)
}

function getTransportErrorMessage(error) {
  const message = String(error)
  if (!message.includes("等待命令 0x37 响应超时")) {
    return message
  }

  const transportLabel = transportForm.commType === 0x02 ? "CAN" : "UART"
  const baudLabel = transportBaudOptions.value.find((item) => item.value === transportForm.baudCode)?.label ?? transportForm.baudCode
  if (transportForm.commType === 0x02) {
    return `配置链路初始化超时。当前页面选择的是 ${transportLabel} / ${baudLabel}，请检查 CANH/CANL/GND 接线、仪表供电以及 CAN 帧类型是否与设备一致`
  }

  return `配置链路初始化超时。请确认适配器连接使用固定 115200，当前页面选择的是 ${transportLabel} / ${baudLabel}，并检查仪表是否上电、TX/RX/GND 接线是否正确`
}

watch(
  () => deviceStore.connectionStatus,
  (status) => {
    if (status !== "CONNECTED") {
      stopCanHeartbeat()
      linkReady.value = false
      meterConfigStore.resetForm()
      return
    }

    if (!linkReady.value && lastAction.value === t("config.actions.none")) {
      lastAction.value = t("config.actions.ready")
    }
  },
  { immediate: true },
)

watch(
  shouldKeepCanHeartbeat,
  (enabled) => {
    if (enabled) {
      startCanHeartbeat()
      return
    }

    stopCanHeartbeat()
  },
  { immediate: true },
)

onBeforeUnmount(() => {
  stopCanHeartbeat()
})

async function initTransport(options = {}) {
  const { silent = false, shouldNotifySuccess = true, trigger = t("config.actions.manualInit") } = options

  if (deviceStore.connectionStatus !== "CONNECTED") {
    openAdapterDialog?.()
    if (!silent) {
      notifyInfo(t("config.actions.connectAdapter"))
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
      if (result.success) {
        if (shouldNotifySuccess) {
          notifySuccess(result.message)
        }
      } else {
        notifyInfo(result.message)
      }
    }
    return result.success
  } catch (error) {
    linkReady.value = false
    lastAction.value = t("config.actions.initFailed", { trigger })
    if (!silent) {
      notifyError(getTransportErrorMessage(error))
    }
    return false
  } finally {
    loading.init = false
  }
}

async function ensureTransportReady(trigger, options = {}) {
  if (linkReady.value) {
    return true
  }

  return initTransport({ silent: false, trigger, ...options })
}

async function executeReadConfig() {
  const result = await readMeterConfig({ commType: transportForm.commType })
  Object.assign(form, decodeMeterConfig(result.bytes))
  hasLoadedConfig.value = true
  readStatus.value = "success"
  linkReady.value = true
  lastAction.value = t("config.actions.readSuccess")
  notifySuccess(t("config.actions.readSuccess"))
}

async function handleReadConfig() {
  if (loading.read || loading.init || loading.write) {
    return
  }

  if (!isDeviceConnected.value) {
    openAdapterDialog?.()
    notifyInfo(t("config.actions.connectAdapter"))
    return
  }

  if (!hasConnectionConfig.value) {
    notifyInfo(t("config.actions.requireConnectionConfig"))
    return
  }

  loading.read = true
  try {
    if (!linkReady.value) {
      const ready = await initTransport({
        silent: true,
        shouldNotifySuccess: false,
        trigger: t("config.actions.initBeforeRead"),
      })

      if (!ready) {
        throw new Error(getTransportErrorMessage("等待命令 0x37 响应超时"))
      }
    }

    await executeReadConfig()
  } catch (error) {
    hasLoadedConfig.value = false
    readStatus.value = "error"
    notifyError(error)
  } finally {
    loading.read = false
  }
}

async function handleWriteConfig() {
  if (!validateMeterConfig(form)) {
    notifyInfo(t("config.actions.required"))
    return
  }

  if (!(await ensureTransportReady(t("config.actions.initBeforeWrite")))) {
    return
  }

  loading.write = true
  try {
    const result = await writeMeterConfig(encodeMeterConfig(form))
    lastAction.value = result.message
    if (result.success) {
      notifySuccess(result.message)
    } else {
      notifyInfo(result.message)
    }
  } catch (error) {
    notifyError(error)
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
    hasLoadedConfig.value = true
    readStatus.value = "success"
    importedFileName.value = file.name
    lastAction.value = t("config.actions.imported", { name: file.name })
    notifySuccess(t("config.actions.importSuccess"))
  } catch (error) {
    notifyError(error)
  } finally {
    event.target.value = ""
  }
}

async function handleExportConfig() {
  if (!validateMeterConfig(form)) {
    notifyInfo(t("config.actions.required"))
    return
  }

  try {
    const path = await save({
      title: t("config.exportDialog.title"),
      defaultPath: t("config.exportDialog.defaultFileName", { timestamp: Date.now() }),
      filters: [{ name: "JSON", extensions: ["json"] }],
    })

    if (!path || Array.isArray(path)) {
      return
    }

    await saveTextFile(path, JSON.stringify(form, null, 2))
    lastAction.value = t("config.actions.exported", { name: path.split(/[\\\\/]/).pop() })
    notifySuccess(t("config.actions.exportSuccess"))
  } catch (error) {
    notifyError(error)
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

function setSectionRef(groupKey, el) {
  if (el) {
    sectionRefs.set(groupKey, el)
    return
  }
  sectionRefs.delete(groupKey)
}

</script>

<style scoped lang="scss">
.config-page {
  --config-page-pad-x: 12px;
  --config-page-pad-y: 12px;
  --config-gap: 12px;
  --config-panel-padding: 0px;
  --config-toolbar-padding-y: clamp(14px, 1vw, 18px);
  --config-toolbar-padding-x: clamp(18px, 1.1vw, 22px);
  --config-section-gap: clamp(16px, 1vw, 20px);
  --config-field-gap-x: clamp(14px, 0.95vw, 18px);
  --config-field-gap-y: clamp(16px, 1vw, 20px);
  display: flex;
  flex-direction: column;
  gap: var(--config-gap);
  height: 100%;
  min-height: 0;
  max-height: 100dvh;
  padding: var(--config-page-pad-y) var(--config-page-pad-x);
  box-sizing: border-box;
  overflow: hidden;
  background: transparent;
}

.config-page--ledger {
  color: var(--dt-text-primary);
}

.toolbar-panel {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 12px;
  width: 100%;
  flex-wrap: nowrap;
  padding: var(--config-toolbar-padding-y) var(--config-toolbar-padding-x);
  background: transparent;
  border-color: var(--dt-header-border);
  color: var(--dt-text-primary);
  overflow: hidden;
}

.toolbar-panel--header {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  width: 100%;
  margin: 0;
  max-width: none;
  padding: 0;
  background: transparent;
  border: 0;
  box-shadow: none;
  color: var(--dt-text-primary);
  align-items: center;
  pointer-events: none;
}

.toolbar-panel__group {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: auto;
  flex-wrap: nowrap;
  pointer-events: none;
  flex: 0 0 auto;
}

.toolbar-panel__group--status {
  gap: 8px;
  justify-content: flex-end;
  flex-wrap: nowrap;
  margin-left: 0;
}

.toolbar-panel__group--actions {
  display: flex;
  justify-content: flex-end;
  flex-wrap: nowrap;
  min-width: auto;
  max-width: none;
  border-left: 1px solid var(--dt-border);
  margin-left: 2px;
  padding: 0 0 0 16px;
  flex: 0 0 auto;
}

.toolbar-panel__group--actions :deep(.q-btn) {
  min-height: 40px;
  min-width: 132px;
  padding: 0 18px;
  border-radius: var(--dt-radius-button);
  white-space: nowrap;
  flex: 0 0 auto;
}

.toolbar-panel__group--actions :deep(.q-btn__content) {
  flex-wrap: nowrap;
  white-space: nowrap;
  font-size: 14px;
  font-weight: 800;
  letter-spacing: 0;
}

.toolbar-panel__group--actions :deep(.q-btn__content > span) {
  white-space: nowrap;
}

.header-status-chip {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  min-height: 32px;
  padding: 0 14px;
  border: 1px solid var(--dt-border);
  border-radius: 999px;
  background: var(--dt-bg-panel);
  color: var(--dt-text-primary);
  font-size: 12px;
  font-weight: 700;
  white-space: nowrap;
  box-shadow: var(--dt-shadow-panel);
  transition:
    border-color 0.6s ease,
    background-color 0.6s ease,
    color 0.6s ease,
    box-shadow 0.6s ease;
}

.header-status-chip__dot {
  width: 9px;
  height: 9px;
  border-radius: 999px;
  background: var(--dt-text-muted);
  box-shadow: none;
  transition:
    background-color 0.6s ease,
    box-shadow 0.6s ease,
    transform 0.6s ease;
}

.header-status-chip--success .header-status-chip__dot,
.header-status-chip__dot--success {
  background: var(--dt-success);
  box-shadow: 0 0 0 3px var(--dt-status-success-soft);
}

.header-status-chip--danger .header-status-chip__dot,
.header-status-chip__dot--danger {
  background: var(--dt-danger);
  box-shadow: 0 0 0 3px var(--dt-status-danger-soft);
}

.header-status-chip--warning .header-status-chip__dot,
.header-status-chip__dot--warning {
  background: var(--dt-warning);
  box-shadow: 0 0 0 3px var(--dt-status-warning-soft);
}

.header-status-chip__dot--idle,
.header-status-chip--idle .header-status-chip__dot {
  background: var(--dt-text-muted);
  box-shadow: none;
}

.toolbar-panel--header :deep(.q-btn),
.toolbar-panel--header :deep(.q-field),
.toolbar-panel--header :deep(.q-select),
.toolbar-panel--header :deep(.q-btn-group),
.toolbar-panel--header :deep(.q-btn-dropdown),
.toolbar-panel--header :deep(button),
.toolbar-panel--header :deep(input),
.toolbar-panel--header :deep(select),
.toolbar-panel--header :deep(textarea),
.toolbar-panel--header :deep([role="button"]),
.toolbar-panel--header :deep([tabindex]) {
  pointer-events: auto;
}

.toolbar-panel--content,
.toolbar-panel--content .toolbar-panel__group {
  pointer-events: auto;
}

.toolbar-panel--content {
  border: 0;
  border-radius: 0;
  background: transparent;
  box-shadow: none;
}

.toolbar-panel--content::before {
  display: none;
}

.toolbar-panel--content .toolbar-panel__group--actions {
  border-left: 0;
  margin-left: 0;
  padding-left: 0;
}

.workspace-grid {
  display: flex;
  flex-direction: column;
  gap: var(--config-gap);
  width: 100%;
  flex: 1 1 auto;
  min-height: 0;
}

.workspace-grid--empty {
  flex: 1 1 auto;
  min-height: 0;
}

.panel--main {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  padding: var(--config-panel-padding);
  min-width: 0;
  min-height: 0;
  background: transparent;
  border: 0;
  border-radius: 0;
  box-shadow: none;
  overflow: hidden;
}

.panel--main--empty {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  min-height: 0;
}

.panel--main::before {
  display: none;
}

.panel-body {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  min-height: 0;
  overflow: hidden;
  padding-top: 0;
  padding-bottom: 0;
}

.panel-body--empty {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  min-height: 0;
  overflow: hidden;
}

.panel-body__content {
  display: flex;
  flex-direction: column;
  gap: var(--config-section-gap);
  min-height: min-content;
  padding: 0 10px 20px 0;
}

.panel-body__scroll {
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  min-height: 0;
  overflow: hidden;
}

.panel-body__scroll :deep(.q-scrollarea__container) {
  border-radius: 18px;
}

.panel-body__scroll :deep(.q-scrollarea__content) {
  min-height: 100%;
}

.panel-body__scroll :deep(.q-scrollarea__thumb) {
  width: 8px;
  border-radius: 999px;
  background: color-mix(in srgb, var(--dt-text-muted) 56%, transparent);
  opacity: 1;
}

.panel-body__scroll :deep(.q-scrollarea__bar) {
  width: 10px;
  right: 2px;
}

.config-empty {
  --config-empty-accent: var(--dt-header-muted);
  --config-empty-accent-soft: color-mix(in srgb, var(--dt-header-muted) 16%, transparent);
  --config-empty-accent-line: color-mix(in srgb, var(--dt-header-muted) 34%, transparent);
  --config-empty-accent-strong: color-mix(in srgb, var(--dt-header-muted) 48%, transparent);
  --config-empty-accent-glow: color-mix(in srgb, var(--dt-header-muted) 24%, transparent);
  position: relative;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: stretch;
  flex: 1 1 auto;
  min-height: 0;
  max-height: none;
  padding: clamp(12px, 1.2vw, 18px) 0 0;
  border-radius: 0;
  border: 0;
  background: transparent;
  box-shadow: none;
  overflow: hidden;
}

.panel-body--empty .config-empty {
  flex: 1 1 auto;
  min-height: 0;
  max-height: none;
}

.config-empty::before {
  display: none;
}

.config-empty--standby {
  --config-empty-accent: var(--dt-brand-primary);
  --config-empty-accent-soft: var(--dt-brand-primary-soft);
  --config-empty-accent-line: color-mix(in srgb, var(--dt-brand-primary) 34%, transparent);
  --config-empty-accent-strong: color-mix(in srgb, var(--dt-brand-primary) 48%, transparent);
  --config-empty-accent-glow: color-mix(in srgb, var(--dt-brand-primary) 24%, transparent);
}

.config-empty__hero {
  position: relative;
  z-index: 1;
  display: flex;
  flex: 1 1 auto;
  min-height: 100%;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: clamp(20px, 2.4vw, 32px);
  width: 100%;
  max-width: 920px;
  margin: 0 auto;
}

.config-empty__visual {
  position: relative;
  display: grid;
  place-items: center;
  flex: 0 0 auto;
  width: min(100%, 360px);
  min-width: 0;
  min-height: 320px;
  transition:
    filter 0.9s cubic-bezier(0.22, 1, 0.36, 1),
    opacity 0.9s cubic-bezier(0.22, 1, 0.36, 1),
    transform 0.9s cubic-bezier(0.22, 1, 0.36, 1);
}

.config-empty__visual::before {
  content: "";
  position: absolute;
  width: 220px;
  height: 220px;
  border-radius: 999px;
  background: radial-gradient(circle, var(--config-empty-accent-glow), transparent 68%);
  filter: blur(10px);
  opacity: 0.9;
  transition:
    background 0.9s ease,
    opacity 0.9s ease;
}

.config-empty__halo {
  position: absolute;
  border-radius: 999px;
  border: 1.5px dashed var(--config-empty-accent-line);
  box-shadow:
    0 0 0 1px color-mix(in srgb, var(--dt-text-contrast) 18%, transparent),
    0 0 24px color-mix(in srgb, var(--dt-text-contrast) 8%, transparent);
  animation: config-empty-orbit 22s linear infinite;
  transition:
    border-color 0.9s ease,
    box-shadow 0.9s ease,
    opacity 0.9s ease;
}

.config-empty__halo--outer {
  width: 280px;
  height: 280px;
  opacity: 0.96;
}

.config-empty__halo--inner {
  width: 212px;
  height: 212px;
  border-color: var(--config-empty-accent-strong);
  opacity: 1;
  animation-duration: 18s;
  animation-direction: reverse;
}

.config-empty__spark {
  position: absolute;
  z-index: 1;
  width: 10px;
  height: 10px;
  border-radius: 999px;
  background: linear-gradient(180deg, color-mix(in srgb, var(--dt-text-contrast) 88%, transparent), var(--config-empty-accent));
  box-shadow:
    0 0 0 6px color-mix(in srgb, var(--dt-text-contrast) 24%, transparent),
    0 0 0 11px var(--config-empty-accent-soft),
    0 0 22px var(--config-empty-accent-glow);
  animation: config-empty-spark 3.2s ease-in-out infinite;
  transition:
    background 0.9s ease,
    box-shadow 0.9s ease,
    opacity 0.9s ease;
}

.config-empty__spark--top {
  top: 56px;
  right: 78px;
}

.config-empty__spark--right {
  top: 50%;
  right: 35px;
  transform: translateY(-50%);
  animation-delay: 0.9s;
}

.config-empty__spark--bottom {
  left: 86px;
  bottom: 54px;
  animation-delay: 1.7s;
}

.config-empty__device-shell {
  position: relative;
  display: grid;
  place-items: center;
  width: 126px;
  height: 126px;
  border-radius: 36px;
  background: var(--dt-gloss-surface);
  border: 1px solid var(--dt-gloss-border);
  box-shadow:
    var(--dt-gloss-inset),
    0 20px 44px color-mix(in srgb, var(--dt-shadow-panel) 100%, transparent),
    0 0 0 12px color-mix(in srgb, var(--dt-text-contrast) 10%, transparent);
  animation: config-empty-float 4.6s ease-in-out infinite;
  transition:
    border-color 0.9s ease,
    box-shadow 0.9s ease,
    background 0.9s ease,
    transform 0.9s ease;
}

.config-empty__device-shell::before {
  content: "";
  position: absolute;
  inset: 14px;
  border-radius: 28px;
  background: linear-gradient(180deg, var(--config-empty-accent-soft), color-mix(in srgb, var(--dt-bg-panel) 86%, transparent));
}

.config-empty__device-shell::after {
  content: "";
  position: absolute;
  inset: -12px;
  border-radius: 44px;
  border: 1px solid color-mix(in srgb, var(--dt-text-contrast) 18%, transparent);
  box-shadow:
    0 0 0 1px var(--config-empty-accent-line),
    0 0 36px var(--config-empty-accent-glow);
  opacity: 0.72;
}

.config-empty__device-core {
  position: relative;
  z-index: 1;
  display: grid;
  place-items: center;
  width: 72px;
  height: 72px;
  border-radius: 24px;
  background: var(--dt-bg-panel);
  color: var(--config-empty-accent);
  box-shadow:
    var(--dt-gloss-inset),
    0 12px 24px color-mix(in srgb, var(--dt-shadow-panel) 100%, transparent),
    0 0 24px color-mix(in srgb, var(--dt-text-contrast) 10%, transparent);
  transition:
    color 0.9s ease,
    box-shadow 0.9s ease,
    background-color 0.9s ease,
    transform 0.9s ease;
}

.config-empty__device-core::before {
  content: "";
  position: absolute;
  inset: -14px;
  border-radius: 30px;
  border: 1px solid color-mix(in srgb, var(--dt-text-contrast) 16%, transparent);
  box-shadow: 0 0 0 1px color-mix(in srgb, var(--config-empty-accent) 18%, transparent);
}

.config-empty__endpoint {
  position: absolute;
  z-index: 1;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  min-height: 34px;
  padding: 0 14px;
  border-radius: 999px;
  border: 1px solid var(--config-empty-accent-strong);
  background: var(--dt-gloss-surface-soft);
  color: var(--config-empty-accent);
  font-size: 12px;
  font-weight: 800;
  letter-spacing: 0.04em;
  box-shadow:
    var(--dt-gloss-inset),
    0 10px 24px color-mix(in srgb, var(--dt-shadow-panel) 100%, transparent),
    0 0 18px color-mix(in srgb, var(--dt-text-contrast) 8%, transparent);
  transition:
    border-color 0.9s ease,
    color 0.9s ease,
    box-shadow 0.9s ease,
    transform 0.9s ease,
    background 0.9s ease;
}

.config-empty__endpoint::before {
  content: "";
  width: 8px;
  height: 8px;
  border-radius: 999px;
  background: var(--config-empty-accent);
  box-shadow:
    0 0 0 4px var(--config-empty-accent-soft),
    0 0 14px var(--config-empty-accent-glow);
  transition:
    background-color 0.9s ease,
    box-shadow 0.9s ease,
    transform 0.9s ease;
}

.config-empty__endpoint--left {
  top: 54px;
  left: 10px;
}

.config-empty__endpoint--right {
  right: 0;
  bottom: 58px;
}

.config-empty__content {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 0 1 auto;
  gap: 14px;
  min-height: 168px;
  text-align: center;
  transition:
    opacity 0.8s ease,
    transform 0.8s cubic-bezier(0.22, 1, 0.36, 1);
}

.config-empty--offline .config-empty__visual {
  filter: grayscale(1) saturate(0.18) brightness(1.02);
  opacity: 0.74;
}

.config-empty--offline .config-empty__content {
  opacity: 0.84;
}

.config-empty--standby .config-empty__visual {
  filter: grayscale(0) saturate(1) brightness(1);
  opacity: 1;
}

.config-empty--standby .config-empty__content {
  opacity: 1;
}

.config-empty h3 {
  margin: 0;
  min-height: 86px;
  max-width: 640px;
  color: var(--dt-text-primary);
  font-size: clamp(28px, 2.8vw, 36px);
  font-weight: 800;
  line-height: 1.18;
}

.config-empty__description {
  margin: 0;
  min-height: 84px;
  color: var(--dt-text-secondary);
  font-size: 15px;
  line-height: 1.8;
}

@keyframes config-empty-orbit {
  from {
    transform: rotate(0deg);
  }

  to {
    transform: rotate(360deg);
  }
}

@keyframes config-empty-float {
  0%,
  100% {
    transform: translateY(0);
  }

  50% {
    transform: translateY(-4px);
  }
}

@keyframes config-empty-spark {
  0%,
  100% {
    transform: scale(0.96);
    opacity: 0.68;
  }

  50% {
    transform: scale(1.12);
    opacity: 1;
  }
}

.section-block {
  position: relative;
  overflow: visible;
  margin-top: 12px;
  padding-top: 0;
  border-left: 1px solid var(--dt-border);
  border-right: 1px solid var(--dt-border);
  border-bottom: 1px solid var(--dt-border);
  border-radius: var(--dt-radius-subtle);
  background: var(--dt-bg-panel);
  box-shadow: var(--dt-shadow-panel);
}

.section-block__tag {
  position: relative;
  z-index: 1;
  flex: 0 0 auto;
}

.section-block__tag::after {
  display: none;
}

.section-block__header {
  display: flex;
  align-items: center;
  gap: 12px;
  min-height: auto;
  padding: 0 0 0 14px;
  border-bottom: 0;
  background: transparent;
  border-radius: 0;
  transform: translateY(-50%);
  margin-bottom: -10px;
}

.section-block__line {
  flex: 1 1 auto;
  height: 1px;
  background: var(--dt-border);
}

.section-block__tag h4 {
  margin: 0;
  display: inline-flex;
  align-items: center;
  min-height: 22px;
  padding: 0;
  border: 0;
  border-radius: 0;
  background: transparent;
  color: var(--dt-text-primary);
  font-size: 13px;
  font-weight: 800;
  letter-spacing: 0;
  text-transform: none;
  box-shadow: none;
}

.form-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: var(--config-field-gap-y) var(--config-field-gap-x);
  padding: 4px 14px 16px;
}

.field-item {
  display: flex;
  flex-direction: column;
  gap: 7px;
  min-width: 0;
}

.field-label {
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 700;
  line-height: 1.35;
  padding-left: 2px;
}

.config-page :deep(.q-field__native),
.config-page :deep(.q-field__input) {
  text-align: left;
}

.config-page :deep(.q-select .q-field__native) {
  justify-content: flex-start;
}

.hidden-input {
  display: none;
}

@media (min-width: 1760px) {
  .form-grid {
    grid-template-columns: repeat(4, minmax(0, 1fr));
  }
}

@media (max-width: 1440px) {
  .config-page {
    --config-page-pad-x: 12px;
    --config-page-pad-y: 12px;
    --config-gap: 12px;
    --config-panel-padding: 0px;
  }
}

@media (max-width: 1180px) {
  .toolbar-panel {
    align-items: stretch;
    justify-content: flex-start;
  }

  .toolbar-panel__group--status {
    justify-content: flex-start;
  }

  .toolbar-panel__group--actions {
    flex-wrap: wrap;
    justify-content: flex-start;
    border-left: 0;
    padding: 0;
    margin-left: 0;
  }

  .toolbar-panel__group--actions :deep(.q-btn) {
    min-width: 120px;
  }
}

@media (max-width: 1080px) {
  .form-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .section-block__header {
    gap: 12px;
    padding-left: 16px;
  }

  .form-grid {
    padding: 8px 16px 16px;
  }
}

@media (max-width: 820px) {
  .config-page {
    --config-page-pad-x: 12px;
    --config-page-pad-y: 12px;
  }

  .config-empty {
    padding: 24px 18px;
  }

  .config-empty__hero {
    flex-direction: column;
    gap: 24px;
    text-align: center;
  }

  .config-empty__visual {
    flex-basis: auto;
    min-width: 0;
    min-height: 240px;
  }

  .config-empty__content {
    align-items: center;
    min-height: 0;
    max-width: 100%;
  }

  .config-empty h3,
  .config-empty__description {
    min-height: 0;
  }

  .config-empty__endpoint--left {
    left: 0;
  }

  .config-empty__endpoint--right {
    right: 8px;
    bottom: 36px;
  }

  .form-grid {
    grid-template-columns: 1fr;
  }

  .section-block__header {
    flex-direction: column;
    align-items: flex-start;
    transform: translateY(-50%);
  }
}

@media (prefers-reduced-motion: reduce) {
  .config-empty__halo,
  .config-empty__spark,
  .config-empty__device-shell {
    animation: none !important;
  }
}

@media (max-width: 720px) {
  .section-block__tag {
    left: 16px;
  }

  .section-block__header {
    padding-left: 16px;
  }
}
</style>
