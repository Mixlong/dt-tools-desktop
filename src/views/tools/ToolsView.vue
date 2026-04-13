<template>
  <q-page class="tools-page column no-wrap">
    <q-card flat class="tools-tabs">
      <div class="tools-tabs__topbar">
        <q-tabs
          v-model="tab"
          dense
          no-caps
          inline-label
          active-color="primary"
          indicator-color="primary"
        >
          <q-tab
            v-for="item in tabOptions"
            :key="item.value"
            :name="item.value"
            :icon="item.icon"
            :label="item.label"
          />
        </q-tabs>
      </div>
    </q-card>

    <q-tab-panels v-model="tab" class="bg-transparent col overflow-hidden">
      <q-tab-panel name="can" class="tools-panel q-pa-none fit overflow-hidden">
          <q-card flat bordered class="panel tools-surface fit column no-wrap">
          <q-card-section class="tools-toolbar">
            <div class="tools-toolbar__row">
              <div class="tools-toolbar__filters">
                <div class="tools-toolbar__filter tools-toolbar__filter--id">
                  <div class="tools-filter-field">
                    <label class="tools-filter-field__label">{{ t("tools.can.idRange") }}</label>
                    <q-input
                      v-model="canFilters.idRange"
                      outlined
                      dense
                      class="tools-filter-field__input"
                      :placeholder="t('tools.can.idRangePlaceholder')"
                      hide-bottom-space
                    />
                  </div>
                </div>

                <div class="tools-toolbar__filter tools-toolbar__filter--mask">
                  <div class="tools-filter-field">
                    <label class="tools-filter-field__label">{{ t("tools.can.bitmask") }}</label>
                    <q-input
                      v-model="canFilters.bitmask"
                      outlined
                      dense
                      class="tools-filter-field__input"
                      :placeholder="t('tools.can.bitmaskPlaceholder')"
                      hide-bottom-space
                    />
                  </div>
                </div>
              </div>
              <div class="tools-toolbar__actions">
                <q-btn class="tools-action-btn" :label="t('tools.can.applyFilter')" color="primary" outline icon="filter_alt" dense no-caps />
                <q-btn
                  class="tools-action-btn"
                  :label="isCanCapturing ? t('tools.can.pause') : t('tools.can.startCapture')"
                  :color="isCanCapturing ? 'warning' : 'positive'"
                  :icon="isCanCapturing ? 'pause' : 'play_arrow'"
                  unelevated
                  dense
                  no-caps
                  @click="toggleCanCapture"
                />
                <q-btn class="tools-action-btn" :label="t('tools.can.stop')" color="negative" icon="stop" unelevated dense no-caps @click="stopCanCapture" />
                <q-btn class="tools-action-btn" :label="t('tools.can.clearView')" color="grey-7" text-color="white" icon="delete_outline" flat dense no-caps @click="clearCanMessages" />
              </div>
            </div>
          </q-card-section>
 
          <q-separator />
 
          <q-scroll-area class="col">
            <q-table
              :rows="canMessages"
              :columns="canColumns"
              row-key="id"
              flat
              bordered
              square
              dense
              separator="cell"
              :pagination="{ rowsPerPage: 0 }"
              hide-pagination
            >
              <template #body-cell-data="props">
                <q-td :props="props">
                  <span class="text-weight-medium">{{ props.value }}</span>
                </q-td>
              </template>
              <template #no-data>
                <div class="full-width row flex-center q-gutter-sm q-pa-lg text-grey-7">
                  <q-icon name="inbox" size="sm" />
                  <span>{{ t("tools.can.noData") }}</span>
                </div>
              </template>
            </q-table>
          </q-scroll-area>
        </q-card>
      </q-tab-panel>

      <q-tab-panel name="serial" class="tools-panel q-pa-none fit overflow-hidden">
        <q-card flat bordered class="panel tools-surface fit column no-wrap">
          <q-card-section class="row q-col-gutter-md q-row-gutter-sm items-center q-pa-md">
            <div class="col-12 col-md">
              <div class="text-subtitle1 text-weight-medium">{{ t("tools.serial.title") }}</div>
              <div class="text-caption text-grey-7">{{ t("tools.serial.description") }}</div>
            </div>
            <div class="col-12 col-md-auto row q-gutter-sm">
              <q-chip square dense :color="isSerialConnected ? 'positive' : 'grey-6'" text-color="white" icon="usb">
                {{ isSerialConnected ? t("common.status.connected") : t("common.status.disconnected") }}
              </q-chip>
              <q-chip square dense color="primary" text-color="white" icon="tune">
                {{ serialSummary }}
              </q-chip>
            </div>
          </q-card-section>
 
          <q-separator />
 
          <q-card-section class="q-pa-sm">
            <div class="row justify-end">
              <q-btn :label="t('tools.serial.clearLog')" color="grey-7" text-color="white" flat icon="delete_outline" @click="clearSerialLog" />
            </div>
          </q-card-section>
 
          <q-separator />
 
          <q-scroll-area class="col">
            <q-list bordered separator>
              <q-item v-for="(log, index) in serialLogs" :key="`${log.time}-${index}`">
                <q-item-section side class="text-caption text-grey-7">{{ log.time }}</q-item-section>
                <q-item-section side>
                  <q-chip
                    square
                    dense
                    :color="log.direction === 'TX' ? 'primary' : log.direction === 'RX' ? 'positive' : 'grey-7'"
                    text-color="white"
                  >
                    {{ log.direction }}
                  </q-chip>
                </q-item-section>
                <q-item-section>{{ log.content }}</q-item-section>
              </q-item>
              <q-item v-if="serialLogs.length === 0">
                <q-item-section class="text-grey-7">{{ t("tools.serial.waiting") }}</q-item-section>
              </q-item>
            </q-list>
          </q-scroll-area>
 
          <q-separator />
 
          <q-card-actions class="q-pa-md">
            <div class="row full-width q-col-gutter-md items-center">
              <div class="col">
                <q-input
                  v-model="serialInput"
                  outlined
                  dense
                  :placeholder="t('tools.serial.inputPlaceholder')"
                  :disable="!isSerialConnected"
                  hide-bottom-space
                  @keyup.enter="sendSerialData"
                />
              </div>
              <div class="col-auto">
                <q-btn :label="t('common.actions.send')" color="primary" unelevated :disable="!isSerialConnected" @click="sendSerialData" />
              </div>
            </div>
          </q-card-actions>
        </q-card>
      </q-tab-panel>

      <q-tab-panel name="wiring" class="tools-panel q-pa-none">
        <div class="row q-col-gutter-md q-row-gutter-md">
          <div v-for="guide in wiringGuides" :key="guide.title" class="col-12 col-sm-6 col-lg-4">
            <q-card flat bordered class="panel tools-guide-card full-height column">
              <q-img :src="guide.image" :ratio="16 / 9" />
              <q-card-section>
                <div class="text-subtitle2 text-weight-medium">{{ guide.title }}</div>
                <div class="text-caption text-grey-7 q-mt-sm">{{ guide.description }}</div>
              </q-card-section>
              <q-separator inset />
              <q-list dense>
                <q-item v-for="(step, idx) in guide.steps" :key="idx">
                  <q-item-section avatar>
                    <q-avatar color="primary" text-color="white" size="sm">{{ idx + 1 }}</q-avatar>
                  </q-item-section>
                  <q-item-section>{{ step }}</q-item-section>
                </q-item>
              </q-list>
              <q-separator />
              <q-card-actions align="right">
                <q-btn flat color="primary" :label="t('tools.wiring.detail')" />
              </q-card-actions>
            </q-card>
          </div>
        </div>
      </q-tab-panel>
    </q-tab-panels>

    <q-card flat bordered class="panel tools-status-card">
      <q-card-section class="row q-col-gutter-md q-row-gutter-sm items-center">
        <template v-if="tab === 'can'">
          <div class="col-12 col-md-auto">
            <q-chip square dense :color="isCanCapturing ? 'positive' : 'grey-6'" text-color="white">
              {{ t("tools.can.state", { state: isCanCapturing ? t("tools.can.capturing") : t("tools.can.stopped") }) }}
            </q-chip>
          </div>
          <div class="col-12 col-md-auto">
            <q-chip square dense color="primary" text-color="white">
              {{ t("tools.can.rate", { value: msgRate }) }}
            </q-chip>
          </div>
          <div class="col-12 col-md-auto">
            <q-chip square dense color="warning" text-color="black">
              {{ t("tools.can.load", { value: busLoad }) }}
            </q-chip>
          </div>
        </template>

        <template v-else-if="tab === 'serial'">
          <div class="col-12 col-md-auto">
            <q-chip square dense :color="isSerialConnected ? 'positive' : 'grey-6'" text-color="white">
              {{ t("tools.serial.status", { value: isSerialConnected ? t("common.status.connected") : t("common.status.disconnected") }) }}
            </q-chip>
          </div>
          <div class="col-12 col-md-auto">
            <q-chip square dense color="primary" text-color="white">
              {{ t("tools.serial.params", { value: serialSummary }) }}
            </q-chip>
          </div>
        </template>

        <template v-else>
          <div class="col-12 col-md-auto">
            <q-chip square dense color="primary" text-color="white">{{ t("tools.wiring.modeChip") }}</q-chip>
          </div>
        </template>

        <q-space />

        <div class="col-12 col-md-auto">
          <q-chip square dense color="grey-8" text-color="white" icon="usb">{{ t("tools.wiring.connectedInterface") }}</q-chip>
        </div>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted, watch } from 'vue'
