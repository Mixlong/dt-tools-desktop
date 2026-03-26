<template>
  <div class="shell">
    <aside class="sidebar">
      <div class="sidebar-main">
        <el-scrollbar class="sidebar-scroll">
          <div class="sidebar-scroll__content">
            <div class="sidebar-brand">
              <img :src="logoImage" alt="DT-Tools logo" class="sidebar-brand__logo" />
            </div>

            <nav class="nav-list">
              <section v-for="section in navSections" :key="section.label" class="nav-section">
                <router-link
                  v-if="section.to"
                  :to="section.to"
                  :class="['nav-section__title', 'nav-section__title--link', { 'nav-section__title--active': route.path === section.to }]"
                >
                  <el-icon><component :is="section.icon" /></el-icon>
                  <span>{{ section.label }}</span>
                </router-link>

                <div v-else :class="['nav-section__title', { 'nav-section__title--active': isSectionActive(section) }]">
                  <el-icon><component :is="section.icon" /></el-icon>
                  <span>{{ section.label }}</span>
                </div>

                <div v-if="section.items?.length" class="nav-section__items">
                  <router-link v-for="item in section.items" :key="item.to" :to="item.to" class="nav-subitem">
                    <span class="nav-subitem__dot" />
                    <span>{{ item.label }}</span>
                  </router-link>
                </div>
              </section>
            </nav>
          </div>
        </el-scrollbar>
      </div>

      <div class="sidebar-bottom">
        <section class="device-dock">
          <div class="device-card" @click="drawer = true">
            <div class="device-card__top">
              <div class="device-icon"><el-icon><Lightning /></el-icon></div>
              <div class="device-meta">
                <strong>{{ deviceStore.currentModelLabel }}</strong>
                <span>{{ deviceStore.onlineStatus }}</span>
              </div>
            </div>

            <div class="device-card__status-row">
              <span :class="['device-status', { 'device-status--online': deviceStore.connectionStatus === 'CONNECTED' }]">
                {{ deviceStore.connectionStatus === "CONNECTED" ? "已连接" : "未连接" }}
              </span>
              <span class="device-card__endpoint">{{ deviceEndpoint }}</span>
            </div>

            <div class="device-card__bottom" @click.stop>
              <el-select v-model="deviceStore.currentModel" class="device-card__select">
                <el-option
                  v-for="item in deviceStore.models"
                  :key="item.value"
                  :label="item.label"
                  :value="item.value"
                />
              </el-select>
            </div>
          </div>
        </section>
      </div>
    </aside>

    <main class="main">
      <section :class="['content', { 'content--home': route.path === '/home' }]">
        <router-view />
      </section>
    </main>

    <el-drawer v-model="drawer" title="适配器连接" size="420px">
        <div class="drawer-form">
          <el-form label-position="top">
          <el-form-item label="通信模式">
            <el-segmented v-model="deviceStore.transport" :options="transportOptions" />
          </el-form-item>
          <el-form-item label="端口">
            <el-select v-model="deviceStore.port" filterable>
              <el-option v-for="item in deviceStore.ports" :key="item.value" :label="item.label" :value="item.value" />
            </el-select>
          </el-form-item>
          <el-form-item label="适配器串口波特率">
            <el-select v-model="deviceStore.baudRate">
              <el-option v-for="item in baudRates" :key="item" :label="item" :value="item" />
            </el-select>
            <div class="drawer-help">这是电脑连接 UniMaster 适配器的串口速率，通常使用 115200。</div>
          </el-form-item>
          <el-form-item v-if="deviceStore.transport === 'can'" label="CAN 波特率">
            <el-select v-model="deviceStore.canBitrate">
              <el-option v-for="item in canRates" :key="item" :label="`${item} kbps`" :value="item" />
            </el-select>
          </el-form-item>
          <el-form-item v-if="deviceStore.transport === 'can'" label="帧类型">
            <el-segmented v-model="deviceStore.frameType" :options="frameTypeOptions" />
          </el-form-item>
        </el-form>
        <div class="drawer-actions">
          <el-button @click="refreshPorts">刷新端口</el-button>
          <el-button type="primary" :loading="connecting" @click="toggleConnection">
          {{ deviceStore.connectionStatus === "CONNECTED" ? "断开连接" : "连接设备" }}
          </el-button>
        </div>
      </div>
    </el-drawer>
  </div>
