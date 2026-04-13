<template>
  <ToolShell :title="t('tools.canConsole.title')" :description="t('tools.canConsole.description')">
    <template #main>
      <div class="preset-list">
        <button v-for="item in presets" :key="item.label" class="preset-item" type="button" @click="applyPreset(item)">
          <strong>{{ item.label }}</strong>
          <span>{{ item.command }}</span>
          <small>{{ item.desc }}</small>
        </button>
      </div>

      <div v-if="lastResult" class="result-card panel">
        <h4>{{ t("tools.can.recentResult") }}</h4>
        <div>{{ t("tools.can.command", { value: lastResult.command }) }}</div>
        <div>{{ t("tools.can.request") }}<code>{{ lastResult.requestHex }}</code></div>
        <div>{{ t("tools.can.response") }}<code>{{ lastResult.responseHex }}</code></div>
      </div>
    </template>

    <template #side>
      <p class="side-text">{{ t("tools.can.presetHint") }}</p>
      <q-banner rounded class="info-banner">
        {{ t("tools.can.adapterBanner") }}
      </q-banner>
    </template>
  </ToolShell>
</template>

<script setup>
import { computed } from "vue"
import { useI18n } from "vue-i18n"
import { notifyError } from "@/services/ui"
import { parseHexInput, sendRawCommand } from "@/api/unimaster"
import ToolShell from "./ToolShell.vue"

const { t } = useI18n()
const presets = computed(() => [
  { label: t("tools.can.presets.access.label"), command: "20", payload: "", desc: t("tools.can.presets.access.desc") },
  { label: t("tools.can.presets.passthrough.label"), command: "AF", payload: "00", desc: t("tools.can.presets.passthrough.desc") },
  { label: t("tools.can.presets.main.label"), command: "AF", payload: "01", desc: t("tools.can.presets.main.desc") },
  { label: t("tools.can.presets.version.label"), command: "A0", payload: "", desc: t("tools.can.presets.version.desc") },
])

const lastResult = ref(null)

async function applyPreset(item) {
  try {
    const exchange = await sendRawCommand({
      command: Number.parseInt(item.command, 16),
      payload: parseHexInput(item.payload),
      timeoutMs: 1500,
    })
    lastResult.value = exchange
  } catch (error) {
    notifyError(error)
  }
}
</script>

<style scoped lang="scss">
.preset-list {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 12px;
}

.preset-item {
  border: 1px solid var(--dt-border);
  border-radius: var(--dt-radius-subtle);
  background: linear-gradient(180deg, var(--dt-bg-panel), var(--dt-bg-panel-soft));
  padding: 16px;
  text-align: left;
  color: var(--dt-text-primary);
  cursor: pointer;
  transition: transform 0.18s ease, border-color 0.18s ease, box-shadow 0.18s ease;
}

.preset-item:hover {
  transform: translateY(-1px);
  border-color: color-mix(in srgb, var(--dt-accent) 42%, var(--dt-border));
  box-shadow: 0 10px 22px rgba(22, 119, 255, 0.08);
}

.preset-item strong,
.preset-item span,
.preset-item small {
  display: block;
}

.preset-item span {
  margin: 8px 0 6px;
  color: var(--dt-accent);
  font-weight: 700;
}

.preset-item small {
  color: var(--dt-text-secondary);
}

.result-card {
  margin-top: 18px;
  padding: 16px;
}

.side-text {
  margin-top: 0;
  margin-bottom: 16px;
  color: var(--dt-text-secondary);
  line-height: 1.8;
}

.info-banner {
  background: color-mix(in srgb, var(--dt-info) 10%, var(--dt-bg-panel));
  border: 1px solid color-mix(in srgb, var(--dt-info) 20%, var(--dt-border));
}

code {
  white-space: pre-wrap;
}

@media (max-width: 960px) {
  .preset-list {
    grid-template-columns: 1fr;
  }
}
</style>
