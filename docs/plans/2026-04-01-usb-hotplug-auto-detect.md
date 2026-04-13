# USB 热插拔自动检测端口实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** USB 串口适配器插入后，自动检测新端口并选中，同时弹窗提示用户确认是否连接。

**Architecture:** Rust 后台线程每 1.5 秒轮询 `serialport::available_ports()`，对比前一次结果检测端口增减变化，通过 Tauri 的 `app.emit("ports-changed", payload)` 推送给前端。前端在 `device.js` store 中订阅该事件，自动更新端口列表并选中新端口，同时通过 Quasar Dialog 弹窗提示用户确认连接。

**Tech Stack:** Rust + Tauri v2 事件系统 (`app.emit`) + `@tauri-apps/api/event` (`listen`) + Vue 3 + Pinia + Quasar (`useQuasar` 的 `$q.dialog`)

---

## Task 1: Rust 后台线程 — 端口变化监听与事件推送

**Files:**
- Modify: `src-tauri/src/lib.rs` (在 `setup()` 闭包内添加后台线程)

### 前置知识

当前 `lib.rs` 第 237 行的 `.setup(|app| { ... })` 是启动后台任务的正确位置。Tauri v2 中用 `app.handle().clone()` 传递 AppHandle 给线程，用 `app_handle.emit("event-name", payload)` 向所有窗口推送事件。

### Step 1: 在 `lib.rs` 顶部补充需要的导入

当前第 1-12 行的导入中缺少线程相关 import。在文件顶部现有的 `use std::{...}` 块中加入 `thread` 和 `time::Duration`（注意：`Duration` 已有但在 `unimaster.rs` 里，`lib.rs` 里还没有）。

**编辑位置**：`src-tauri/src/lib.rs` 第 1 行

将：
```rust
use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
```

改为：
```rust
use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};
```

### Step 2: 在 `setup()` 闭包中启动端口监听后台线程

**编辑位置**：`src-tauri/src/lib.rs`，在 `.setup(|app| {` 闭包里，`app.manage(AppState::default());` 这一行（第 238 行）之后，托盘代码之前，插入以下代码：

```rust
            // USB 串口热插拔监听后台线程
            {
                let app_handle = app.handle().clone();
                thread::spawn(move || {
                    let mut last_ports: Vec<String> = serialport::available_ports()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|p| p.port_name)
                        .collect();

                    loop {
                        thread::sleep(Duration::from_millis(1500));

                        let current_ports: Vec<String> = serialport::available_ports()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|p| p.port_name)
                            .collect();

                        if current_ports != last_ports {
                            let added: Vec<String> = current_ports
                                .iter()
                                .filter(|p| !last_ports.contains(p))
                                .cloned()
                                .collect();

                            let removed: Vec<String> = last_ports
                                .iter()
                                .filter(|p| !current_ports.contains(p))
                                .cloned()
                                .collect();

                            let _ = app_handle.emit(
                                "ports-changed",
                                serde_json::json!({
                                    "ports": current_ports,
                                    "added": added,
                                    "removed": removed,
                                }),
                            );

                            last_ports = current_ports;
                        }
                    }
                });
            }
```

**注意**：`serde_json` 已在 `Cargo.toml` 中作为 Tauri 的间接依赖存在；如果编译报错找不到 `serde_json`，需要在 `src-tauri/Cargo.toml` 的 `[dependencies]` 中显式添加 `serde_json = "1"`。

### Step 3: 验证 Rust 代码能编译

在 `src-tauri/` 目录下运行：
```bash
cargo check
```
预期：无 error（可能有警告，可忽略）。

---

## Task 2: 前端 — device.js store 订阅端口变化事件

**Files:**
- Modify: `src/store/device.js`

### 前置知识

当前 `device.js` 的 `refreshPorts()` action（第 31-40 行）已包含完整的端口列表更新和"只有 1 个端口时自动选中"的逻辑。我们只需要新增一个 `startPortWatcher()` action，它订阅 Tauri 的 `ports-changed` 事件，并在收到事件时：
1. 用事件 payload 中的 `ports` 更新 `this.ports`
2. 复用现有的自动选中逻辑
3. 如果有新增端口（`added.length > 0`）且当前未连接，触发一个 store-level 的标记让 UI 显示弹窗

### Step 1: 在 device.js 顶部添加 Tauri event 导入

