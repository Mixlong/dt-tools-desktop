<template>
  <div class="settings-page">
    <section class="page-intro">
      <div>
        <span class="section-eyebrow">System Preferences</span>
        <h2>系统设置</h2>
        <p>保持桌面工具的默认工作路径、启动行为和更新策略一致，避免现场环境因为配置分散而失控。</p>
      </div>
    </section>

    <div class="settings-grid">
      <section class="page-card settings-panel settings-panel--primary">
        <div class="panel-head">
          <div>
            <span class="section-eyebrow">Runtime</span>
            <h3>运行与更新</h3>
            <p>面向本机环境的基础设置，优先保证启动和升级流程稳定。</p>
          </div>
        </div>
        <div class="panel-body">
          <el-form label-position="top" class="settings-form">
            <div class="setting-block">
              <div class="setting-block__copy">
                <strong>默认日志目录</strong>
                <p>用于保存抓包结果、升级日志和错误追踪。</p>
              </div>
              <el-form-item label="日志输出路径">
                <div class="path-field">
                  <el-input v-model="form.logDirectory" @change="saveAllPreferences" />
                  <el-button :disabled="!isTauriDesktop" @click="selectLogDirectory">选择目录</el-button>
                </div>
              </el-form-item>
            </div>

            <div class="setting-block setting-block--compact">
              <div class="setting-block__copy">
                <strong>开机自启</strong>
                <p>适合固定工位环境，减少重复打开工具的操作。</p>
              </div>
              <el-switch v-model="form.autostart" :loading="loading.autostart" @change="handleAutostartChange" />
            </div>

            <div class="setting-block setting-block--compact">
              <div class="setting-block__copy">
                <strong>启动时检查更新</strong>
                <p>保持升级工具和协议逻辑处于最新发布版本。</p>
              </div>
              <el-switch v-model="form.checkUpdatesOnLaunch" @change="saveAllPreferences" />
            </div>
          </el-form>
        </div>
      </section>
    </div>
  </div>
</template>

<script setup>
import { ElMessage } from "element-plus"
import { documentDir, join } from "@tauri-apps/api/path"
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart"
import { open } from "@tauri-apps/plugin-dialog"
import { DEFAULT_PREFERENCES, loadPreferences, savePreferences } from "@/utils/preferences"

const form = reactive({
  ...DEFAULT_PREFERENCES,
  ...loadPreferences(),
})

const loading = reactive({
  autostart: false,
})

const isTauriDesktop = typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__)

onMounted(async () => {
  if (!form.logDirectory) {
    try {
      const docs = await documentDir()
      form.logDirectory = await join(docs, "DT-Tools", "logs")
      saveAllPreferences()
    } catch {
      form.logDirectory = "~/Documents/DT-Tools/logs"
      saveAllPreferences()
    }
  }

  if (!isTauriDesktop) return

  try {
    form.autostart = await isEnabled()
    saveAllPreferences()
  } catch (error) {
    console.warn("[settings] failed to read autostart state:", error)
  }
})

function saveAllPreferences() {
  savePreferences({ ...form })
}

async function selectLogDirectory() {
  if (!isTauriDesktop) {
    ElMessage.info("仅桌面应用支持系统目录选择器")
    return
  }

  try {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: form.logDirectory || undefined,
    })

    if (!selected || Array.isArray(selected)) return

    form.logDirectory = selected
    saveAllPreferences()
    ElMessage.success("日志目录已更新")
  } catch (error) {
    ElMessage.error(String(error))
  }
}

async function handleAutostartChange(value) {
  if (!isTauriDesktop) {
    saveAllPreferences()
    return
  }

  loading.autostart = true
  try {
    if (value) {
      await enable()
    } else {
      await disable()
    }
    saveAllPreferences()
    ElMessage.success(value ? "已开启开机自启" : "已关闭开机自启")
  } catch (error) {
    form.autostart = !value
    ElMessage.error(String(error))
  } finally {
    loading.autostart = false
  }
}
</script>

<style scoped lang="scss">
.settings-page {
  display: flex;
  flex-direction: column;
  gap: 12px;
  height: 100%;
  min-height: 0;
}

.settings-grid {
  display: grid;
  grid-template-columns: minmax(0, 1fr);
  flex: 1;
  min-height: 0;
}

.settings-panel {
  padding: 18px;
  display: flex;
  flex-direction: column;
  min-height: 0;
}

.settings-panel--primary {
  background: linear-gradient(180deg, #ffffff, #f8fbfe);
}

.settings-panel--dark {
  background: linear-gradient(180deg, #15233a, #1a2b45);
  border-color: rgba(109, 136, 184, 0.28);
  color: #e9f1ff;
}

.settings-panel--dark .section-eyebrow,
.settings-panel--dark p,
.settings-panel--dark h3 {
  color: inherit;
}

.panel-head h3,
.settings-panel h3 {
  margin: 0 0 8px;
}

.panel-head p {
  margin: 0;
  color: var(--dt-text-secondary);
}

.panel-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
  padding-right: 4px;
}

.settings-form {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-top: 14px;
}

.setting-block {
  padding: 14px;
  border: 1px solid #dfe7f2;
  border-radius: var(--dt-radius-subtle);
  background: #f7fafe;
}

.setting-block--compact {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 18px;
}

.setting-block__copy strong {
  display: block;
  margin-bottom: 6px;
  font-size: 14px;
}

.setting-block__copy p {
  margin: 0;
  color: var(--dt-text-secondary);
  font-size: 13px;
  line-height: 1.7;
}

.setting-block :deep(.el-form-item) {
  margin: 14px 0 0;
}

.setting-block :deep(.el-form-item__content) {
  width: 100%;
}

.path-field {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  gap: 10px;
  align-items: stretch;
  width: 100%;
}

.path-field :deep(.el-input) {
  width: 100%;
}

.path-field :deep(.el-input__wrapper) {
  width: 100%;
}

@media (max-width: 1080px) {
  .panel-body {
    overflow: visible;
    padding-right: 0;
  }

  .setting-block--compact {
    align-items: flex-start;
    flex-direction: column;
  }

  .path-field {
    grid-template-columns: 1fr;
  }
}
</style>
