<template>
  <ToolShell :title="t('tools.serialConsole.title')" :description="t('tools.serialConsole.description')">
    <template #main>
      <div class="log-panel dark-box">
        <div v-for="item in logs" :key="item.id" class="log-block">
          <div class="log-head">
            <span>{{ item.time }}</span>
            <strong>0x{{ item.command.toString(16).padStart(2, "0").toUpperCase() }}</strong>
          </div>
          <div>TX: <code>{{ item.requestHex }}</code></div>
          <div>RX: <code>{{ item.responseHex }}</code></div>
        </div>
        <div v-if="logs.length === 0" class="log-empty">
          <strong>{{ t("tools.serialConsole.waitingTitle") }}</strong>
          <p>{{ t("tools.serialConsole.waitingDescription") }}</p>
        </div>
      </div>
    </template>

    <template #side>
      <p class="side-copy">{{ t("tools.serialConsole.sideCopy") }}</p>
      <div class="command-form">
        <div class="command-field">
          <label>{{ t("tools.serialConsole.command") }}</label>
          <q-input v-model="commandText" outlined dense :placeholder="t('tools.serialConsole.commandPlaceholder')" />
        </div>
        <div class="command-field">
          <label>{{ t("tools.serialConsole.payload") }}</label>
          <q-input v-model="payload" outlined dense type="textarea" autogrow :placeholder="t('tools.serialConsole.payloadPlaceholder')" />
        </div>
      </div>
      <q-btn class="send-button" color="primary" unelevated :loading="loading" :label="t('tools.serialConsole.send')" @click="send" />
    </template>
  </ToolShell>
</template>

<script setup>
import { useI18n } from "vue-i18n"
import { notifyError } from "@/services/ui"
import { parseHexInput, sendRawCommand } from "@/api/unimaster"
import ToolShell from "./ToolShell.vue"

const { t } = useI18n()
const commandText = ref("A0")
const payload = ref("")
const loading = ref(false)
const logs = ref([])

async function send() {
  loading.value = true
  try {
    const command = Number.parseInt(commandText.value.replace(/^0x/i, ""), 16)
    if (Number.isNaN(command)) {
      throw new Error(t("tools.serialConsole.invalidCommand"))
    }

    const exchange = await sendRawCommand({
      command,
      payload: parseHexInput(payload.value),
      timeoutMs: 1500,
    })

    logs.value.unshift({
      ...exchange,
      id: Date.now(),
      time: new Date().toLocaleTimeString(),
    })
  } catch (error) {
    notifyError(error)
  } finally {
    loading.value = false
  }
}
</script>

<style scoped lang="scss">
.log-panel {
  display: flex;
  flex-direction: column;
  gap: 14px;
  min-height: 100%;
  padding: 16px;
}

.log-block {
  padding: 12px 14px;
  border: 1px solid var(--dt-border);
  border-radius: var(--dt-radius-field);
  background: #ffffff;
}

.log-head {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  color: var(--dt-text-secondary);
}

.log-empty {
  display: grid;
  place-items: center;
  min-height: 320px;
  text-align: center;
  color: var(--dt-text-secondary);
}

.log-empty strong {
  margin-bottom: 8px;
  font-size: 15px;
}

.log-empty p {
  max-width: 320px;
  margin: 0;
  line-height: 1.7;
}

.side-copy {
  margin: 0 0 16px;
  color: var(--dt-text-secondary);
  line-height: 1.8;
}

.command-form {
  display: flex;
  flex-direction: column;
  gap: 14px;
  margin-bottom: 16px;
}

.command-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.command-field label {
  color: var(--dt-text-secondary);
  font-size: 12px;
  font-weight: 800;
}

.send-button {
  width: 100%;
}

code {
  white-space: pre-wrap;
}
</style>