import { useI18n } from "vue-i18n"
import { useRoute, useRouter } from "vue-router"
import { useDeviceStore } from '@/store/device'

const TOOL_TABS = ["wiring", "serial", "can"] as const

function normalizeToolTab(value: unknown) {
  return typeof value === "string" && TOOL_TABS.includes(value as (typeof TOOL_TABS)[number])
    ? value
    : "can"
}

const route = useRoute()
const router = useRouter()
const tab = ref(normalizeToolTab(route.query.tab))
const { t, tm } = useI18n()
const tabOptions = computed(() => [
  { label: t("tools.tabs.wiring"), value: "wiring", icon: "cable" },
  { label: t("tools.tabs.serial"), value: "serial", icon: "usb" },
  { label: t("tools.tabs.can"), value: "can", icon: "directions_car" }
])
const deviceStore = useDeviceStore()
const isSerialConnected = computed(() => deviceStore.connectionStatus === 'CONNECTED')
const serialSummary = computed(() => {
  const portLabel = deviceStore.port || t("tools.serial.noPort")
  return `${portLabel} @ ${deviceStore.baudRate}bps`
})

// --- CAN Logic ---
const isCanCapturing = ref(true)
const msgRate = ref(4250)
const busLoad = ref(66)
const canFilters = ref({ idRange: '', bitmask: '' })