将第 1-3 行：
```js
import { defineStore } from "pinia"
import { connectSerial, disconnectSerial, getSerialStatus, listSerialPorts } from "@/api/unimaster"
import { MODEL_OPTIONS } from "@/constants/unimaster"
```

改为：
```js
import { defineStore } from "pinia"
import { listen } from "@tauri-apps/api/event"
import { connectSerial, disconnectSerial, getSerialStatus, listSerialPorts } from "@/api/unimaster"
import { MODEL_OPTIONS } from "@/constants/unimaster"
```

### Step 2: 在 state 中添加热插拔通知标记

将 `state: () => ({` 块（第 8-24 行），在 `meterLinkReady: false,` 之后追加：
```js
    pendingHotplugPort: "",   // USB 插入检测到的新端口，用于触发确认弹窗
```

完整 state 结果：
```js
  state: () => ({
    models: MODEL_OPTIONS,
    ports: [],
    currentModel: MODEL_OPTIONS[0].value,
    transport: "uart",
    port: "",
    baudRate: ADAPTER_BAUD_RATE,
    canBitrate: 500,
    frameType: "standard",
    connectionStatus: "DISCONNECTED",
    onlineStatus: "未连接适配器",
    lastHeartbeatAt: "--",
    meterCommType: 0x01,
    meterBaudCode: 0x04,
    meterFrameType: 0,
    meterLinkReady: false,
    pendingHotplugPort: "",   // USB 插入检测到的新端口，用于触发确认弹窗
  }),
```

### Step 3: 在 actions 中添加 startPortWatcher() 和 clearHotplugPending()

在现有 `actions` 块的末尾（第 91 行的 `},` 之前），添加以下两个 action：

```js
    clearHotplugPending() {
      this.pendingHotplugPort = ""
    },
    async startPortWatcher() {
      const unlisten = await listen("ports-changed", (event) => {
        const { ports, added } = event.payload

        // 更新端口列表
        this.ports = ports.map((name) => ({ label: name, value: name }))

        // 如果当前选中的端口已不存在，清空选择
        const hasSelectedPort = this.ports.some((item) => item.value === this.port)
        if (!hasSelectedPort) {
          this.port = ""
        }

        // 有新增端口且当前未连接：自动选中（若只有 1 个可用端口）并触发弹窗通知
        if (added.length > 0 && this.connectionStatus !== "CONNECTED") {
          // 自动选中：如果当前没有选中端口且总端口只有 1 个
          if (!this.port && this.ports.length === 1) {
            this.port = this.ports[0].value
          }
          // 如果有新增端口且当前有选中（包括刚才自动选的），设置 pending 提示
          if (this.port) {
            this.pendingHotplugPort = this.port
          }
        }

        // 端口被移除时：如果当前连接的端口消失，自动断开状态（不调用后端，仅更新 UI 状态）
        // 注意：实际串口读写失败会有错误，这里只更新 UI 状态以保持一致
        if (event.payload.removed.length > 0 && this.connectionStatus === "CONNECTED") {
          const connectedPortRemoved = event.payload.removed.some(
            (name) => name === this.port
          )
          if (connectedPortRemoved) {
            this.connectionStatus = "DISCONNECTED"
            this.onlineStatus = "适配器已拔出"
            this.meterLinkReady = false
            this.port = ""
          }
        }
      })

      // 返回 unlisten 函数供调用方在组件卸载时清理
      return unlisten
    },
```

---

## Task 3: 前端 — AppLayout.vue 启动 watcher 并显示确认弹窗

**Files:**
- Modify: `src/layout/AppLayout.vue`

### 前置知识

需要先读 `AppLayout.vue` 了解：
- `onMounted` 中现有的 `refreshPorts()` 调用位置
- `useQuasar` 是否已引入（Quasar 的 dialog 通过 `$q.dialog()` 或组合式 API 的 `useQuasar()` 调用）
- 当前的 script setup 结构

### Step 1: 读取 AppLayout.vue 完整内容

执行前先读文件确认当前结构，再做精准编辑。

### Step 2: 引入 useQuasar 和 watch

在 `<script setup>` 区域，确保以下导入存在（如已有则跳过）：
```js
import { useQuasar } from "quasar"
import { watch, onMounted, onUnmounted } from "vue"
```

在现有变量声明区域添加：
```js
const $q = useQuasar()
let unlistenPortWatcher = null
```

### Step 3: 在 onMounted 中启动端口 watcher

