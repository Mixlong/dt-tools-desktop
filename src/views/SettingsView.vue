<template>
  <div class="settings-page">
    <div class="settings-shell">
      <aside class="settings-aside">
        <q-card flat bordered class="settings-summary">
          <q-card-section class="settings-summary__head">
            <span class="settings-eyebrow">SYSTEM PREFERENCES</span>
            <h1>{{ t("settings.title") }}</h1>
            <p>{{ t("settings.description") }}</p>
          </q-card-section>

          <q-separator />

          <q-card-section class="settings-summary__body">
            <div class="settings-summary__item">
              <div class="settings-summary__icon settings-summary__icon--path">
                <q-icon name="folder_open" size="18px" />
              </div>
              <div class="settings-summary__copy">
                <span>{{ t("settings.logDirectory.title") }}</span>
                <strong>{{ form.logDirectory ? "已配置" : "未配置" }}</strong>
              </div>
            </div>

            <div class="settings-summary__item">
              <div class="settings-summary__icon settings-summary__icon--startup">
                <q-icon name="power_settings_new" size="18px" />
              </div>
              <div class="settings-summary__copy">
                <span>{{ t("settings.autostart.title") }}</span>
                <strong>{{ form.autostart ? "已开启" : "已关闭" }}</strong>
              </div>
            </div>

            <div class="settings-summary__item">
              <div class="settings-summary__icon settings-summary__icon--update">
                <q-icon name="system_update_alt" size="18px" />
              </div>
              <div class="settings-summary__copy">
                <span>{{ t("settings.updates.title") }}</span>
                <strong>{{ form.checkUpdatesOnLaunch ? "已开启" : "已关闭" }}</strong>
              </div>
            </div>
          </q-card-section>
        </q-card>
      </aside>

      <section class="settings-main">
        <q-card flat bordered class="settings-panel">
          <q-card-section class="settings-panel__head">
            <div>
              <span class="settings-panel__eyebrow">Storage</span>
              <h2>{{ t("settings.logDirectory.title") }}</h2>
              <p>{{ t("settings.logDirectory.description") }}</p>
            </div>
          </q-card-section>

          <q-separator />

          <q-card-section class="settings-panel__body">
            <div class="settings-field">
              <label>{{ t("settings.logDirectory.title") }}</label>
              <q-input v-model="form.logDirectory" outlined dense :bg-color="undefined" @update:model-value="saveAllPreferences">
                <template #prepend>
                  <q-icon name="folder_open" />
                </template>
                <template #append>
                  <q-btn
                    flat
                    dense
                    no-caps
                    color="primary"
                    :label="t('settings.logDirectory.select')"
                    :disable="!isTauriDesktop"
                    @click="selectLogDirectory"
                  />
                </template>
              </q-input>
            </div>
          </q-card-section>
        </q-card>

        <q-card flat bordered class="settings-panel">
          <q-card-section class="settings-row">
            <div class="settings-row__copy">
              <span class="settings-panel__eyebrow">Boot</span>
              <h3>{{ t("settings.autostart.title") }}</h3>
              <p>{{ t("settings.autostart.description") }}</p>
            </div>
            <q-toggle
              v-model="form.autostart"
              checked-icon="done"
              unchecked-icon="close"
              color="primary"
              :disable="loading.autostart"
              @update:model-value="handleAutostartChange"
            />
          </q-card-section>
        </q-card>

        <q-card flat bordered class="settings-panel">
          <q-card-section class="settings-row">
            <div class="settings-row__copy">
              <span class="settings-panel__eyebrow">Updates</span>
              <h3>{{ t("settings.updates.title") }}</h3>
              <p>{{ t("settings.updates.description") }}</p>
            </div>
            <q-toggle
              v-model="form.checkUpdatesOnLaunch"
              checked-icon="done"
              unchecked-icon="close"
              color="primary"
              @update:model-value="saveAllPreferences"
            />
          </q-card-section>
        </q-card>

        <UniMasterAboutSection />
      </section>
    </div>
  </div>
</template>

<script setup>
import { documentDir, join } from "@tauri-apps/api/path"
import { disable, enable, isEnabled } from "@tauri-apps/plugin-autostart"
import { open } from "@tauri-apps/plugin-dialog"
import { useI18n } from "vue-i18n"
import { notifyError, notifyInfo, notifySuccess } from "@/services/ui"
import { DEFAULT_PREFERENCES, loadPreferences, savePreferences } from "@/utils/preferences"
import UniMasterAboutSection from "@/views/UniMasterAboutView.vue"

const { t } = useI18n()

const form = reactive({
  ...DEFAULT_PREFERENCES,
  ...loadPreferences(),
})

const loading = reactive({
  autostart: false,
})

const isTauriDesktop = typeof window !== "undefined" && Boolean(window.__TAURI_INTERNALS__)

onMounted(async () => {
  saveAllPreferences()

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
    notifyInfo(t("settings.logDirectory.desktopOnly"))
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
    notifySuccess(t("settings.logDirectory.updated"))
  } catch (error) {
    notifyError(error)
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
    notifySuccess(value ? t("settings.autostart.enabled") : t("settings.autostart.disabled"))
  } catch (error) {
    form.autostart = !value
    notifyError(error)
  } finally {
    loading.autostart = false
  }
}
</script>

