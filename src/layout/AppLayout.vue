<template>
  <q-layout class="app-shell" view="lHh Lpr lFf">
    <div :class="['shell-header', { 'shell-header--config': isConfigRoute }]">
      <template v-if="isConfigRoute">
        <div class="shell-header__config-layout">
          <div class="shell-header__config-bar">
            <div
              class="shell-drag-region shell-drag-region--full"
              @mousedown.left="handleWindowTitlebarMouseDown"
              @dblclick.stop="handleWindowTitlebarDoubleClick"
            />
            <div
              class="shell-toolbar__brand shell-toolbar__brand--inline shell-toolbar__brand--config shell-toolbar__brand--drag"
              @mousedown.left="handleWindowTitlebarMouseDown"
              @dblclick.stop="handleWindowTitlebarDoubleClick"
            >
              <q-icon :name="route.meta?.icon || 'dashboard'" size="22px" class="shell-toolbar__page-icon" />
              <div class="shell-toolbar__title">
                <strong :style="{ color: headerTitleColor }">{{ resolvedRouteTitle }}</strong>
              </div>
            </div>
            <div id="config-toolbar-host" class="shell-header__config-host" />
            <div class="shell-header__actions">
              <q-btn
                v-if="themeToggleEnabled"
                dense
                flat
                round
                size="14px"
                class="shell-theme-switch"
                :icon="themeToggleIcon"
                @click="handleThemeToggle"
              >
                <q-tooltip>{{ themeToggleLabel }}</q-tooltip>
              </q-btn>
              <q-btn dense flat round size="14px" class="shell-lang-switch" :disable="languageSwitching">
                <span class="shell-lang-switch__label">{{ languageSwitchLabel }}</span>
                <q-menu
                  anchor="bottom middle"
                  self="top middle"
                  :offset="[0, 12]"
                >
                  <q-list dense>
                    <q-item clickable v-close-popup :active="currentLocale === 'zh-CN'" @click="handleLanguageChange('zh-CN')">
                      <q-item-section>简体中文</q-item-section>
                    </q-item>
                    <q-item clickable v-close-popup :active="currentLocale === 'en-US'" @click="handleLanguageChange('en-US')">
                      <q-item-section>English</q-item-section>
                    </q-item>
                  </q-list>
                </q-menu>
              </q-btn>
              <div v-if="windowControlsEnabled" class="shell-window-actions">
                <q-btn dense flat icon="minimize" class="shell-window-action" @click.stop="handleWindowMinimize" />
                <q-btn dense flat icon="crop_square" class="shell-window-action" @click.stop="handleWindowToggleMaximize" />
                <q-btn dense flat icon="close" class="shell-window-action shell-window-action--close" @click.stop="handleWindowClose" />
              </div>
            </div>
          </div>
        </div>
      </template>

      <q-toolbar v-else class="shell-toolbar">
        <div
          class="shell-drag-region shell-drag-region--full"
          @mousedown.left="handleWindowTitlebarMouseDown"
          @dblclick.stop="handleWindowTitlebarDoubleClick"
        />
        <div
          class="shell-toolbar__brand shell-toolbar__brand--drag"
          @mousedown.left="handleWindowTitlebarMouseDown"
          @dblclick.stop="handleWindowTitlebarDoubleClick"
        >
          <q-icon :name="route.meta?.icon || 'dashboard'" size="22px" class="shell-toolbar__page-icon" />
          <div class="shell-toolbar__title">
            <strong :style="{ color: headerTitleColor }">{{ resolvedRouteTitle }}</strong>
          </div>
        </div>
        <div id="page-toolbar-host" class="shell-toolbar__page-host" />
        <div class="shell-header__actions">
          <q-btn
            v-if="themeToggleEnabled"
            dense
            flat
            round
            size="14px"
            class="shell-theme-switch"
            :icon="themeToggleIcon"
            @click="handleThemeToggle"
          >
            <q-tooltip>{{ themeToggleLabel }}</q-tooltip>
          </q-btn>
          <q-btn dense flat round size="14px" class="shell-lang-switch" :disable="languageSwitching">
            <span class="shell-lang-switch__label">{{ languageSwitchLabel }}</span>
            <q-menu
              anchor="bottom middle"
              self="top middle"
              :offset="[0, 12]"
            >
              <q-list dense>
                <q-item clickable v-close-popup :active="currentLocale === 'zh-CN'" @click="handleLanguageChange('zh-CN')">
                  <q-item-section>简体中文</q-item-section>
                </q-item>
                <q-item clickable v-close-popup :active="currentLocale === 'en-US'" @click="handleLanguageChange('en-US')">
                  <q-item-section>English</q-item-section>
                </q-item>
              </q-list>
            </q-menu>
          </q-btn>
          <div v-if="windowControlsEnabled" class="shell-window-actions">
            <q-btn dense flat icon="minimize" class="shell-window-action" @click.stop="handleWindowMinimize" />
            <q-btn dense flat icon="crop_square" class="shell-window-action" @click.stop="handleWindowToggleMaximize" />
            <q-btn dense flat icon="close" class="shell-window-action shell-window-action--close" @click.stop="handleWindowClose" />
          </div>
        </div>
      </q-toolbar>
    </div>

    <aside :class="['shell-sidebar', { 'shell-sidebar--config': isConfigRoute }]">
      <div class="sidebar-surface">
        <div class="drawer-shell">
          <div class="sidebar-brand">
            <div class="sidebar-brand__mark">
              <img :src="logoImage" alt="DT-Tools logo" class="sidebar-brand__logo" />
            </div>
          </div>

          <div class="sidebar-main">
            <q-scroll-area class="sidebar-scroll">
              <div class="sidebar-scroll__content">
                <q-list class="nav-list">
                  <q-btn
                    v-for="item in primaryNavSections"
                    :key="item.to"
                    :to="item.to"
                    :icon="item.icon"
                    :label="t(item.labelKey)"
                    :color="isNavActive(item) ? 'primary' : undefined"
                    :text-color="isNavActive(item) ? 'white' : undefined"
                    :flat="!isNavActive(item)"
                    :push="isNavActive(item)"
                    :glossy="isNavActive(item)"
                    no-caps
                    align="left"
                    :class="['nav-home', 'nav-home--single', { 'nav-home--active': isNavActive(item) }]"
                  />
                </q-list>
              </div>
            </q-scroll-area>
          </div>

          <div class="sidebar-bottom">
            <section class="device-dock">
              <section class="device-connect-panel">
                <div class="device-connect-panel__head">
                  <div>
                    <h3>{{ t("layout.device.title") }}</h3>
                  </div>
                  <q-btn
                    to="/settings"
                    flat
                    round
                    dense
                    unelevated
                    :class="['device-dock__settings', { 'device-dock__settings--active': route.path === '/settings' }]"
                  >
                    <q-icon name="settings" size="18px" />
                  </q-btn>
                </div>

                <div class="device-connect-panel__body">
                  <div class="drawer-form">
                    <div class="drawer-field">
                      <label>{{ t("layout.device.manualModel") }}</label>
                      <q-input
                        v-model="manualModelInput"
                        outlined
                        dense
                        clearable
                        :placeholder="t('layout.device.manualModelPlaceholder')"
                        @update:model-value="handleManualModelInputChange"
                        @keyup.enter="handleManualModelSubmit"
                      />
                    </div>

                    <div class="drawer-field">
                      <label>{{ t("layout.device.cqCode") }}</label>
                      <q-input
                        v-model="panelCqCode"
                        outlined
                        dense
                        clearable
                        :placeholder="t('layout.device.cqCodePlaceholder')"
                        @update:model-value="handlePanelCqCodeChange"
                      />
                    </div>

                    <div class="drawer-field">
                      <label>{{ t("layout.device.port") }}</label>
                      <q-select
                        v-model="deviceStore.port"
                        :options="deviceStore.ports"
                        emit-value
                        map-options
                        outlined
                        dense
                        option-label="label"
                        option-value="value"
                      />
                    </div>

                  </div>
                </div>

                <div class="device-connect-panel__version">
                  <div class="device-connect-panel__version-badge">
                    <span class="device-connect-panel__version-entry" @click="handleDeveloperModeEntryClick">
                      {{ displayedAppVersionLabel }}
                    </span>
                    <q-btn
                      flat
                      round
                      dense
                      size="11px"
                      icon="refresh"
                      class="device-connect-panel__version-refresh"
                      :loading="versionRefreshing"
                      @click="refreshRemoteVersion"
                    />
                  </div>
                </div>

                <div class="device-connect-panel__actions">
                  <q-btn class="device-connect-panel__action-secondary" color="white" text-color="black" :label="t('layout.device.refreshPorts')" @click="refreshPorts" />
                  <q-btn
                    :class="[
                      'device-connect-panel__action-primary',
                      {
                        'device-connect-panel__action-primary--disconnect': deviceStore.connectionStatus === 'CONNECTED',
                        'device-connect-panel__action-primary--connect': deviceStore.connectionStatus !== 'CONNECTED',
                      },
                    ]"
                    :color="deviceStore.connectionStatus === 'CONNECTED' ? 'negative' : 'primary'"
                    push
                    :loading="connecting"
                    :label="deviceStore.connectionStatus === 'CONNECTED' ? t('layout.device.disconnectDevice') : t('layout.device.connectDevice')"
                    @click="toggleConnection"
                  />
                </div>
              </section>
            </section>
          </div>
        </div>
      </div>
    </aside>

    <q-page-container class="shell-frame">
      <div class="shell-frame-shadow" aria-hidden="true" />
      <q-page class="shell-page" :style-fn="getShellPageStyle">
        <section
          :class="[
            'shell-page-container',
            { 'shell-page-container--full': isConfigRoute },
          ]"
        >
          <section
            :class="[
              'content',
              {
                'content--home': route.path === '/home',
                'content--config': isConfigRoute,
                'content--settings': isSettingsRoute,
              },
            ]"
          >
            <router-view v-slot="{ Component, route: currentRoute }">
              <keep-alive>
                <component
                  :is="Component"
                  v-if="currentRoute.meta?.keepAlive !== false"
                  :key="currentRoute.name || currentRoute.path"
                />
              </keep-alive>
              <component
                :is="Component"
                v-if="currentRoute.meta?.keepAlive === false"
                :key="currentRoute.fullPath"
              />
            </router-view>
          </section>
        </section>
      </q-page>
    </q-page-container>

  </q-layout>

  <q-dialog v-model="cqGeneratorDialogOpen" persistent no-shake>
    <q-card class="cq-dialog cq-dialog--generator">
      <q-card-section class="cq-dialog__section">
        <div class="cq-dialog__title">CQ 配置生成器</div>
        <div class="cq-dialog__desc">可直接生成新的 CQ 配置串，也可回显左侧当前配置串。</div>
      </q-card-section>
      <q-card-section class="cq-dialog__section cq-dialog__section--compact">
        <q-btn-toggle
          v-model="cqTransportMode"
          spread
          no-caps
          push
          glossy
          toggle-color="primary"
          color="white"
          text-color="grey-7"
          class="cq-dialog__transport-tabs"
          :options="cqTransportOptions"
        />
      </q-card-section>
      <q-card-section class="cq-dialog__section cq-dialog__grid">
        <q-select v-model="cqGeneratorForm.burnFileType" :options="cqBurnFileTypeOptions" emit-value map-options outlined dense label="目标文件类型" />
        <q-select
          v-if="cqTransportMode === 'uart'"
          v-model="cqGeneratorForm.commType"
          :options="cqUartCommTypeOptions"
          emit-value
          map-options
          outlined
          dense
          label="UART 电平"
        />
        <q-input
          v-else
          :model-value="'CAN'"
          outlined
          dense
          readonly
          label="通讯类型"
        />
        <q-select v-model="cqGeneratorForm.baudCode" :options="cqBaudOptions" emit-value map-options outlined dense label="波特率编码" />
        <q-select v-model="cqGeneratorForm.frameType" :options="cqFrameTypeOptions" emit-value map-options outlined dense label="帧类型" :disable="cqGeneratorForm.commType !== 0x02" />
        <q-select v-model="cqGeneratorForm.powerVoltage" :options="cqPowerVoltageOptions" emit-value map-options outlined dense label="供电电压" />
        <q-select v-model="cqGeneratorForm.vlk5vEnabled" :options="cqVlk5vOptions" emit-value map-options outlined dense label="VLK5V 开关" />
        <q-select v-model="cqGeneratorForm.protocolType" :options="cqProtocolTypeOptions" emit-value map-options outlined dense label="升级协议类型" />
        <q-select v-model="cqGeneratorForm.frameId" :options="cqFrameIdOptions" emit-value map-options outlined dense label="帧ID" :disable="cqGeneratorForm.commType !== 0x02" />
        <q-input
          v-if="Number(cqGeneratorForm.frameId) === 0x02"
          v-model="cqGeneratorForm.specialFrameValue"
          outlined
          dense
          label="特殊帧ID"
          class="cq-dialog__field--full"
        />
      </q-card-section>
      <q-card-section class="cq-dialog__section cq-dialog__section--compact">
        <q-input :model-value="cqGeneratorPreview" class="cq-dialog__preview-input" outlined dense readonly label="CQ 配置串">
          <template #append>
            <q-btn flat dense no-caps color="primary" :loading="cqGeneratorRestoring" label="按型号回显" @click="restoreCurrentCqCode" />
            <q-btn flat round dense icon="content_copy" @click="copyCqCode" />
          </template>
        </q-input>
      </q-card-section>
      <q-card-actions align="right" class="cq-dialog__actions">
        <q-btn flat label="关闭" @click="cqGeneratorDialogOpen = false" />
      </q-card-actions>
    </q-card>
  </q-dialog>

  <q-dialog v-model="developerModeDialogOpen" persistent no-shake>
    <q-card class="cq-dialog cq-dialog--developer">
      <q-card-section class="cq-dialog__section">
        <div class="cq-dialog__title">开发模式验证</div>
        <div class="cq-dialog__desc">输入密码后可切换到开发模式，显示升级详细日志。</div>
      </q-card-section>
      <q-card-section class="cq-dialog__section cq-dialog__section--compact">
        <q-input
          v-model="developerModePassword"
          outlined
          dense
          autofocus
          type="password"
          label="开发模式密码"
          @keyup.enter="submitDeveloperModePassword"
        />
      </q-card-section>
      <q-card-actions align="right" class="cq-dialog__actions">
        <q-btn flat label="取消" @click="closeDeveloperModeDialog" />
        <q-btn unelevated color="primary" label="确认" @click="submitDeveloperModePassword" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>

