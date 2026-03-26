<template>
  <ToolShell title="CAN 模式命令预置" description="通过适配器串口下发与 CAN 升级相关的常用控制命令。">
    <template #main>
      <div class="preset-list">
        <button v-for="item in presets" :key="item.label" class="preset-item" @click="applyPreset(item)">
          <strong>{{ item.label }}</strong>
          <span>{{ item.command }}</span>
          <small>{{ item.desc }}</small>
        </button>
      </div>
      <div class="result-card" v-if="lastResult">
        <h4>最近结果</h4>
        <div>命令字：{{ lastResult.command }}</div>
        <div>请求帧：<code>{{ lastResult.requestHex }}</code></div>
        <div>响应帧：<code>{{ lastResult.responseHex }}</code></div>
      </div>
    </template>
    <template #side>
      <p class="side-text">这些预置命令主要用于调试 CAN 相关流程，本质仍是通过 PC 与适配器之间的 UART 链路发送 `55` 协议。</p>
      <el-alert type="info" :closable="false" title="使用前先在左侧连接串口适配器" />
    </template>
  </ToolShell>
</template>

<script setup>
import { ElMessage } from "element-plus"
import { parseHexInput, sendRawCommand } from "@/api/unimaster"
import ToolShell from "./ToolShell.vue"

const presets = [
  { label: "读取接入状态", command: "20", payload: "", desc: "检测仪表是否已接入" },
  { label: "切换透传界面", command: "AF", payload: "00", desc: "进入实时烧录透传界面" },
  { label: "切回主界面", command: "AF", payload: "01", desc: "返回主界面" },
  { label: "获取版本信息", command: "A0", payload: "", desc: "读取 UniMaster APP/UI 版本" },
]

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
    ElMessage.error(String(error))
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
  border: 1px solid #e0e7f1;
  border-radius: var(--dt-radius-subtle);
  background: linear-gradient(180deg, #ffffff, #f8fbff);
  padding: 16px;
  text-align: left;
  cursor: pointer;
  transition: transform 0.18s ease, border-color 0.18s ease, box-shadow 0.18s ease;
}

.preset-item:hover {
  transform: translateY(-1px);
  border-color: #bfd5f6;
  box-shadow: 0 10px 22px rgba(22, 119, 255, 0.08);
}

.preset-item strong,
.preset-item span,
.preset-item small {
  display: block;
}

.preset-item span {
  margin: 8px 0 6px;
  color: var(--el-color-primary);
  font-weight: 700;
}

.preset-item small {
  color: #72809a;
}

.result-card {
  margin-top: 18px;
  border-radius: var(--dt-radius-subtle);
  background: #f6f9fd;
  border: 1px solid #dde7f3;
  padding: 16px;
}

.side-text {
  margin-top: 0;
  margin-bottom: 16px;
  color: #72809a;
  line-height: 1.8;
}

code {
  white-space: pre-wrap;
}
</style>