在现有 `onMounted` 中已有 `deviceStore.refreshPorts()` 调用。在该调用**之后**添加 watcher 启动：

```js
unlistenPortWatcher = await deviceStore.startPortWatcher()
```

注意：如果 `onMounted` 当前不是 `async`，需要改为 `onMounted(async () => { ... })`。

### Step 4: 在 onUnmounted 中清理 watcher

添加 `onUnmounted`：
```js
onUnmounted(() => {
  if (unlistenPortWatcher) {
    unlistenPortWatcher()
    unlistenPortWatcher = null
  }
})
```

### Step 5: 用 watch 监听 pendingHotplugPort，弹出确认对话框

在 `<script setup>` 中添加以下 watch（放在 `onMounted` 之后）：

```js
watch(
  () => deviceStore.pendingHotplugPort,
  (port) => {
    if (!port) return

    $q.dialog({
      title: "检测到新设备",
      message: `检测到串口适配器已接入（${port}），是否立即连接？`,
      cancel: {
        label: "稍后连接",
        flat: true,
      },
      ok: {
        label: "立即连接",
        color: "primary",
        unelevated: true,
      },
      persistent: false,
    })
      .onOk(async () => {
        try {
          await deviceStore.toggleConnection()
        } catch (error) {
          notifyError(error)
        }
      })
      .onDismiss(() => {
        deviceStore.clearHotplugPending()
      })
  }
)
```

**注意**：`notifyError` 已在 AppLayout.vue 中使用过（来自 `@/services/ui`），直接使用现有的引入即可。

---

## Task 4: 验证 Cargo.toml 依赖

**Files:**
- Read then maybe modify: `src-tauri/Cargo.toml`

### Step 1: 检查 serde_json 依赖

读取 `src-tauri/Cargo.toml`，确认 `serde_json` 是否已在 `[dependencies]` 中。

如果没有，在 `[dependencies]` 区域添加：
```toml
serde_json = "1"
```

### Step 2: 确认 serialport 依赖已在

检查 `serialport` crate 是否直接在 `[dependencies]` 中（而非只作为插件的间接依赖）。当前 `unimaster.rs` 直接 `use serialport;`，说明它已是直接依赖，无需额外添加。

---

## Task 5: 全量构建验证

### Step 1: Rust 侧编译检查

```bash
# 在 src-tauri/ 目录下
cargo check
```

预期：`Finished` 无 error。

### Step 2: 前端构建检查

```bash
# 在项目根目录
pnpm build
```

预期：构建完成无错误。如果只是开发验证也可以用：
```bash
pnpm dev:web
```

### Step 3: 功能验证

启动 `pnpm dev:desktop`，插拔 USB 串口适配器，验证：
1. 插入 USB 后约 1.5 秒内弹出"检测到新设备"对话框
2. 对话框显示正确的端口名
3. 点击"立即连接"后设备成功连接
4. 点击"稍后连接"后对话框关闭，端口已自动选中，用户可手动点击"连接设备"
5. 拔出 USB 后如已连接则状态自动更新为"适配器已拔出"

---

## 关键注意事项

1. **`@tauri-apps/api/event`**：确认项目已安装 `@tauri-apps/api`。在 `package.json` 中应已存在（因为已有 Tauri IPC 调用）。

2. **后台线程生命周期**：线程在应用退出时会自然结束（Rust 线程随进程退出），无需显式停止机制。

3. **端口比较顺序无关性**：当前实现用 `Vec` 比较 `current_ports != last_ports`，这在端口顺序一致时有效。如果操作系统返回端口顺序不固定，需改为排序后比较：
   ```rust
   let mut sorted_current = current_ports.clone();
   sorted_current.sort();
   let mut sorted_last = last_ports.clone();
   sorted_last.sort();
   if sorted_current != sorted_last {
   ```
   建议**直接使用排序比较**以提高可靠性。

4. **Web 模式兼容**：`pnpm dev:web` 模式下没有 Tauri 后端，`listen()` 会报错。`startPortWatcher()` 需要在 `window.__TAURI__` 存在时才调用。检查方式：
   ```js
   if (window.__TAURI__) {
     unlistenPortWatcher = await deviceStore.startPortWatcher()
   }
   ```

5. **弹窗重复问题**：如果用户未关闭弹窗期间又插入另一个设备，`pendingHotplugPort` 会被覆盖。当前实现通过 `onDismiss` 清理，可接受此行为（后一个端口会覆盖前一个弹窗触发）。