<script setup>
import { computed, nextTick, onMounted, onUnmounted, provide, reactive, ref, watch } from "vue"
import { useRoute } from "vue-router"
import { useQuasar } from "quasar"
import { useI18n } from "vue-i18n"
import { getCurrentWindow } from "@tauri-apps/api/window"
import { frontendLog, queryCommonDictType, queryModelConfigByComputerName, switchLanguage } from "@/api/unimaster"
import { navSections } from "@/config/navigation"
import { applyLocale, getCurrentLocale, getDeviceLanguageCode, getLocaleSwitchLabel, getTargetLocale } from "@/i18n"
import { REMOTE_VERSION_REFRESH_EVENT } from "@/services/deviceVersion"
import { notifyError, notifyInfo, notifySuccess } from "@/services/ui"
import { useDeviceStore } from "@/store/device"
import { getRemoteAppVersion } from "@/updater"
import { loadPreferences, savePreferences, saveThemeMode } from "@/utils/preferences"
import { applyThemeMode } from "@/utils/theme"
import tauriConfig from "../../src-tauri/tauri.conf.json"
import {
  UPGRADE_CAN_BAUD_OPTIONS,
  UPGRADE_POWER_VOLTAGE_OPTIONS,
  UPGRADE_PROTOCOL_TYPE_OPTIONS,
  UPGRADE_UART_BAUD_OPTIONS,
  UPGRADE_UART_COMM_TYPE_OPTIONS,
  UPGRADE_VLK5V_OPTIONS,
  buildUpgradeCqCode,
  createDefaultUpgradeCqState,
  getUpgradeBaudOptions,
  parseUpgradeCqCode,
  getUpgradeFrameIdOptions,
  getUpgradeFrameTypeOptions,
} from "@/utils/upgrade-cq"
import { buildCqCodeFromModelConfig, buildUpgradeCqStateFromModelConfig } from "@/utils/model-config"
import darkLogoImage from "@/assect/images/logo.svg"
import lightLogoImage from "@/assect/images/log2.svg"

const route = useRoute()
const $q = useQuasar()
const { t, locale } = useI18n()
const deviceStore = useDeviceStore()
const DEV_MODE_UNLOCK_CLICKS = 7
const DEV_MODE_UNLOCK_PASSWORD = import.meta.env.VITE_DEV_MODE_PASSWORD || "dtkj2026dev"
const appWindow = typeof window !== "undefined" && window.__TAURI_INTERNALS__ ? getCurrentWindow() : null
const themeToggleEnabled = computed(() => !(typeof document !== "undefined" && document.documentElement.classList.contains("mac-theme-locked")))
const connecting = ref(false)
const languageSwitching = ref(false)
const hotplugDialogOpen = ref(false)
const languageSwitchLabel = computed(() => getLocaleSwitchLabel(locale.value))
const currentLocale = computed(() => locale.value)
const windowControlsEnabled = computed(() => Boolean(appWindow))
const currentThemeMode = computed(() => ($q.dark.isActive ? "dark" : "light"))
const headerTitleColor = computed(() => (currentThemeMode.value === "dark" ? "#ffffff" : "#17355e"))
const logoImage = computed(() => ($q.dark.isActive ? darkLogoImage : lightLogoImage))
const themeToggleIcon = computed(() => (currentThemeMode.value === "dark" ? "light_mode" : "dark_mode"))
const themeToggleLabel = computed(() => (currentThemeMode.value === "dark" ? "浅色模式" : "深色模式"))
const currentAppVersion = String(tauriConfig?.version || "0.1.0")
const displayedAppVersion = ref(currentAppVersion)
const versionRefreshing = ref(false)
const displayedAppVersionLabel = computed(() => (displayedAppVersion.value ? `v${displayedAppVersion.value}` : "--"))
const resolvedRouteTitle = computed(() => {
  if (route.meta?.titleKey) {
    return t(route.meta.titleKey)
  }
  return t("nav.workspace")
})
const currentWindowTitle = computed(() => resolvedRouteTitle.value || "DT-Tools")
let unlistenPortWatcher = null
let startupHotplugPromptTimer = null
let hiddenEntryTimer = null
let developerModeEntryTimer = null
const isConfigRoute = computed(() => route.path === "/config")
const isSettingsRoute = computed(() => route.path === "/settings")
const isSoftwareRoute = computed(() => route.path === "/software")
const cqSyncKey = computed(() => `${route.path}:${deviceStore.softwareUpgradeTargetKind || "app"}`)
const primaryNavSections = computed(() => (
  navSections.filter((item) => ["/config", "/software", "/unimaster-about"].includes(item.to))
))
const manualModelInput = ref("")
const panelCqCode = computed({
  get: () => deviceStore.upgradeCqCode,
  set: (value) => {
    deviceStore.setUpgradeCqCode(value)
  },
})
const hiddenEntryClicks = ref(0)
const developerModeEntryClicks = ref(0)
const developerModeDialogOpen = ref(false)
const developerModePassword = ref("")
const cqGeneratorDialogOpen = ref(false)
const cqGeneratorRestoring = ref(false)
const panelCqSyncing = ref(false)
const cqDictMap = ref(new Map())
const cqGeneratorForm = reactive({
  burnFileType: 1,
  ...createDefaultUpgradeCqState(),
})
const cqBurnFileTypeOptions = [
  { label: "APP", value: 1 },
  { label: "UI", value: 2 },
]
const cqTransportOptions = [
  { label: "UART", value: "uart" },
  { label: "CAN", value: "can" },
]
const cqTransportMode = computed({
  get: () => (Number(cqGeneratorForm.commType) === 0x02 ? "can" : "uart"),
  set: (value) => {
    const nextBaudOptions = value === "can" ? getDictDrivenBaudOptions(0x02) : getDictDrivenBaudOptions(0x00)
    if (value === "can") {
      cqGeneratorForm.commType = 0x02
      if (!nextBaudOptions.some((item) => Number(item.value) === Number(cqGeneratorForm.baudCode))) {
        cqGeneratorForm.baudCode = 0x08
      }
      return
    }

    if (Number(cqGeneratorForm.commType) === 0x02) {
      cqGeneratorForm.commType = 0x00
    }
    if (!nextBaudOptions.some((item) => Number(item.value) === Number(cqGeneratorForm.baudCode))) {
      cqGeneratorForm.baudCode = 0x0B
    }
  },
})
const cqUartCommTypeOptions = computed(() => getDictDrivenCommTypeOptions())
const cqBaudOptions = computed(() => getDictDrivenBaudOptions(cqGeneratorForm.commType))
const cqPowerVoltageOptions = computed(() => getDictDrivenPlainOptions(4, UPGRADE_POWER_VOLTAGE_OPTIONS))
const cqVlk5vOptions = computed(() => getDictDrivenPlainOptions(5, UPGRADE_VLK5V_OPTIONS))
const cqProtocolTypeOptions = computed(() => getDictDrivenPlainOptions(6, UPGRADE_PROTOCOL_TYPE_OPTIONS))
const cqFrameTypeOptions = computed(() => getDictDrivenFrameTypeOptions(cqGeneratorForm.commType))
const cqFrameIdOptions = computed(() => getDictDrivenFrameIdOptions(cqGeneratorForm.commType))
const cqGeneratorPreview = computed(() => {
  try {
    return buildUpgradeCqCode(cqGeneratorForm, cqGeneratorForm.burnFileType)
  } catch {
    return ""
  }
})