</template>

<script setup>
import { ElMessage } from "element-plus"
import { useRoute } from "vue-router"
import { useDeviceStore } from "@/store/device"
import logoImage from "@/assect/images/logo.png"

const route = useRoute()
const deviceStore = useDeviceStore()
const drawer = ref(false)
const connecting = ref(false)

const navSections = [
  {
    label: "首页",
    icon: "House",
    to: "/home",
  },
  {
    label: "工具",
    icon: "Tools",
    items: [
      { to: "/tools/wiring", label: "接线指引" },
      { to: "/tools/serial", label: "串口抓包" },
      { to: "/tools/can", label: "CAN 数据抓包" },
    ],
  },
  {
    label: "软件",
    icon: "Upload",
    items: [{ to: "/software", label: "固件升级" }],
  },
  {
    label: "设置",
    icon: "Setting",
    items: [
      { to: "/config", label: "仪表参数" },
      { to: "/settings", label: "系统设置" },
    ],
  },
  {
    label: "支持",
    icon: "QuestionFilled",
    items: [{ to: "/support", label: "使用说明" }],
  },
]

const transportOptions = [
  { label: "UART", value: "uart" },
  { label: "CAN", value: "can" },
]
const frameTypeOptions = [
  { label: "标准帧", value: "standard" },
  { label: "扩展帧", value: "extended" },
]
const baudRates = [115200, 57600, 38400, 19200, 9600]
const canRates = [125, 250, 500, 1000]
const deviceEndpoint = computed(() => {
  if (deviceStore.connectionStatus === "CONNECTED" && deviceStore.port) {
    return deviceStore.port
  }
  return deviceStore.transport === "can" ? "CAN 模式" : "等待选择端口"
})

function isSectionActive(section) {
  return section.items.some((item) => item.to === route.path)
}

onMounted(async () => {
  try {
    await deviceStore.refreshPorts()
    await deviceStore.syncStatus()
  } catch (error) {
    ElMessage.error(String(error))
  }
})

async function refreshPorts() {
  try {
    await deviceStore.refreshPorts()
    ElMessage.success("端口列表已刷新")
  } catch (error) {
    ElMessage.error(String(error))
  }
}

async function toggleConnection() {
  connecting.value = true
  try {
    const status = await deviceStore.toggleConnection()
    ElMessage.success(status.connected ? "串口已连接" : "串口已断开")
  } catch (error) {
    ElMessage.error(String(error))
  } finally {
    connecting.value = false
  }
}
</script>

