<template>
  <ToolShell title="串口协议控制台" description="直接向 UniMaster 适配器发送 55 协议命令，查看请求帧和应答帧。">
    <template #main>
      <div class="log-panel">
        <div v-for="item in logs" :key="item.id" class="log-block">
          <div class="log-head">
            <span>{{ item.time }}</span>
            <strong>0x{{ item.command.toString(16).padStart(2, "0").toUpperCase() }}</strong>
          </div>
          <div>TX: <code>{{ item.requestHex }}</code></div>
          <div>RX: <code>{{ item.responseHex }}</code></div>
        </div>
        <div v-if="logs.length === 0" class="log-empty">
          <strong>等待命令执行</strong>
          <p>从右侧输入命令字和 HEX 负载后，发送结果会按时间倒序显示在这里。</p>
        </div>
      </div>
    </template>
    <template #side>
      <p class="side-copy">适合临时调试版本读取、接入状态和透传切换命令。保持输入区简洁，重点放在收发帧结果。</p>
      <el-form label-position="top" class="command-form">
        <el-form-item label="命令字">
          <el-input v-model="commandText" placeholder="例如 A0 / B1 / A7" />
        </el-form-item>
        <el-form-item label="负载 HEX">
          <el-input v-model="payload" type="textarea" :rows="6" placeholder="例如 04 或 00 00 00 01" />
        </el-form-item>
      </el-form>
      <el-button class="send-button" type="primary" :loading="loading" @click="send">发送命令</el-button>
    </template>
  </ToolShell>
</template>

<script setup>
import { ElMessage } from "element-plus"
import { parseHexInput, sendRawCommand } from "@/api/unimaster"
import ToolShell from "./ToolShell.vue"

const commandText = ref("A0")
const payload = ref("")
const loading = ref(false)
const logs = ref([])

async function send() {
  loading.value = true
  try {
    const command = Number.parseInt(commandText.value.replace(/^0x/i, ""), 16)
    if (Number.isNaN(command)) {
      throw new Error("命令字格式错误")
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
    ElMessage.error(String(error))
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
}

.log-block {
  padding: 12px 14px;
  border: 1px solid rgba(135, 160, 204, 0.2);
  border-radius: var(--dt-radius-field);
  background: rgba(255, 255, 255, 0.02);
}

.log-head {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
  color: #8ea7cf;
}

.log-empty {
  display: grid;
  place-items: center;
  min-height: 320px;
  text-align: center;
  color: #afc4e8;
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
  margin-bottom: 16px;
}

.send-button {
  width: 100%;
}

code {
  white-space: pre-wrap;
}
</style>