function getPerfNow() {
  return typeof performance !== "undefined" ? performance.now() : Date.now()
}

function waitForDelay(delayMs) {
  return new Promise((resolve) => {
    window.setTimeout(resolve, delayMs)
  })
}

function getShellPageStyle() {
  return {
    minHeight: "0px",
    height: "100%",
  }
}

function logStartupPerf(stage, details = {}) {
  const serializedDetails = Object.entries(details)
    .map(([key, value]) => `${key}=${value}`)
    .join(" ")

  const message = `[perf][startup][${stage}]${serializedDetails ? ` ${serializedDetails}` : ""}`
  console.info(message)
  frontendLog("info", message).catch(() => {})
}

function normalizeDictOptions(rawOptions) {
  if (!Array.isArray(rawOptions)) {
    return []
  }

  return rawOptions
    .map((item) => ({
      label: String(item?.label || "").trim(),
      value: Number(item?.value),
      type: String(item?.type || "").trim().toLowerCase(),
    }))
    .filter((item) => item.label && Number.isFinite(item.value))
}

function parseDictRemark(remark) {
  if (!String(remark || "").trim()) {
    return []
  }

  try {
    return normalizeDictOptions(JSON.parse(String(remark)))
  } catch (error) {
    console.warn("[cq-dict] 解析字典 remark 失败", remark, error)
    return []
  }
}

function getDictOptionsByValue(dictValue) {
  return cqDictMap.value.get(Number(dictValue)) || []
}

function getDictDrivenCommTypeOptions() {
  const options = getDictOptionsByValue(1)
  return options.length ? options.filter((item) => Number(item.value) !== 0x02) : UPGRADE_UART_COMM_TYPE_OPTIONS
}

function getDictDrivenBaudOptions(commType) {
  const options = getDictOptionsByValue(2)
  if (!options.length) {
    return getUpgradeBaudOptions(commType)
  }

  const targetType = Number(commType) === 0x02 ? "can" : "uart"
  const filtered = options.filter((item) => item.type === targetType)
  return filtered.length ? filtered : getUpgradeBaudOptions(commType)
}

function getDictDrivenFrameTypeOptions(commType) {
  const options = getDictOptionsByValue(3)
  if (!options.length) {
    return getUpgradeFrameTypeOptions(commType)
  }

  const targetType = Number(commType) === 0x02 ? "can" : "uart"
  const filtered = options.filter((item) => item.type === targetType)
  return filtered.length ? filtered : getUpgradeFrameTypeOptions(commType)
}

function getDictDrivenPlainOptions(dictValue, fallbackOptions) {
  const options = getDictOptionsByValue(dictValue)
  return options.length ? options.map(({ label, value }) => ({ label, value })) : fallbackOptions
}

function getDictDrivenFrameIdOptions(commType) {
  if (Number(commType) !== 0x02) {
    return [{ label: "串口默认", value: 0x00 }]
  }

  return getDictDrivenPlainOptions(8, getUpgradeFrameIdOptions(commType))
}

async function loadUpgradeCqDict() {
  try {
    const dictRows = await queryCommonDictType("dt_upgrade_cq_config")
    const nextMap = new Map()

    dictRows
      .slice()
      .sort((left, right) => Number(left?.dictValue) - Number(right?.dictValue))
      .forEach((row) => {
        nextMap.set(Number(row?.dictValue), parseDictRemark(row?.remark))
      })

    cqDictMap.value = nextMap
  } catch (error) {
    console.warn("[cq-dict] 读取字典失败，继续使用本地默认配置", error)
  }
}

function applyParsedCqToDevice(parsedCq) {
  const isCan = Number(parsedCq.commType) === 0x02
  const nextCommType = isCan ? 0x02 : 0x01
  const nextBaudCode = Number(parsedCq.baudCode)
  const nextFrameType = isCan ? Number(parsedCq.frameType || 0x01) : 0

  deviceStore.setMeterCommType(nextCommType)
  deviceStore.setMeterBaudCode(nextBaudCode)
  if (isCan) {
    deviceStore.setMeterFrameType(nextFrameType)
  }
}

function syncDeviceConfigWithCqCode(input, options = {}) {
  const { notifyOnError = false, syncTransport = !isSoftwareRoute.value } = options
  const normalized = String(input || "").trim().toUpperCase()

  deviceStore.setUpgradeCqCode(normalized)
  if (!normalized) {
    return false
  }

  try {
    const parsedCq = parseUpgradeCqCode(normalized)
    if (syncTransport) {
      applyParsedCqToDevice(parsedCq)
    }
    return true
  } catch (error) {
    if (notifyOnError) {
      notifyError(error)
    }
    return false
  }
}

function handlePanelCqCodeChange(value) {
  syncDeviceConfigWithCqCode(value)
}

function handleManualModelInputChange(value) {
  const normalized = String(value || "")
  manualModelInput.value = normalized

  if (String(deviceStore.upgradeCqCode || "").trim()) {
    deviceStore.setUpgradeCqCode("")
    deviceStore.resetMeterLink()
  }
}

async function handleManualModelSubmit() {
  const normalized = String(manualModelInput.value || "").trim().toUpperCase()
  manualModelInput.value = normalized

  if (!normalized) {
    notifyError(t("layout.device.manualModelRequired"))
    return
  }

  try {
    const config = await queryModelConfigByComputerName(normalized)
    if (!config) {
      deviceStore.setModel(normalized)
      notifyInfo(t("layout.device.manualModelNotFound", { model: normalized }))
      return
    }

    const cqCode = buildCqCodeFromModelConfig(config, getGeneratorProfileOptions())
    deviceStore.setModel(normalized)
    manualModelInput.value = String(config.computerName || normalized).trim().toUpperCase()
    const syncSuccess = syncDeviceConfigWithCqCode(cqCode, { notifyOnError: true })
    if (!syncSuccess) {
      throw new Error(t("layout.device.manualModelReadFailed", { model: normalized }))
    }
    notifySuccess(t("layout.device.manualModelLoaded", { model: normalized }))
  } catch (error) {
    console.error("[layout] failed to read model config:", error)
    notifyError(error?.message ? error : t("layout.device.manualModelReadFailed", { model: normalized }))
  }
}

function resetHiddenEntryCounter() {
  hiddenEntryClicks.value = 0
  if (hiddenEntryTimer) {
    window.clearTimeout(hiddenEntryTimer)
    hiddenEntryTimer = null
  }
}

function resetDeveloperModeEntryCounter() {
  developerModeEntryClicks.value = 0
  if (developerModeEntryTimer) {
    window.clearTimeout(developerModeEntryTimer)
    developerModeEntryTimer = null
  }
}

function closeDeveloperModeDialog() {
  developerModeDialogOpen.value = false
  developerModePassword.value = ""
}

function openDeveloperModeDialog() {
  developerModePassword.value = ""
  developerModeDialogOpen.value = true
}

function handleDeveloperModeEntryClick() {
  if (deviceStore.developerModeEnabled) {
    notifyInfo("开发模式已开启")
    return
  }

  developerModeEntryClicks.value += 1
  if (developerModeEntryTimer) {
    window.clearTimeout(developerModeEntryTimer)
  }

  developerModeEntryTimer = window.setTimeout(() => {
    resetDeveloperModeEntryCounter()
  }, 1800)

  if (developerModeEntryClicks.value < DEV_MODE_UNLOCK_CLICKS) {
    return
  }

  resetDeveloperModeEntryCounter()
  openDeveloperModeDialog()
}

function submitDeveloperModePassword() {
  const normalized = String(developerModePassword.value || "").trim()

  if (!normalized) {
    notifyError("请输入开发模式密码")
    return
  }

  if (normalized !== DEV_MODE_UNLOCK_PASSWORD) {
    notifyError("开发模式密码错误")
    return
  }

  deviceStore.developerModeEnabled = true
  savePreferences({
    ...loadPreferences(),
    developerModeEnabled: true,
  })
  closeDeveloperModeDialog()
  notifySuccess("开发模式已开启，详细日志已解锁")
}

function handleHiddenEntryClick() {
  hiddenEntryClicks.value += 1
  if (hiddenEntryTimer) {
    window.clearTimeout(hiddenEntryTimer)
  }

  hiddenEntryTimer = window.setTimeout(() => {
    resetHiddenEntryCounter()
  }, 1500)

  if (hiddenEntryClicks.value < 5) {
    return
  }

  resetHiddenEntryCounter()
  openCqGeneratorDialog()
}

function handleGlobalHiddenEntryClick(event) {
  if (cqGeneratorDialogOpen.value) {
    resetHiddenEntryCounter()
    return
  }

  if (event?.button !== 2) {
    resetHiddenEntryCounter()
    return
  }

  handleHiddenEntryClick()
}