<style scoped lang="scss">
.shell {
  display: grid;
  grid-template-columns: 320px 1fr;
  width: 100%;
  height: 100vh;
  gap: 0;
  background: linear-gradient(180deg, #0c1119, #0f1722);
  overflow: hidden;
}

.sidebar {
  display: flex;
  flex-direction: column;
  background:
    linear-gradient(180deg, rgba(255, 255, 255, 0.94), rgba(246, 248, 251, 0.98)),
    linear-gradient(135deg, rgba(22, 119, 255, 0.05), rgba(22, 119, 255, 0));
  border-right: 1px solid #d9e1ec;
  padding: 24px 20px 20px;
  min-height: 0;
  overflow: hidden;
  overflow-x: hidden;
}

.sidebar-scroll {
  flex: 1;
  min-height: 0;
}

.sidebar-main {
  display: flex;
  flex: 1;
  min-height: 0;
  flex-direction: column;
}

.sidebar-scroll__content {
  min-height: 100%;
}

.sidebar-brand {
  display: flex;
  align-items: center;
  min-height: 54px;
  padding: 2px 2px 12px;
}

.sidebar-brand__logo {
  display: block;
  width: 132px;
  height: auto;
  object-fit: contain;
}

.device-card {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding: 16px 18px;
  border-radius: var(--dt-radius-panel);
  background: linear-gradient(180deg, #ffffff, #f8fbff);
  border: 1px solid #dbe4f0;
  box-shadow: var(--dt-shadow-panel);
  cursor: pointer;
}

.device-card__top {
  display: flex;
  gap: 14px;
  align-items: center;
}

.drawer-help {
  margin-top: 6px;
  color: var(--dt-text-muted);
  font-size: 12px;
  line-height: 1.5;
}

.device-card__bottom {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-top: 14px;
  border-top: 1px solid #e3eaf4;
}

.device-card__status-row {
  display: grid;
  grid-template-columns: auto minmax(0, 1fr);
  align-items: start;
  gap: 10px;
  margin-top: -2px;
}

.device-status {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-height: 24px;
  padding: 0 12px;
  border-radius: 999px;
  background: #eef2f7;
  color: #6b7a91;
  font-size: 12px;
  font-weight: 700;
  white-space: nowrap;
}

.device-status--online {
  background: rgba(24, 160, 88, 0.12);
  color: #15804a;
}

.device-card__endpoint {
  color: #6f7d93;
  font-size: 13px;
  line-height: 1.45;
  min-width: 0;
  overflow-wrap: anywhere;
  word-break: break-word;
  text-align: left;
}

.device-card__label {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.device-card__label strong {
  color: #20324f;
  font-size: 15px;
}

.device-card__label span {
  color: #748299;
  font-size: 13px;
  line-height: 1.5;
}

.device-card__select {
  width: 100%;
}

.device-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--dt-radius-subtle);
  display: grid;
  place-items: center;
  background: linear-gradient(135deg, #1677ff, #0f5cc8);
  color: white;
  font-size: 20px;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.2);
}

.device-meta {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.device-meta strong {
  color: #17305a;
  font-size: 16px;
  font-weight: 700;
  line-height: 1.25;
}

.device-meta span {
  color: #6f7d93;
  font-size: 13px;
}

.nav-list {
  display: flex;
  flex-direction: column;
  margin-top: 26px;
  gap: 20px;
  padding-bottom: 16px;
}

.nav-section {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.nav-section__title {
  display: flex;
  align-items: center;
  gap: 10px;
  color: #42536d;
  font-size: 16px;
  font-weight: 700;
  letter-spacing: 0;
}

.nav-section__title--link {
  text-decoration: none;
}

.nav-section__title--active {
  color: #1668dc;
}

.nav-section__items {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-left: 14px;
  padding-left: 16px;
  border-left: 1px solid #d8e2ee;
}

.nav-subitem {
  display: flex;
  gap: 12px;
  align-items: center;
  min-height: 40px;
  padding: 10px 12px;
  border-radius: var(--dt-radius-field);
  color: #495a74;
  text-decoration: none;
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 0;
  transition: background-color 0.2s ease, color 0.2s ease;
}

.nav-subitem__dot {
  width: 5px;
  height: 5px;
  border-radius: 999px;
  background: #9dafc8;
  flex: 0 0 auto;
}

.nav-subitem.router-link-active,
.nav-subitem.router-link-active {
  background: #e8f1ff;
  color: #1668dc;
}

.nav-subitem.router-link-active .nav-subitem__dot {
  background: #1668dc;
}

.sidebar-bottom {
  display: flex;
  flex-direction: column;
  gap: 0;
  flex: 0 0 auto;
  min-height: auto;
  padding-top: 16px;
  border-top: 1px solid #dde5ef;
  background:
    linear-gradient(180deg, rgba(246, 248, 251, 0), rgba(246, 248, 251, 0.9) 20%, rgba(246, 248, 251, 0.98));
}

.device-dock {
  display: block;
}

.main {
  display: flex;
  flex-direction: column;
  background:
    radial-gradient(circle at top right, rgba(22, 119, 255, 0.06), transparent 24%),
    linear-gradient(180deg, #eef3f8, #e9eef5);
  min-width: 0;
  min-height: 0;
  overflow: hidden;
}

.content {
  flex: 1;
  overflow: hidden;
  min-height: 0;
  padding: 20px 20px 20px;
}

.content--home {
  padding: 0;
}

.drawer-form {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.drawer-actions {
  display: flex;
  gap: 10px;
}

@media (max-width: 1080px) {
  .shell {
    grid-template-columns: 280px 1fr;
  }
}
</style>