<style scoped lang="scss">
.settings-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  min-height: 0;
  padding: 12px;
  background: transparent;
  box-sizing: border-box;
}

.settings-shell {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-columns: 320px minmax(0, 1fr);
  gap: 18px;
}

.settings-aside,
.settings-main {
  min-height: 0;
}

.settings-aside {
  display: flex;
}

.settings-main {
  display: flex;
  flex-direction: column;
  gap: 14px;
  overflow-y: auto;
  padding-right: 4px;
}

.settings-main::-webkit-scrollbar {
  width: 6px;
}

.settings-main::-webkit-scrollbar-thumb {
  background: var(--dt-border-strong);
  border-radius: 3px;
}

.settings-eyebrow,
.settings-panel__eyebrow {
  display: inline-block;
  margin-bottom: 10px;
  color: var(--dt-text-muted);
  font-size: 11px;
  font-weight: 800;
  letter-spacing: 0.18em;
  text-transform: uppercase;
}

.settings-summary,
.settings-panel {
  background: var(--dt-gloss-surface);
  border: 1px solid var(--dt-border);
  box-shadow: var(--dt-shadow-float);
}

.settings-summary {
  position: sticky;
  top: 0;
  flex: 1 1 auto;
}

.settings-summary__head {
  padding: 22px 22px 20px;
}

.settings-summary__head h1 {
  margin: 0 0 10px;
  font-size: 28px;
  font-weight: 800;
  line-height: 1.15;
  color: var(--dt-text-primary);
}

.settings-summary__head p,
.settings-panel__head p,
.settings-row__copy p {
  margin: 0;
  color: var(--dt-text-secondary);
  font-size: 14px;
  line-height: 1.6;
}

.settings-summary__body {
  display: flex;
  flex-direction: column;
  gap: 14px;
  padding: 18px 22px 22px;
}

.settings-summary__item {
  display: flex;
  align-items: center;
  gap: 12px;
}

.settings-summary__icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 38px;
  height: 38px;
  border-radius: 12px;
}

.settings-summary__icon--path {
  color: var(--dt-brand-primary);
  background: var(--dt-brand-primary-soft);
}

.settings-summary__icon--startup {
  color: var(--dt-success);
  background: var(--dt-status-success-soft);
}

.settings-summary__icon--update {
  color: var(--dt-warning);
  background: var(--dt-status-warning-soft);
}

.settings-summary__copy span {
  display: block;
  color: var(--dt-text-muted);
  font-size: 12px;
  font-weight: 700;
}

.settings-summary__copy strong {
  color: var(--dt-text-primary);
  font-size: 16px;
  font-weight: 800;
}

.settings-panel__head {
  padding: 18px 20px 16px;
}

.settings-panel__head h2,
.settings-row__copy h3 {
  margin: 0 0 6px;
  font-size: 20px;
  font-weight: 800;
  line-height: 1.2;
  color: var(--dt-text-primary);
}

.settings-panel__body {
  padding: 18px 20px 20px;
}

.settings-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.settings-field label {
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 700;
}

.settings-row {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 18px;
  padding: 18px 20px;
}

.settings-row__copy {
  flex: 1;
}

.settings-panel :deep(.q-toggle) {
  flex: 0 0 auto;
}

.settings-panel :deep(.q-separator) {
  background: var(--dt-border);
}

.settings-field :deep(.q-field--outlined .q-field__control) {
  background: var(--dt-bg-input);
  border-radius: var(--dt-radius-field);
  color: var(--dt-text-primary);
}

.settings-field :deep(.q-field--outlined .q-field__control:before) {
  border-color: var(--dt-border-strong);
}

.settings-field :deep(.q-field--outlined .q-field__control:hover:before),
.settings-field :deep(.q-field--outlined.q-field--focused .q-field__control:before),
.settings-field :deep(.q-field--outlined.q-field--highlighted .q-field__control:before) {
  border-color: var(--dt-brand-primary);
}

.settings-field :deep(.q-field__native),
.settings-field :deep(.q-field__input),
.settings-field :deep(.q-field__prefix),
.settings-field :deep(.q-field__suffix),
.settings-field :deep(.q-field__prepend),
.settings-field :deep(.q-field__append) {
  color: var(--dt-text-primary);
}

.settings-field :deep(.q-field__label),
.settings-field :deep(.q-field__marginal),
.settings-field :deep(.q-placeholder) {
  color: var(--dt-text-secondary);
}

.settings-panel :deep(.q-toggle__inner) {
  color: var(--dt-text-muted);
}

.settings-panel :deep(.q-toggle__track) {
  opacity: 1;
  background: var(--dt-brand-secondary-soft);
}

.settings-panel :deep(.q-toggle__thumb) {
  color: var(--dt-bg-panel-strong);
}

.settings-panel :deep(.q-toggle__inner--truthy) {
  color: var(--dt-brand-primary);
}

.settings-panel :deep(.q-toggle__inner--truthy .q-toggle__track) {
  background: var(--dt-brand-primary-soft);
}

@media (max-width: 960px) {
  .settings-page {
    padding: 12px;
  }

  .settings-shell {
    grid-template-columns: 1fr;
  }

  .settings-summary {
    position: static;
  }

  .settings-panel__head,
  .settings-panel__body,
  .settings-row {
    padding: 16px;
  }

  .settings-row {
    flex-direction: column;
    align-items: stretch;
  }
}
</style>