function syncCqGeneratorWithDevice() {
  if (deviceStore.meterCommType === 0x02) {
    cqGeneratorForm.commType = 0x02
    cqGeneratorForm.baudCode = Number(deviceStore.meterBaudCode ?? 0x08)
    cqGeneratorForm.frameType = Number(deviceStore.meterFrameType ?? 0x01) || 0x01
    cqGeneratorForm.frameId = cqGeneratorForm.frameId === 0x02 ? 0x02 : 0x01
    cqGeneratorForm.fileFormat = 0x01
    return
  }

  cqGeneratorForm.commType = 0x00
  cqGeneratorForm.baudCode = Number(deviceStore.meterBaudCode ?? 0x0B)
  cqGeneratorForm.frameType = 0x00
  cqGeneratorForm.fileFormat = 0x01
  cqGeneratorForm.frameId = 0x00
  cqGeneratorForm.specialFrameValue = ""
}

function applyParsedCqToGenerator(parsedCq) {
  cqGeneratorForm.commType = Number(parsedCq.commType)
  cqGeneratorForm.baudCode = Number(parsedCq.baudCode)
  cqGeneratorForm.frameType = Number(parsedCq.frameType ?? 0x00)
  cqGeneratorForm.powerVoltage = Number(parsedCq.powerVoltage)
  cqGeneratorForm.vlk5vEnabled = Number(parsedCq.vlk5vEnabled)
  cqGeneratorForm.protocolType = Number(parsedCq.protocolType)
  cqGeneratorForm.burnFileType = Number(parsedCq.burnFileType ?? 1)
  cqGeneratorForm.fileFormat = Number(parsedCq.fileFormat ?? 0x01)
  cqGeneratorForm.frameId = Number(parsedCq.frameId ?? 0x01)
  cqGeneratorForm.specialFrameValue = String(parsedCq.specialFrameValue || "")
}

function getGeneratorModelName() {
  return String(manualModelInput.value || deviceStore.currentModel || "").trim().toUpperCase()
}

function getGeneratorProfileOptions() {
  if (isSoftwareRoute.value) {
    const softwareKind = ["app", "ui", "boot", "config"].includes(deviceStore.softwareUpgradeTargetKind)
      ? deviceStore.softwareUpgradeTargetKind
      : "app"
    return {
      transportProfile: "boot",
      burnFileType: softwareKind === "ui" ? 2 : softwareKind === "config" ? 3 : softwareKind === "boot" ? 0 : 1,
    }
  }

  return {
    transportProfile: "app",
    burnFileType: 1,
  }
}

async function syncPanelCqFromModel(options = {}) {
  const { notifyOnSuccess = false } = options
  const modelName = getGeneratorModelName()

  if (!modelName) {
    if (notifyOnSuccess) {
      notifyError("请先输入型号")
    }
    return false
  }

  if (panelCqSyncing.value) {
    return false
  }

  panelCqSyncing.value = true
  try {
    const config = await queryModelConfigByComputerName(modelName)
    if (!config) {
      if (notifyOnSuccess) {
        notifyError(`未找到型号 ${modelName} 的配置`)
      }
      return false
    }

    const profileOptions = getGeneratorProfileOptions()
    const resolvedModelName = String(config.computerName || modelName).trim().toUpperCase()
    const cqCode = buildCqCodeFromModelConfig(config, profileOptions)
    deviceStore.setModel(resolvedModelName)
    manualModelInput.value = resolvedModelName
    const syncSuccess = syncDeviceConfigWithCqCode(cqCode, { notifyOnError: notifyOnSuccess })
    if (!syncSuccess) {
      return false
    }

    if (notifyOnSuccess) {
      notifySuccess(`已回显型号 CQ：${cqCode}`)
    }
    return config
  } catch (error) {
    if (notifyOnSuccess) {
      notifyError(error?.message ? error.message : error)
    }
    return false
  } finally {
    panelCqSyncing.value = false
  }
}

async function restoreCurrentCqCode(options = {}) {
  const { notifyOnSuccess = true } = options

  cqGeneratorRestoring.value = true
  try {
    const config = await syncPanelCqFromModel({ notifyOnSuccess })
    if (!config) {
      return false
    }

    const cqState = buildUpgradeCqStateFromModelConfig(config, getGeneratorProfileOptions())
    Object.assign(cqGeneratorForm, cqState)
    return true
  } finally {
    cqGeneratorRestoring.value = false
  }
}

function openCqGeneratorDialog() {
  syncCqGeneratorWithDevice()
  cqGeneratorDialogOpen.value = true
}

async function copyCqCode() {
  if (!cqGeneratorPreview.value) {
    notifyError("CQ 配置串生成失败")
    return
  }

  try {
    await navigator.clipboard.writeText(cqGeneratorPreview.value)
    notifySuccess("CQ 配置串已复制")
  } catch (error) {
    notifyError(error)
  }
}

async function refreshRemoteVersion(options = {}) {
  const {
    silent = false,
    retries = silent ? 2 : 0,
    retryDelayMs = 450,
  } = options

  if (versionRefreshing.value) {
    return false
  }

  versionRefreshing.value = true
  try {
    let lastError = null

    for (let attempt = 0; attempt <= retries; attempt += 1) {
      try {
        const remoteVersion = String(await getRemoteAppVersion() || currentAppVersion).trim()

        if (!remoteVersion) {
          throw new Error("未读取到项目版本号")
        }

        const previousVersion = displayedAppVersion.value
        displayedAppVersion.value = remoteVersion

        if (!silent) {
          if (remoteVersion === currentAppVersion) {
            notifyInfo(`当前已是最新版本 v${currentAppVersion}`)
          } else if (remoteVersion === previousVersion) {
            notifyInfo(`检测到新版本 v${remoteVersion}`)
          } else {
            notifySuccess(`检测到新版本 v${remoteVersion}`)
          }
        }
        return true
      } catch (error) {
        lastError = error
        if (attempt < retries) {
          await waitForDelay(retryDelayMs)
        }
      }
    }

    if (!silent && lastError) {
      notifyError(lastError)
    } else if (lastError) {
      console.warn("[layout] failed to refresh remote version:", lastError)
    }
  } finally {
    versionRefreshing.value = false
  }

  return false
}

function handleRemoteVersionRefreshRequest(event) {
  const detail = event?.detail || {}
  refreshRemoteVersion({
    silent: detail.silent !== false,
    retries: Number.isInteger(detail.retries) ? detail.retries : undefined,
    retryDelayMs: Number.isInteger(detail.retryDelayMs) ? detail.retryDelayMs : undefined,
  })
}

function openAdapterDialog() {
  refreshPorts()
}

provide("openAdapterDialog", openAdapterDialog)

function isNavActive(item) {
  if (item.to === "/tools") {
    return route.path === "/tools" || route.path.startsWith("/tools/")
  }
  return route.path === item.to
}

async function handleWindowMinimize() {
  if (!appWindow) return

  await appWindow.minimize()
}

async function handleWindowToggleMaximize() {
  if (!appWindow) return

  await appWindow.toggleMaximize()
  await syncWindowState()
}

async function handleWindowTitlebarMouseDown(event) {
  if (!windowControlsEnabled.value || !appWindow) return
  if (event.detail > 1) return

  await appWindow.startDragging()
}

async function handleWindowTitlebarDoubleClick() {
  if (!windowControlsEnabled.value) return

  await handleWindowToggleMaximize()
}

async function handleWindowClose() {
  if (!appWindow) return

  await appWindow.close()
}

function handleThemeToggle() {
  if (!themeToggleEnabled.value) {
    return
  }
  const nextThemeMode = currentThemeMode.value === "dark" ? "light" : "dark"
  const appliedThemeMode = applyThemeMode(nextThemeMode, $q.dark)
  saveThemeMode(appliedThemeMode)
}

async function handleLanguageChange(targetLocale) {
  if (languageSwitching.value) {
    return
  }

  const nextLocale = targetLocale || getTargetLocale(getCurrentLocale())
  if (nextLocale === getCurrentLocale()) {
    return
  }

  languageSwitching.value = true

  try {
    await applyLocale(nextLocale)
    const result = await switchLanguage(getDeviceLanguageCode(nextLocale))
    if (result?.success) {
      notifySuccess(result.message)
    } else {
      notifyInfo(result?.message || t("layout.language.notApplied"))
    }
  } catch (error) {
    notifyError(error)
  } finally {
    languageSwitching.value = false
  }
}

onMounted(async () => {
  document.body.addEventListener("mousedown", handleGlobalHiddenEntryClick)
  window.addEventListener(REMOTE_VERSION_REFRESH_EVENT, handleRemoteVersionRefreshRequest)
  const startupStartedAt = getPerfNow()
  logStartupPerf("mount-start", { route: route.path })

  try {
    await loadUpgradeCqDict()
    const portsTaskStartedAt = getPerfNow()
    const refreshPortsTask = deviceStore.refreshPorts().then((ports) => {
      logStartupPerf("refresh-ports-ok", {
        elapsedMs: Math.round(getPerfNow() - portsTaskStartedAt),
        count: ports.length,
      })
      return ports
    })
    const syncStatusTask = deviceStore.syncStatus().then((status) => {
      logStartupPerf("sync-status-ok", {
        elapsedMs: Math.round(getPerfNow() - portsTaskStartedAt),
        connected: status.connected,
        port: status.portName || "",
      })
      return status
    })

    await Promise.all([refreshPortsTask, syncStatusTask])
    logStartupPerf("device-ready", {
      totalMs: Math.round(getPerfNow() - startupStartedAt),
      portCount: deviceStore.ports.length,
      connected: deviceStore.connectionStatus,
    })

    if (typeof window !== "undefined" && window.__TAURI_INTERNALS__) {
      const watcherStartedAt = getPerfNow()
      unlistenPortWatcher = await deviceStore.startPortWatcher()
      logStartupPerf("watcher-ready", {
        elapsedMs: Math.round(getPerfNow() - watcherStartedAt),
      })
    }

    await nextTick()
    logStartupPerf("post-next-tick", {
      totalMs: Math.round(getPerfNow() - startupStartedAt),
    })
    startupHotplugPromptTimer = window.setTimeout(() => {
      startupHotplugPromptTimer = null
      logStartupPerf("queue-hotplug-prompt", {
        totalMs: Math.round(getPerfNow() - startupStartedAt),
        pendingPort: deviceStore.port || deviceStore.ports[0]?.value || "",
      })
      deviceStore.queueStartupHotplugPrompt()
    }, 180)
  } catch (error) {
    logStartupPerf("mount-fail", {
      totalMs: Math.round(getPerfNow() - startupStartedAt),
      error: String(error),
    })
    notifyError(error)
  }
})