interface CanMessage {
  id: number
  timestamp: string
  msgId: string
  dlc: number
  data: string
}

const canMessages = ref<CanMessage[]>([])
let canIdCounter = 0

const canColumns = computed(() => [
  { name: 'timestamp', label: t("tools.can.columns.timestamp"), field: 'timestamp', align: 'left', sortable: true },
  { name: 'msgId', label: t("tools.can.columns.messageId"), field: 'msgId', align: 'left', sortable: true },
  { name: 'dlc', label: 'DLC', field: 'dlc', align: 'left', sortable: true },
  { name: 'data', label: t("tools.can.columns.data"), field: 'data', align: 'left' }
])

const generateCanMockData = () => {
  if (!isCanCapturing.value || tab.value !== 'can') return
  const now = new Date()
  const timestamp = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}.${now.getMilliseconds().toString().padStart(3, '0')}`
  const ids = ['0x00F3', '0x01A4', '0x80F3', '0x60F3', '0x90F3']
  const msgId = ids[Math.floor(Math.random() * ids.length)]
  const dlc = Math.random() > 0.5 ? 8 : 4
  const data = dlc === 8 ? 'FF 0A 1B C4 55 EE 01 99' : '12 34 56 78'
  canMessages.value.unshift({ id: canIdCounter++, timestamp, msgId, dlc, data })
  if (canMessages.value.length > 100) canMessages.value.pop()
}

const toggleCanCapture = () => { isCanCapturing.value = !isCanCapturing.value }
const stopCanCapture = () => { isCanCapturing.value = false }
const clearCanMessages = () => { canMessages.value = [] }

// --- Serial Logic ---
const serialLogs = ref<{time: string, direction: string, content: string}[]>([])
const serialInput = ref('')

const clearSerialLog = () => { serialLogs.value = [] }

const sendSerialData = () => {
  if (!isSerialConnected.value || !serialInput.value) return
  serialLogs.value.push({
    time: new Date().toLocaleTimeString(),
    direction: 'TX',
    content: serialInput.value
  })
  serialInput.value = ''
  
  // Simulate echo/response
  setTimeout(() => {
    serialLogs.value.push({
      time: new Date().toLocaleTimeString(),
      direction: 'RX',
      content: t("tools.serial.ack")
    })
  }, 500)
}

const generateSerialMockData = () => {
  if (!isSerialConnected.value || tab.value !== 'serial' || Math.random() > 0.1) return
  const mockMsgs = [
    'Sensor data: temp=25.4C, hum=45%',
    'Heartbeat: OK',
    'System status: IDLE',
    'Warning: Voltage low (3.2V)'
  ]
  serialLogs.value.push({
    time: new Date().toLocaleTimeString(),
    direction: 'RX',
    content: mockMsgs[Math.floor(Math.random() * mockMsgs.length)]
  })
  if (serialLogs.value.length > 50) serialLogs.value.shift()
}

// --- Wiring Guides ---
const canGuideSteps = computed(() => tm("tools.wiring.guides.can.steps"))
const uartGuideSteps = computed(() => tm("tools.wiring.guides.uart.steps"))
const rs485GuideSteps = computed(() => tm("tools.wiring.guides.rs485.steps"))
const wiringGuides = computed(() => [
  {
    title: t("tools.wiring.guides.can.title"),
    image: 'https://picsum.photos/seed/canbus/400/225',
    description: t("tools.wiring.guides.can.description"),
    steps: canGuideSteps.value
  },
  {
    title: t("tools.wiring.guides.uart.title"),
    image: 'https://picsum.photos/seed/uart/400/225',
    description: t("tools.wiring.guides.uart.description"),
    steps: uartGuideSteps.value
  },
  {
    title: t("tools.wiring.guides.rs485.title"),
    image: 'https://picsum.photos/seed/rs485/400/225',
    description: t("tools.wiring.guides.rs485.description"),
    steps: rs485GuideSteps.value
  }
])

// --- Lifecycle ---
let canInterval: any = null
let serialInterval: any = null

watch(
  () => route.query.tab,
  (value) => {
    const normalizedTab = normalizeToolTab(value)
    if (tab.value !== normalizedTab) {
      tab.value = normalizedTab
    }
  }
)

watch(
  tab,
  (value) => {
    if (route.query.tab === value) {
      return
    }

    router.replace({
      path: "/tools",
      query: {
        ...route.query,
        tab: value,
      },
    })
  },
  { immediate: true }
)

onMounted(() => {
  canInterval = setInterval(generateCanMockData, 100)
  serialInterval = setInterval(generateSerialMockData, 1000)
})

onUnmounted(() => {
  if (canInterval) clearInterval(canInterval)
  if (serialInterval) clearInterval(serialInterval)
})
</script>

<style scoped lang="scss">
.tools-page {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  gap: var(--dt-space-3);
  padding: var(--dt-space-3);
}

.tools-tabs {
  display: flex;
  flex-direction: column;
  flex: 0 0 auto;
  background: transparent;
  border: 0;
  box-shadow: none;
}

.tools-tabs__topbar {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 10px 12px;
}

.tools-page :deep(.q-tab-panels) {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.tools-page :deep(.q-panel) {
  height: 100%;
  min-height: 0;
}

.tools-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  overflow: hidden;
}

.tools-surface,
.tools-guide-card,
.tools-status-card {
  border-radius: var(--dt-radius-panel);
}

.tools-surface {
  container-type: inline-size;
}

.tools-guide-card {
  min-height: 100%;
}

.tools-toolbar {
  padding: 10px 12px;
}

.tools-toolbar__row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  min-width: 0;
}

.tools-toolbar__filters {
  display: grid;
  grid-template-columns: minmax(220px, 320px) minmax(180px, 220px);
  gap: 12px;
  flex: 1 1 auto;
  min-width: 0;
}

.tools-toolbar__filter {
  min-width: 0;
}

.tools-filter-field {
  display: grid;
  grid-template-columns: 62px minmax(0, 1fr);
  align-items: center;
  gap: 10px;
  min-width: 0;
}

.tools-filter-field__label {
  color: var(--dt-text-muted);
  font-size: 12px;
  font-weight: 700;
  line-height: 1;
  white-space: nowrap;
  text-align: right;
}

.tools-filter-field :deep(.q-field) {
  min-width: 0;
}

.tools-filter-field__input :deep(.q-field--dense .q-field__control),
.tools-filter-field__input :deep(.q-field--dense .q-field__native),
.tools-filter-field__input :deep(.q-field--dense .q-field__marginal) {
  min-height: 36px;
}

.tools-filter-field__input :deep(.q-field--outlined .q-field__control) {
  height: 36px;
}

.tools-filter-field__input :deep(.q-field--dense .q-field__control-container) {
  padding-top: 0;
}

.tools-filter-field__input :deep(.q-field--dense .q-field__native),
.tools-filter-field__input :deep(.q-field--dense .q-field__input) {
  padding-top: 0;
  padding-bottom: 0;
}

.tools-toolbar__actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  flex-wrap: nowrap;
  gap: 10px;
  flex: 0 0 auto;
  white-space: nowrap;
}

.tools-action-btn {
  height: 36px;
  min-height: 36px;
}

.tools-toolbar__actions :deep(.q-btn) {
  height: 36px;
  min-height: 36px;
  padding: 0 12px;
  font-size: 12px;
  font-weight: 700;
}

.tools-action-btn :deep(.q-btn__content) {
  min-height: 36px;
  padding: 0;
  font-size: 12px;
  font-weight: 700;
}

@container (max-width: 1320px) {
  .tools-toolbar {
    padding: 8px 10px;
  }

  .tools-toolbar__row {
    flex-direction: column;
    align-items: stretch;
    gap: 12px;
  }

  .tools-toolbar__filters {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  .tools-toolbar__filter {
    min-width: 0;
  }

  .tools-filter-field {
    grid-template-columns: 56px minmax(0, 1fr);
    gap: 8px;
  }

  .tools-filter-field__label {
    text-align: right;
  }

  .tools-toolbar__actions {
    justify-content: flex-end;
    flex-wrap: wrap;
    gap: 8px;
  }

  .tools-toolbar__actions :deep(.q-btn) {
    height: 36px;
    min-height: 36px;
    padding: 0 12px;
    font-size: 12px;
  }
}

@container (max-width: 980px) {
  .tools-toolbar {
    padding: 10px;
  }

  .tools-toolbar__row {
    gap: 10px;
  }

  .tools-toolbar__filters {
    grid-template-columns: 1fr;
  }

  .tools-toolbar__actions {
    justify-content: flex-start;
  }

  .tools-filter-field {
    grid-template-columns: 1fr;
    gap: 6px;
  }

  .tools-filter-field__label {
    line-height: 1.2;
    text-align: left;
  }
}

@media (max-width: 920px) {
  .tools-page {
    padding: var(--dt-space-3);
  }

  .tools-toolbar__filters {
    grid-template-columns: 1fr;
  }

  .tools-toolbar__actions {
    justify-content: flex-start;
  }
}
</style>