watch(
  cqSyncKey,
  async (nextKey, previousKey) => {
    if (nextKey === previousKey) {
      return
    }

    const modelName = getGeneratorModelName()
    if (!modelName) {
      return
    }

    await syncPanelCqFromModel()
  },
)

watch(
  () => cqGeneratorForm.commType,
  (value) => {
    if (Number(value) !== 0x02) {
      cqGeneratorForm.frameType = 0x00
      cqGeneratorForm.fileFormat = 0x01
      cqGeneratorForm.frameId = 0x00
      cqGeneratorForm.specialFrameValue = ""
      return
    }

    if (![0x01, 0x02].includes(Number(cqGeneratorForm.frameType))) {
      cqGeneratorForm.frameType = 0x01
    }
    if (![0x01, 0x02].includes(Number(cqGeneratorForm.frameId))) {
      cqGeneratorForm.frameId = 0x01
    }
    cqGeneratorForm.fileFormat = 0x01
  },
  { immediate: true },
)

watch(
  () => cqGeneratorForm.frameId,
  (value) => {
    if (Number(value) !== 0x02) {
      cqGeneratorForm.specialFrameValue = ""
    }
  },
)

onUnmounted(() => {
  document.body.removeEventListener("mousedown", handleGlobalHiddenEntryClick)
  window.removeEventListener(REMOTE_VERSION_REFRESH_EVENT, handleRemoteVersionRefreshRequest)
  resetHiddenEntryCounter()
  resetDeveloperModeEntryCounter()

  if (startupHotplugPromptTimer) {
    window.clearTimeout(startupHotplugPromptTimer)
    startupHotplugPromptTimer = null
  }

  if (unlistenPortWatcher) {
    unlistenPortWatcher()
    unlistenPortWatcher = null
  }
})

watch(
  () => deviceStore.connectionStatus,
  async (status, previousStatus) => {
    if (status === previousStatus) {
      return
    }

    if (status === "CONNECTED") {
      await refreshRemoteVersion({ silent: true, retries: 2, retryDelayMs: 450 })
      return
    }
  },
)

watch(
  () => deviceStore.pendingHotplugPort,
  (port) => {
    if (!port || hotplugDialogOpen.value) return
    if (deviceStore.upgradeInProgress) {
      deviceStore.clearHotplugPending()
      return
    }

    logStartupPerf("hotplug-dialog-open", { port })
    hotplugDialogOpen.value = true
    $q.dialog({
      title: t("layout.dialog.hotplugTitle"),
      message: t("layout.dialog.hotplugMessage", { port }),
      cancel: {
        label: t("layout.dialog.hotplugLater"),
        flat: true,
      },
      ok: {
        label: t("layout.dialog.hotplugNow"),
        color: "primary",
        unelevated: true,
      },
      persistent: false,
    })
      .onOk(async () => {
        logStartupPerf("hotplug-dialog-ok", { port })
        hotplugDialogOpen.value = false
        deviceStore.clearHotplugPending()

        try {
          if (port) {
            deviceStore.port = port
          }
          logStartupPerf("hotplug-connect-start", { port })
          await deviceStore.toggleConnection("hotplug-dialog")
          logStartupPerf("hotplug-connect-ok", { port })
        } catch (error) {
          logStartupPerf("hotplug-connect-fail", { port, error: String(error) })
          notifyError(error)
        }
      })
      .onDismiss(() => {
        logStartupPerf("hotplug-dialog-dismiss", { port })
        hotplugDialogOpen.value = false
        deviceStore.clearHotplugPending()
      })
  }
)

async function refreshPorts() {
  try {
    await deviceStore.refreshPorts()
    notifySuccess(t("layout.device.portsRefreshed"))
  } catch (error) {
    notifyError(error)
  }
}

async function toggleConnection() {
  if (deviceStore.upgradeInProgress) {
    notifyInfo("升级进行中，暂不允许切换串口连接")
    return
  }

  const shouldConnect = deviceStore.connectionStatus !== "CONNECTED"
  if (shouldConnect) {
    if (!panelCqCode.value) {
      notifyError("请先输入 CQ 配置串")
      return
    }

    if (!syncDeviceConfigWithCqCode(panelCqCode.value, { notifyOnError: true })) {
      return
    }
  }

  connecting.value = true
  try {
    const status = await deviceStore.toggleConnection("device-panel-button")
    notifySuccess(status.connected ? t("layout.device.serialConnected") : t("layout.device.serialDisconnected"))
  } catch (error) {
    notifyError(error)
  } finally {
    connecting.value = false
  }
}
</script>

<style scoped lang="scss">
.app-shell {
  --shell-sidebar-width: 272px;
  --shell-header-height: 64px;
  --shell-header-surface: var(--dt-header-surface);
  --shell-divider-color: color-mix(in srgb, var(--dt-border-strong) 78%, transparent);
  --shell-gloss-surface: var(--dt-gloss-surface);
  --shell-gloss-surface-soft: var(--dt-gloss-surface-soft);
  --shell-gloss-surface-ghost: var(--dt-gloss-surface-ghost);
  --shell-gloss-active: var(--dt-gloss-active);
  --shell-gloss-border: var(--dt-gloss-border);
  --shell-gloss-border-strong: var(--dt-gloss-border-strong);
  --shell-gloss-shadow: var(--dt-gloss-shadow);
  --shell-gloss-shadow-soft: var(--dt-gloss-shadow-soft);
  --shell-gloss-shadow-strong: var(--dt-gloss-shadow-strong);
  --shell-gloss-inset: var(--dt-gloss-inset);
  --shell-gloss-blue-inset: var(--dt-gloss-blue-inset);
  position: relative;
  width: 100%;
  height: 100%;
  background: transparent;
  color: var(--dt-text-primary);
  border: 0;
  border-radius: var(--dt-radius-window);
  box-shadow:
    0 18px 40px color-mix(in srgb, var(--dt-shadow-window) 100%, transparent),
    0 6px 18px rgba(15, 23, 42, 0.08);
  overflow: hidden;
  clip-path: none;
  transform: none;
  isolation: isolate;
}

.app-shell::before {
  content: "";
  position: absolute;
  inset: 0;
  pointer-events: none;
  background-image:
    linear-gradient(rgba(199, 211, 228, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(199, 211, 228, 0.02) 1px, transparent 1px);
  background-position: 0 0, 0 0;
  background-size: 44px 44px, 44px 44px;
  mask-image: linear-gradient(180deg, rgba(0, 0, 0, 0.36), transparent 86%);
  opacity: 0.06;
  z-index: 0;
}

.app-shell::after {
  content: "";
  position: absolute;
  inset: 0;
  pointer-events: none;
  background:
    radial-gradient(circle at top left, color-mix(in srgb, var(--dt-text-contrast) 18%, transparent), transparent 20%),
    linear-gradient(180deg, color-mix(in srgb, var(--dt-text-contrast) 10%, transparent), transparent 14%);
  z-index: 0;
}

.shell-sidebar {
  position: absolute;
  top: 0;
  left: 0;
  background: transparent;
  color: var(--dt-text-primary);
  width: var(--shell-sidebar-width);
  min-width: var(--shell-sidebar-width);
  height: 100%;
  box-sizing: border-box;
  border-right: 1px solid transparent;
  overflow: hidden;
  z-index: 20;
}

.sidebar-surface {
  position: relative;
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--dt-bg-sidebar);
  overflow: hidden;
  box-shadow: none;
  z-index: 2;
}

.sidebar-surface::after {
  content: "";
  position: absolute;
  top: calc(var(--shell-header-height) + 10px);
  right: -1px;
  bottom: 18px;
  width: 1px;
  pointer-events: none;
  background: var(--shell-divider-color);
  z-index: 5;
}

.drawer-shell {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px 14px;
  border-right: 0;
  background: var(--dt-bg-sidebar);
  box-shadow: none;
}

.sidebar-main {
  flex: 1;
  min-height: 0;
  background: var(--dt-bg-sidebar);
  overflow: hidden;
}

.sidebar-scroll {
  height: 100%;
  background: var(--dt-bg-sidebar);
}

.sidebar-scroll__content {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-top: 8px;
  padding-right: 4px;
  background: var(--dt-bg-sidebar);
}

.sidebar-brand {
  flex: 0 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-height: 52px;
  padding: 12px 8px 10px;
  border-bottom: 0;
  position: relative;
  z-index: 6;
}

.sidebar-brand__mark {
  display: flex;
  flex-direction: column;
  gap: 0;
  min-width: 0;
  position: relative;
  z-index: 1;
}

.sidebar-brand__logo {
  display: block;
  width: 132px;
  height: auto;
  max-width: 100%;
  opacity: 1;
  visibility: visible;
  flex: 0 0 auto;
}

.lang-switch-enter-active,
.lang-switch-leave-active {
  transition: opacity 0.16s ease, transform 0.16s ease;
  will-change: opacity, transform;
}

.lang-switch-enter-from {
  opacity: 0;
  transform: translateY(5px);
}

.lang-switch-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.lang-switch-leave-from {
  opacity: 1;
  transform: translateY(0);
}

.lang-switch-leave-to {
  opacity: 0;
  transform: translateY(-5px);
}

.nav-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin: 0;
  padding: 0;
  background: transparent;
}

.nav-home {
  width: 100%;
  min-height: 40px;
  border: 1px solid transparent;
  border-radius: var(--dt-radius-subtle);
  background: transparent;
  font-size: 15px;
  font-weight: 700;
  transition: all 0.3s ease;
  position: relative;
  overflow: visible;
  box-shadow: none;
}

.nav-home :deep(.q-btn__content) {
  width: 100%;
  min-height: 40px;
  align-items: center;
  justify-content: flex-start;
  gap: 10px;
  padding: 0 12px;
  border-radius: inherit;
}

.nav-home :deep(.q-focus-helper) {
  display: none;
}

.nav-home::before,
.nav-home::after,
.nav-home :deep(.q-btn__content)::before,
.nav-home :deep(.q-btn__content)::after {
  border-radius: inherit;
}

.nav-home:not(.nav-home--active):hover {
  background: var(--dt-brand-primary-soft);
  border-color: transparent;
}

.nav-home:not(.nav-home--active) {
  color: var(--dt-text-secondary);
}

.nav-home :deep(.q-icon),
.nav-subitem :deep(.q-icon) {
  width: 20px;
  height: 20px;
  border-radius: 0;
  background: transparent;
  color: var(--dt-text-secondary);
  box-shadow: none;
  transition: color 0.3s ease;
}

.nav-home--active {
  border-color: transparent;
  border-radius: var(--dt-radius-subtle);
  box-shadow: var(--dt-shadow-panel);
}

.nav-home--active :deep(.q-icon) {
  color: #fff;
  background: transparent;
  box-shadow: none;
}

.nav-home.q-btn--push {
  transform: none !important;
}

.nav-section {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.nav-section__title {
  padding: 0 12px;
  color: var(--dt-text-secondary);
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.18em;
  text-transform: uppercase;
  min-height: auto;
  border: 0;
  border-radius: 0;
  background: transparent;
}

.nav-section__items {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.nav-subitem {
  display: inline-flex;
  align-items: center;
  gap: 14px;
  min-height: 44px;
  padding: 0 12px;
  border-radius: var(--dt-radius-subtle);
  border: 1px solid transparent;
  color: var(--dt-text-secondary);
  font-size: 14px;
  font-weight: 600;
  transition: background-color 0.18s ease, color 0.18s ease, border-color 0.18s ease;
}

.nav-subitem--active,
.nav-subitem:hover {
  background: rgba(25, 118, 210, 0.12);
  color: var(--dt-accent);
  border-color: transparent;
  box-shadow: none;
}

.nav-subitem--active :deep(.q-icon),
.nav-subitem:hover :deep(.q-icon) {
  color: var(--dt-accent);
}

.sidebar-bottom {
  flex: 0 0 auto;
  margin-top: 14px;
}

.device-dock {
  padding-top: 18px;
  border-top: 0;
}

.device-dock__settings {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 34px;
  height: 34px;
  border-radius: var(--dt-radius-subtle);
  color: var(--dt-text-secondary);
  background: var(--dt-gloss-surface-soft);
  border: 1px solid var(--dt-border);
  box-shadow: var(--dt-gloss-inset);
  transition: background-color 0.18s ease, color 0.18s ease, box-shadow 0.18s ease;
}

.device-dock__settings:hover,
.device-dock__settings--active {
  color: var(--dt-accent);
  background: var(--dt-gloss-surface);
  box-shadow:
    var(--dt-gloss-inset),
    var(--dt-shadow-panel);
}

.device-connect-panel {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: 0;
  padding: 14px;
  min-height: 0;
  height: auto;
  border-radius: 12px;
  background: var(--dt-gloss-surface);
  border: 1px solid var(--dt-gloss-border);
  box-shadow:
    var(--dt-gloss-inset),
    var(--dt-shadow-panel);
  isolation: isolate;
}

.device-connect-panel::before {
  content: "";
  position: absolute;
  inset: 0;
  pointer-events: none;
  background: linear-gradient(180deg, color-mix(in srgb, var(--dt-text-contrast) 10%, transparent), transparent 22%);
}

:global(html[data-theme="dark"]) .device-connect-panel,
:global(html.theme-dark) .device-connect-panel,
:global(body.body--dark) .device-connect-panel,
:global(body.theme-dark) .device-connect-panel,
:global(body[data-theme="dark"]) .device-connect-panel {
  background: linear-gradient(180deg, rgba(28, 23, 40, 0.96), rgba(20, 17, 30, 0.94));
  border: 1px solid rgba(255, 255, 255, 0.12);
  box-shadow:
    inset 0 1px 0 rgba(255, 255, 255, 0.08),
    0 18px 40px rgba(0, 0, 0, 0.32),
    0 0 0 1px rgba(110, 124, 255, 0.06);
}

:global(html[data-theme="dark"]) .device-connect-panel::before,
:global(html.theme-dark) .device-connect-panel::before,
:global(body.body--dark) .device-connect-panel::before,
:global(body.theme-dark) .device-connect-panel::before,
:global(body[data-theme="dark"]) .device-connect-panel::before {
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.08), transparent 26%),
    radial-gradient(circle at top left, rgba(110, 124, 255, 0.12), transparent 38%);
}

.device-connect-panel__head,
.device-connect-panel__actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.device-connect-panel__head {
  padding-bottom: 12px;
}

.device-connect-panel__head h3 {
  margin: 0;
  font-size: 16px;
  line-height: 1.2;
  font-weight: 800;
  color: var(--dt-text-primary);
}

.device-connect-panel__body {
  flex: 1 1 auto;
  min-height: 0;
  padding: 0 0 12px;
}

.device-connect-panel__version {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  padding: 4px 0 10px;
}

.device-connect-panel__version-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 4px;
  min-width: 104px;
  padding: 4px 8px 4px 12px;
  border-radius: 10px;
  background: color-mix(in srgb, var(--dt-bg-panel) 72%, white 28%);
  border: 1px solid color-mix(in srgb, var(--dt-border) 86%, transparent);
}

.device-connect-panel__version span {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  line-height: 1;
  font-weight: 800;
  color: var(--dt-text-secondary);
}

.device-connect-panel__version-entry {
  cursor: pointer;
  user-select: none;
}

.device-connect-panel__version-refresh {
  color: var(--dt-text-secondary);
  min-width: 22px;
  min-height: 22px;
}

.device-connect-panel__actions {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  margin-top: auto;
  gap: 10px;
  padding-top: 12px;
}

.device-connect-panel__actions :deep(.q-btn) {
  font-weight: 800;
  border-radius: var(--dt-radius-button);
}

.device-connect-panel__actions :deep(.device-connect-panel__action-secondary) {
  color: #131623 !important;
}

.device-connect-panel__actions :deep(.device-connect-panel__action-secondary .q-btn__content) {
  color: #131623 !important;
}

.device-connect-panel__actions :deep(.device-connect-panel__action-primary) {
  color: #fff !important;
}

.device-connect-panel__actions :deep(.device-connect-panel__action-primary.device-connect-panel__action-primary--disconnect) {
  background: linear-gradient(135deg, #ff7a7a, #ef4444) !important;
  border-color: rgba(239, 68, 68, 0.42) !important;
  box-shadow: 0 12px 28px rgba(239, 68, 68, 0.28) !important;
}

.device-connect-panel__actions :deep(.device-connect-panel__action-primary.device-connect-panel__action-primary--disconnect:hover) {
  background: linear-gradient(135deg, #ff8d8d, #f05252) !important;
}

:global(html[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary),
:global(html.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary),
:global(body.body--dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary),
:global(body.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary),
:global(body[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary) {
  background: linear-gradient(180deg, rgba(248, 250, 255, 0.92), rgba(226, 231, 243, 0.88)) !important;
  border: 1px solid rgba(255, 255, 255, 0.2) !important;
  color: #131623 !important;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.24) !important;
}

:global(html[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary .q-btn__content),
:global(html.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary .q-btn__content),
:global(body.body--dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary .q-btn__content),
:global(body.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary .q-btn__content),
:global(body[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary .q-btn__content) {
  color: #131623 !important;
}

:global(html[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-primary),
:global(html.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-primary),
:global(body.body--dark) .device-connect-panel__actions :deep(.device-connect-panel__action-primary),
:global(body.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-primary),
:global(body[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-primary) {
  background: linear-gradient(135deg, rgba(110, 124, 255, 0.92), rgba(88, 103, 232, 0.92)) !important;
  border: 1px solid rgba(164, 173, 255, 0.22) !important;
  color: #ffffff !important;
  box-shadow: 0 12px 28px rgba(78, 89, 214, 0.28) !important;
}

:global(html[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-primary.device-connect-panel__action-primary--disconnect),
:global(html.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-primary.device-connect-panel__action-primary--disconnect),
:global(body.body--dark) .device-connect-panel__actions :deep(.device-connect-panel__action-primary.device-connect-panel__action-primary--disconnect),
:global(body.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-primary.device-connect-panel__action-primary--disconnect),
:global(body[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-primary.device-connect-panel__action-primary--disconnect) {
  background: linear-gradient(135deg, rgba(255, 122, 122, 0.96), rgba(220, 38, 38, 0.92)) !important;
  border: 1px solid rgba(255, 160, 160, 0.22) !important;
  color: #ffffff !important;
  box-shadow: 0 12px 28px rgba(220, 38, 38, 0.3) !important;
}

:global(html[data-theme="dark"]) .device-connect-panel__actions :deep(.q-btn.q-btn--disabled),
:global(html.theme-dark) .device-connect-panel__actions :deep(.q-btn.q-btn--disabled),
:global(body.body--dark) .device-connect-panel__actions :deep(.q-btn.q-btn--disabled),
:global(body.theme-dark) .device-connect-panel__actions :deep(.q-btn.q-btn--disabled),

:global(html[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled),
:global(html.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled),
:global(body.body--dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled),
:global(body.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled),
:global(body[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled) {
  background: linear-gradient(180deg, rgba(241, 244, 250, 0.78), rgba(215, 221, 235, 0.72)) !important;
  color: rgba(45, 52, 70, 0.4) !important;
  border-color: rgba(255, 255, 255, 0.14) !important;
}

:global(html[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled .q-btn__content),
:global(html.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled .q-btn__content),
:global(body.body--dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled .q-btn__content),
:global(body.theme-dark) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled .q-btn__content),
:global(body[data-theme="dark"]) .device-connect-panel__actions :deep(.device-connect-panel__action-secondary.q-btn--disabled .q-btn__content) {
  color: rgba(45, 52, 70, 0.4) !important;
}

:global(html[data-theme="dark"]) .device-connect-panel__version-badge,
:global(html.theme-dark) .device-connect-panel__version-badge,
:global(body.body--dark) .device-connect-panel__version-badge,
:global(body.theme-dark) .device-connect-panel__version-badge,
:global(body[data-theme="dark"]) .device-connect-panel__version-badge {
  background: rgba(255, 255, 255, 0.06);
  border-color: rgba(255, 255, 255, 0.1);
}

:global(html[data-theme="dark"]) .device-connect-panel__version span,
:global(html.theme-dark) .device-connect-panel__version span,
:global(body.body--dark) .device-connect-panel__version span,
:global(body.theme-dark) .device-connect-panel__version span,
:global(body[data-theme="dark"]) .device-connect-panel__version span,
:global(html[data-theme="dark"]) .device-connect-panel__version-refresh,
:global(html.theme-dark) .device-connect-panel__version-refresh,
:global(body.body--dark) .device-connect-panel__version-refresh,
:global(body.theme-dark) .device-connect-panel__version-refresh,
:global(body[data-theme="dark"]) .device-connect-panel__version-refresh {
  color: rgba(237, 242, 255, 0.9);
}

.device-connect-panel__actions > :deep(.q-btn) {
  flex: 1 1 0;
  width: calc((100% - 10px) / 2);
  max-width: calc((100% - 10px) / 2);
  min-width: 0;
}

.device-connect-panel__actions :deep(.q-btn__content) {
  white-space: nowrap;
}

.shell-header {
  position: absolute;
  top: 0;
  left: var(--shell-sidebar-width);
  right: 0;
  min-height: var(--shell-header-height);
  background: var(--shell-header-surface);
  color: var(--dt-text-primary);
  backdrop-filter: none;
  border-bottom: 1px solid var(--shell-divider-color);
  box-shadow: none;
  border-top-left-radius: 0;
  border-top-right-radius: var(--dt-radius-window);
  z-index: 30;
}

.shell-header::after {
  content: "";
  position: absolute;
  top: 0;
  left: -2px;
  bottom: 0;
  width: 3px;
  background: var(--shell-header-surface);
  pointer-events: none;
  z-index: 1;
}

.shell-header--config {
  overflow: visible;
}

.shell-header--config::before {
  content: none;
}

.shell-header__actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
  margin-left: 12px;
  position: relative;
  z-index: 2;
  pointer-events: auto;
  flex: 0 0 auto;
  contain: layout paint;
  transform: translateZ(0);
  backface-visibility: hidden;
}

.shell-lang-switch {
  width: 40px;
  min-width: 40px;
  height: 40px;
  min-height: 40px;
  border-radius: 999px;
  border: 1px solid var(--dt-border);
  background: var(--dt-brand-primary-soft);
  color: var(--dt-text-primary);
}

.shell-lang-switch :deep(.q-btn__content),
.shell-lang-switch :deep(.q-icon) {
  color: inherit !important;
}

.shell-theme-switch {
  width: 40px;
  min-width: 40px;
  height: 40px;
  min-height: 40px;
  border-radius: 999px;
  border: 1px solid var(--dt-border);
  background: var(--dt-bg-panel-soft);
}

.shell-theme-switch :deep(.q-btn__content),
.shell-theme-switch :deep(.q-icon) {
  color: inherit !important;
}

.shell-theme-switch:hover {
  background: var(--dt-brand-primary-soft);
  color: var(--dt-text-primary);
}

.shell-lang-switch__label {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 0;
  font-size: 14px;
  font-weight: 700;
  line-height: 1;
}

.shell-window-actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 8px;
  position: relative;
  z-index: 2;
  pointer-events: auto;
  flex: 0 0 auto;
  contain: layout paint;
}

.shell-window-action {
  min-width: 30px;
  min-height: 30px;
  color: var(--dt-text-secondary);
  border-radius: var(--dt-radius-button);
}

.shell-window-action :deep(.q-focus-helper) {
  display: none;
}

.shell-window-action :deep(.q-btn__content) {
  min-width: 18px;
  min-height: 18px;
  color: inherit !important;
}

.shell-window-action :deep(.q-icon) {
  font-size: 18px;
  color: inherit !important;
}

.shell-window-action:hover {
  background: var(--dt-brand-primary-soft);
  color: var(--dt-text-primary);
}

.shell-window-action--close:hover {
  background: var(--dt-status-danger-soft);
  color: var(--dt-danger);
}

.shell-window-action.q-btn--disabled {
  opacity: 0.5 !important;
}

.shell-toolbar {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  gap: 12px;
  min-height: var(--shell-header-height);
  padding: 0 18px;
  position: relative;
}

.shell-header__config-bar {
  display: flex;
  flex: 1;
  width: 100%;
  align-items: center;
  gap: 12px;
  min-height: var(--shell-header-height);
  padding: 0 18px;
  background: transparent;
  backdrop-filter: none;
  position: relative;
}

.shell-header__config-host {
  min-width: 0;
  margin: 0;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  pointer-events: none;
}

.shell-toolbar__brand--config {
  flex: 0 0 auto;
}

.shell-toolbar__brand {
  display: flex;
  align-items: center;
  align-self: stretch;
  gap: 10px;
  min-width: 0;
  padding: 0;
  flex:1;
  border-radius: 0;
  background: transparent;
  border: 0;
  box-shadow: none;
}

.shell-toolbar__brand--drag {
  cursor: grab;
  user-select: none;
}

.shell-toolbar__page-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: auto;
  height: auto;
  line-height: 1;
  border-radius: 0;
  background: transparent;
  color: #2e7df1;
  box-shadow: none;
  flex: 0 0 auto;
}

.shell-toolbar__page-host {
  flex: 0 1 auto;
  min-width: 0;
  display: flex;
  align-items: center;
  justify-content: flex-start;
  position: relative;
  z-index: 2;
  pointer-events: none;
  margin-left: auto;
}

.shell-toolbar__page-host > :deep(*) {
  pointer-events: auto;
}

.shell-toolbar__title {
  display: flex;
  align-items: center;
  align-self: stretch;
  min-width: 0;
  user-select: none;
}

.shell-toolbar__title--inline {
  flex: 0 0 auto;
  padding-right: 12px;
}

.shell-toolbar__title strong {
  display: block;
  font-size: 17px;
  font-weight: 800;
  line-height: 1.1;
  color: #17355e;
  cursor: pointer;
}

.shell-drag-region {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 28px;
  z-index: 1;
  cursor: grab;
  user-select: none;
}

.shell-drag-region--full {
  bottom: 0;
  height: auto;
}

.shell-toolbar__brand,
.shell-header__config-host {
  position: relative;
  z-index: 2;
}

.shell-frame {
  position: relative;
  display: flex;
  flex-direction: column;
  height: calc(100% - var(--shell-header-height));
  min-height: 0;
  margin-top: var(--shell-header-height);
  padding-left: var(--shell-sidebar-width);
  box-sizing: border-box;
}

.shell-frame-shadow {
  position: absolute;
  top: 0;
  bottom: 0;
  left: 0;
  width: var(--shell-sidebar-width);
  pointer-events: none;
  background: transparent;
  box-shadow: none;
  z-index: 2;
}

.shell-page-container {
  position: relative;
  display: flex;
  flex-direction: column;
  flex: 1 1 auto;
  height: 100%;
  min-height: 0;
  background: var(--dt-bg-shell);
  border-left: 0;
  border-bottom-right-radius: var(--dt-radius-window);
}

.shell-page-container::before {
  content: none;
}

.shell-page-container--full {
  height: 100%;
  overflow: auto;
}

.shell-header--config {
  background: var(--shell-header-surface);
  border-bottom: 1px solid var(--dt-header-border);
  box-shadow: none;
}

.shell-header__config-layout {
  display: flex;
  align-items: center;
  min-height: var(--shell-header-height);
}

.shell-page {
  display: flex;
  flex: 1 1 auto;
  height: 100%;
  min-height: 0;
  min-width: 0;
  overflow: hidden;
  position: relative;
  z-index: 1;
}

.content {
  display: flex;
  flex-direction: column;
  flex: 1;
  height: 100%;
  min-height: 0;
  width: 100%;
  padding: 0;
  border: 0;
  border-radius: 0;
  background: transparent;
  box-shadow: none;
  overflow: hidden;
}

.content--home,
.content--config,
.content--settings {
  gap: 12px;
}

.content--home {
  padding: 0;
}

.content--config {
  padding: 0;
  height: 100%;
  min-height: 0;
  overflow: hidden;
  background: transparent;
}

.content--settings {
  padding: 0;
}

.drawer-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.drawer-field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.drawer-field label {
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 800;
}

.drawer-field :deep(.q-field__control) {
  background: var(--dt-bg-input);
  box-shadow: var(--dt-gloss-inset);
}

.drawer-field :deep(.q-field--outlined .q-field__control:before) {
  border-color: var(--dt-border);
}

.drawer-field :deep(.q-field--outlined.q-field--focused .q-field__control:after),
.drawer-field :deep(.q-field--outlined.q-field--highlighted .q-field__control:after) {
  border-color: var(--dt-brand-primary);
}

.drawer-field :deep(.q-field--dense .q-field__control),
.drawer-field :deep(.q-field--dense .q-field__native),
.drawer-field :deep(.q-field--dense .q-field__marginal) {
  min-height: 36px;
}

.drawer-field :deep(.q-field--outlined .q-field__control) {
  height: 36px;
}

.drawer-field :deep(.q-field--dense .q-field__control-container) {
  padding-top: 0;
}

.drawer-field :deep(.q-field--dense .q-field__native),
.drawer-field :deep(.q-field--dense .q-field__input) {
  padding-top: 0;
  padding-bottom: 0;
}

.drawer-help {
  color: var(--dt-text-muted);
  font-size: 12px;
  line-height: 1.6;
}

.shell-toolbar__brand :deep(.q-btn) {
  min-height: 38px;
  border-radius: var(--dt-radius-button);
}

.shell-toolbar__brand :deep(.q-btn--flat) {
  color: var(--dt-text-primary);
}

.shell-toolbar__brand :deep(.q-btn--flat:hover) {
  background: var(--dt-brand-primary-soft);
}

.shell-toolbar__brand :deep(.q-btn[round]) {
  color: var(--dt-header-muted);
}

.shell-sidebar {
  background: transparent !important;
  color: var(--dt-text-primary) !important;
  border-right-color: var(--shell-divider-color) !important;
  box-shadow: none !important;
  overflow: visible !important;
}

.shell-sidebar .sidebar-surface {
  background: var(--dt-bg-sidebar) !important;
  box-shadow: none !important;
}

.shell-sidebar .drawer-shell {
  background: var(--dt-bg-sidebar) !important;
  border-right-color: var(--shell-divider-color);
  position: relative;
  z-index: 2;
}

.shell-sidebar .sidebar-brand,
.shell-sidebar .sidebar-main,
.shell-sidebar .sidebar-bottom,
.shell-sidebar .sidebar-scroll,
.shell-sidebar .sidebar-scroll__content {
  background: var(--dt-bg-sidebar) !important;
}

.shell-sidebar .sidebar-brand {
  background: var(--dt-bg-sidebar) !important;
  border: 0;
  box-shadow: none;
}

.shell-sidebar :deep(.q-scrollarea__content),
.shell-sidebar :deep(.q-scrollarea__container),
.shell-sidebar :deep(.q-scrollarea__viewport) {
  background: var(--dt-bg-sidebar) !important;
}

.shell-sidebar .sidebar-brand {
  padding-bottom: 16px;
  border-bottom: 0;
}

.shell-sidebar .sidebar-brand .q-icon {
  color: var(--dt-text-secondary);
}

.shell-sidebar .sidebar-brand__logo {
  filter: none;
}

.shell-sidebar .nav-section {
  gap: 12px;
}

.shell-sidebar .nav-section__title {
  padding: 0 12px;
  color: var(--dt-text-secondary);
  font-weight: 700;
  border: 0;
  border-radius: 0;
}

.shell-sidebar .nav-section__title--active {
  color: var(--dt-text-secondary);
  background: transparent;
  border-color: transparent;
  box-shadow: none;
}

.shell-sidebar .nav-section__items {
  margin-left: 0;
  padding-left: 0;
  border-left: 0;
}

.shell-sidebar .nav-subitem {
  min-height: 44px;
  padding: 0 12px;
  color: var(--dt-text-secondary);
  border: 1px solid transparent;
  border-radius: var(--dt-radius-subtle);
}

.shell-sidebar .nav-subitem--active,
.shell-sidebar .nav-subitem:hover {
  background: var(--dt-bg-panel);
  color: var(--dt-text-primary);
  border-color: var(--dt-border);
  box-shadow: none;
}

.shell-sidebar .nav-subitem__dot {
  opacity: 0.9;
  background: var(--dt-text-secondary);
}

.shell-sidebar--config {
  background: transparent !important;
  color: var(--dt-text-primary) !important;
  border-right: 1px solid var(--shell-divider-color) !important;
  box-shadow: none !important;
  overflow: visible !important;
}

.shell-sidebar--config .sidebar-surface {
  background: var(--dt-bg-sidebar) !important;
}

.shell-sidebar--config .sidebar-brand,
.shell-sidebar--config .drawer-shell,
.shell-sidebar--config .sidebar-main,
.shell-sidebar--config .sidebar-bottom,
.shell-sidebar--config .sidebar-scroll,
.shell-sidebar--config .sidebar-scroll__content {
  background: var(--dt-bg-sidebar) !important;
  border-right-color: var(--shell-divider-color);
}

.shell-sidebar--config .sidebar-brand {
  background: var(--dt-bg-sidebar) !important;
  border: 0;
  box-shadow: none;
}

.shell-sidebar--config .sidebar-brand {
  border-bottom-color: transparent;
}

.shell-sidebar--config .sidebar-brand__tag,
.shell-sidebar--config .sidebar-brand .q-icon {
  color: var(--dt-text-secondary);
}

.shell-sidebar--config .sidebar-brand__logo {
  filter: none;
}

.shell-sidebar--config .nav-section__title {
  color: var(--dt-text-secondary);
  border-color: transparent;
  background: transparent;
}

.shell-sidebar--config .nav-section__title:hover,
.shell-sidebar--config .nav-section__title--active {
  color: var(--dt-text-secondary);
  background: transparent;
  border-color: transparent;
  box-shadow: none;
}

.shell-sidebar--config .nav-section__title--active::before {
  background: transparent;
}

.shell-sidebar--config .nav-section__items {
  border-left-color: transparent;
}

.shell-sidebar--config .nav-subitem {
  color: var(--dt-text-secondary);
  background: transparent;
}

.shell-sidebar--config .nav-subitem--active,
.shell-sidebar--config .nav-subitem:hover {
  background: var(--dt-brand-primary-soft);
  color: var(--dt-accent);
  border-color: transparent;
  box-shadow: none;
}

.shell-sidebar--config .nav-subitem__dot {
  background: var(--dt-text-secondary);
}

.shell-sidebar--config .nav-subitem--active .nav-subitem__dot,
.shell-sidebar--config .nav-subitem:hover .nav-subitem__dot {
  background: var(--dt-accent);
}

.shell-sidebar--config .device-dock {
  border-top-color: rgba(148, 163, 184, 0.12);
}

.shell-sidebar .nav-subitem--active .nav-subitem__dot,
.shell-sidebar .nav-subitem:hover .nav-subitem__dot {
  background: var(--dt-accent);
}

.shell-sidebar .device-dock {
  border-top: 1px solid transparent;
}

@media (max-width: 920px) {
  .shell-sidebar {
    display: none;
  }

  .shell-frame-shadow {
    display: none;
  }

  .shell-page-container {
    padding: 12px;
    height: 100%;
  }

  .shell-page-container--full {
    height: 100%;
  }

  .content {
    padding: 0;
  }

  .content--config {
    padding-left: 0;
    padding-bottom: 0;
  }

  .content--settings {
    padding: 0;
  }

  .shell-header {
    left: 0;
  }

  .shell-frame {
    padding-left: 0;
  }
}

@media (max-width: 1280px) {
  .shell-header__config-layout,
  .shell-header__config-bar {
    min-height: auto;
  }

  .shell-header__config-bar {
    padding-top: 10px;
    padding-bottom: 10px;
    align-items: flex-start;
  }
}

:global(html[data-theme="dark"]) .shell-header,
:global(html.theme-dark) .shell-header,
:global(body.body--dark) .shell-header,
:global(body.theme-dark) .shell-header,
:global(body[data-theme="dark"]) .shell-header {
  border-bottom-color: color-mix(in srgb, var(--dt-accent) 34%, var(--dt-border-strong)) !important;
}

:global(html[data-theme="light"]) .shell-header,
:global(html.theme-light) .shell-header,
:global(body.body--light) .shell-header,
:global(body.theme-light) .shell-header,
:global(body[data-theme="light"]) .shell-header {
  border-bottom-color: color-mix(in srgb, var(--dt-accent) 14%, var(--dt-border-strong)) !important;
}

:global(html[data-theme="dark"]) .app-shell,
:global(html.theme-dark) .app-shell,
:global(body.body--dark) .app-shell,
:global(body.theme-dark) .app-shell,
:global(body[data-theme="dark"]) .app-shell {
  --shell-divider-color: color-mix(in srgb, var(--dt-accent) 34%, var(--dt-border-strong));
}

:global(html[data-theme="light"]) .app-shell,
:global(html.theme-light) .app-shell,
:global(body.body--light) .app-shell,
:global(body.theme-light) .app-shell,
:global(body[data-theme="light"]) .app-shell {
  --shell-divider-color: color-mix(in srgb, var(--dt-accent) 14%, var(--dt-border-strong));
}

:global(html[data-theme="dark"]) .shell-header--config::before,
:global(html.theme-dark) .shell-header--config::before,
:global(body.body--dark) .shell-header--config::before,
:global(body.theme-dark) .shell-header--config::before,
:global(body[data-theme="dark"]) .shell-header--config::before {
  border-bottom-color: transparent !important;
}

.cq-dialog {
  width: min(520px, 92vw);
  border-radius: 12px;
  background: var(--dt-gloss-surface);
  color: var(--dt-text-primary);
  box-shadow: var(--dt-shadow-float);
}

.cq-dialog--generator {
  width: min(760px, 94vw);
}

.cq-dialog__section {
  padding: 18px 20px;
}

.cq-dialog__section--compact {
  padding-top: 0;
}

.cq-dialog__title {
  font-size: 18px;
  font-weight: 800;
  line-height: 1.3;
}

.cq-dialog__desc {
  margin-top: 8px;
  color: var(--dt-text-secondary);
  font-size: 13px;
  line-height: 1.6;
}

.cq-dialog__transport-tabs {
  width: 280px;
  margin: 0 auto;
}

.cq-dialog__transport-tabs :deep(.q-btn) {
  min-height: 38px;
  font-size: 13px;
  font-weight: 800;
  letter-spacing: 0.04em;
}

.cq-dialog__grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 14px;
}

.cq-dialog__field--full {
  grid-column: 1 / -1;
}

.cq-dialog__preview-input :deep(.q-field__append) {
  gap: 6px;
  padding-left: 10px;
}

.cq-dialog__actions {
  padding: 0 20px 18px;
}

@media (max-width: 760px) {
  .cq-dialog__grid {
    grid-template-columns: minmax(0, 1fr);
  }
}
</style>

<style lang="scss">
body.platform-mac .app-shell {
  --shell-header-surface: var(--dt-header-surface);
}

body.platform-mac .shell-header {
  background: var(--shell-header-surface);
  color: var(--dt-text-primary);
  border-bottom: 1px solid var(--shell-divider-color);
}

body.platform-mac .shell-header--config {
  background: var(--shell-header-surface);
}

body.platform-mac .shell-toolbar,
body.platform-mac .shell-header__config-bar {
  padding-left: 18px;
}

body.platform-mac .shell-toolbar .q-btn--flat {
  color: var(--dt-text-primary);
}

body.platform-mac .shell-toolbar .q-btn--flat:hover {
  background: rgba(26, 115, 232, 0.06);
}

body.platform-mac .shell-toolbar .q-btn[round] {
  color: var(--dt-header-muted);
}
</style>
