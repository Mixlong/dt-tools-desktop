# `dt_upgrade_cq_config` 字典设计说明

## 1. 设计目标

在若依体系下，只维护一个字典类型：

```text
dictType = dt_upgrade_cq_config
```

该字典类型下固定维护 8 条字典数据，分别对应 CQ 配置串从左到右的 8 个位置。

规则如下：

- `dictLabel`：表示这一位的名称
- `dictValue`：表示这一位的顺序，固定为 `1~8`
- `remark`：表示这一位可选项的完整配置值，使用 JSON 数组维护
- 真正的业务配置内容全部维护在 `remark` 中

---

## 2. CQ 配置串位序定义

CQ 配置串从 `CQ` 后开始，按顺序对应：

| 顺序 | 名称 |
|---|---|
| 1 | 通讯类型 |
| 2 | 波特率 |
| 3 | 帧类型 |
| 4 | 供电电压 |
| 5 | VLK5V开关 |
| 6 | 升级协议类型 |
| 7 | 烧录文件类型 |
| 8 | 帧ID |

示例：

```text
CQ04000111
```

含义：

- 0：通讯类型
- 4：波特率
- 0：帧类型
- 0：供电电压
- 0：VLK5V开关
- 1：升级协议类型
- 1：烧录文件类型
- 1：帧ID

---

## 3. 字典类型定义

字典类型建议如下：

| 字段 | 值 |
|---|---|
| 字典名称 | CQ配置生成器 |
| 字典类型 | `dt_upgrade_cq_config` |
| 状态 | 正常 |
| 备注 | CQ配置生成器8位配置定义 |

---

## 4. 字典数据设计

该字典类型下固定维护 8 条数据：

| dictValue | dictLabel |
|---|---|
| 1 | 通讯类型 |
| 2 | 波特率 |
| 3 | 帧类型 |
| 4 | 供电电压 |
| 5 | VLK5V开关 |
| 6 | 升级协议类型 |
| 7 | 烧录文件类型 |
| 8 | 帧ID |

说明：

- `dictValue` 固定表示第几位
- `dictLabel` 固定表示该位名称
- `remark` 存该位的完整可选项配置
- `dictSort` 建议与 `dictValue` 一致

---

## 5. remark 数据结构约定

### 5.1 普通项结构

普通项统一用 JSON 数组表示：

```json
[
  { "label": "12V", "value": 0 },
  { "label": "24V", "value": 1 }
]
```

字段说明：

- `label`：下拉展示名称
- `value`：CQ 该位实际编码值

---

### 5.2 带类型区分的结构

像 `波特率` 这种，`UART` 和 `CAN` 使用不同展示项，但编码位仍然是第 2 位，因此统一用一个数组，通过 `type` 区分：

```json
[
  { "label": "9600", "value": 4, "type": "uart" },
  { "label": "400k", "value": 7, "type": "can" }
]
```

字段说明：

- `type=uart`：串口场景使用
- `type=can`：CAN 场景使用

同理，`帧类型` 也建议使用 `type` 区分。

---

## 6. 8 条字典数据完整定义

### 6.1 `dictValue = 1`
### 通讯类型

- `dictLabel`: 通讯类型
- `dictValue`: 1

`remark`：

```json
[
  { "label": "3.3V 串口", "value": 0 },
  { "label": "5V 串口", "value": 1 },
  { "label": "CAN", "value": 2 }
]
```

---

### 6.2 `dictValue = 2`
### 波特率

- `dictLabel`: 波特率
- `dictValue`: 2

`remark`：

```json
[
  { "label": "1200", "value": 1, "type": "uart" },
  { "label": "2400", "value": 2, "type": "uart" },
  { "label": "4800", "value": 3, "type": "uart" },
  { "label": "9600", "value": 4, "type": "uart" },
  { "label": "14400", "value": 5, "type": "uart" },
  { "label": "19200", "value": 6, "type": "uart" },
  { "label": "38400", "value": 7, "type": "uart" },
  { "label": "43000", "value": 8, "type": "uart" },
  { "label": "57600", "value": 9, "type": "uart" },
  { "label": "76800", "value": 10, "type": "uart" },
  { "label": "115200", "value": 11, "type": "uart" },
  { "label": "128000", "value": 12, "type": "uart" },

  { "label": "100k", "value": 1, "type": "can" },
  { "label": "125k", "value": 2, "type": "can" },
  { "label": "150k", "value": 3, "type": "can" },
  { "label": "200k", "value": 4, "type": "can" },
  { "label": "250k", "value": 5, "type": "can" },
  { "label": "300k", "value": 6, "type": "can" },
  { "label": "400k", "value": 7, "type": "can" },
  { "label": "500k", "value": 8, "type": "can" },
  { "label": "600k", "value": 9, "type": "can" },
  { "label": "900k", "value": 10, "type": "can" }
]
```

---

### 6.3 `dictValue = 3`
### 帧类型

- `dictLabel`: 帧类型
- `dictValue`: 3

`remark`：

```json
[
  { "label": "串口默认", "value": 0, "type": "uart" },
  { "label": "标准帧", "value": 1, "type": "can" },
  { "label": "扩展帧", "value": 2, "type": "can" }
]
```

---

### 6.4 `dictValue = 4`
### 供电电压

- `dictLabel`: 供电电压
- `dictValue`: 4

`remark`：

```json
[
  { "label": "12V", "value": 0 },
  { "label": "24V", "value": 1 }
]
```

---

### 6.5 `dictValue = 5`
### VLK5V开关

- `dictLabel`: VLK5V开关
- `dictValue`: 5

`remark`：

```json
[
  { "label": "关闭", "value": 0 },
  { "label": "打开", "value": 1 }
]
```

---

### 6.6 `dictValue = 6`
### 升级协议类型

- `dictLabel`: 升级协议类型
- `dictValue`: 6

`remark`：

```json
[
  { "label": "通用彩屏", "value": 1 },
  { "label": "通用段码屏", "value": 2 },
  { "label": "高标", "value": 3 },
  { "label": "华芯微特", "value": 4 },
  { "label": "开阳", "value": 5 },
  { "label": "LIME", "value": 6 },
  { "label": "SPARROW", "value": 7 },
  { "label": "美的", "value": 8 },
  { "label": "IOT", "value": 9 },
  { "label": "K71U", "value": 10 }
]
```

---

### 6.7 `dictValue = 7`
### 烧录文件类型

- `dictLabel`: 烧录文件类型
- `dictValue`: 7

`remark`：

```json
[
  { "label": "BOOT", "value": 0 },
  { "label": "APP", "value": 1 },
  { "label": "UI", "value": 2 },
  { "label": "配置文件", "value": 3 }
]
```

---

### 6.8 `dictValue = 8`
### 帧ID

- `dictLabel`: 帧ID
- `dictValue`: 8

`remark`：

```json
[
  { "label": "默认帧ID", "value": 1 },
  { "label": "自定义帧ID", "value": 2 }
]
```

---

## 7. 前端解析规则

前端获取字典 `dt_upgrade_cq_config` 后，按如下规则使用：

### 7.1 通用规则

1. 按 `dictValue` 排序
2. 对每一条数据的 `remark` 执行 `JSON.parse`
3. 将解析结果作为该位的下拉选项源

---

### 7.2 CQ 位和字段映射

| dictValue | 前端字段 |
|---|---|
| 1 | `commType` |
| 2 | `baudCode` |
| 3 | `frameType` |
| 4 | `powerVoltage` |
| 5 | `vlk5vEnabled` |
| 6 | `protocolType` |
| 7 | `burnFileType` |
| 8 | `frameId` |

---

### 7.3 波特率过滤规则

当 `commType` 不同时，`波特率`从同一条 remark 中按 `type` 过滤：

- `commType = 0` 或 `1` 时，只取 `type = uart`
- `commType = 2` 时，只取 `type = can`

示例逻辑：

```js
const baudOptions = allDictMap[2].filter(item => {
  if (commType === 2) return item.type === "can"
  return item.type === "uart"
})
```

---

### 7.4 帧类型过滤规则

`帧类型`同样按 `type` 过滤：

- `commType = 0` 或 `1` 时，只取 `type = uart`
- `commType = 2` 时，只取 `type = can`

---

## 8. CQ 生成规则

CQ 从 `CQ` 后开始的 8 位，依次取：

1. `commType`
2. `baudCode`
3. `frameType`
4. `powerVoltage`
5. `vlk5vEnabled`
6. `protocolType`
7. `burnFileType`
8. `frameId`

### 8.1 默认帧ID
默认帧ID编码值固定为：

```text
1
```

### 8.2 自定义帧ID
自定义帧ID编码值固定为：

```text
2
```

且 CQ 生成结果格式为：

```text
CQxxxxxxxx-自定义ID-自定义ID
```

示例：

```text
CQ25211112-1801FFF4-1801FFF4
```

---

## 9. 维护规范

### 9.1 必须遵守
- 只能维护一个字典类型：`dt_upgrade_cq_config`
- 该字典类型下固定 8 条字典数据
- `dictValue` 固定为 `1~8`
- `remark` 才是实际维护内容
- `dictLabel` 只表示这一位的名称

### 9.2 不建议做法
- 不要把每个选项拆成单独字典项
- 不要把 `uart`、`can` 拆成两个字典类型
- 不要把业务值维护在 `dictLabel`
- 不要把 `dictValue` 用来存具体下拉选项编码

---

## 10. 验收标准

满足以下条件即为设计正确：

- 后台只看到一个字典类型 `dt_upgrade_cq_config`
- 该字典类型下只有 8 条数据
- 每条数据 `dictValue` 对应 CQ 固定位置
- 每条数据 `remark` 可被前端直接解析
- 波特率在 UART / CAN 下能按 `type` 正确切换
- 生成的 CQ 与当前规则一致

---

## 11. 示例结果校验

### 11.1 UART 默认示例

条件：
- 通讯类型：3.3V 串口 -> `0`
- 波特率：9600 -> `4`
- 帧类型：串口默认 -> `0`
- 供电电压：12V -> `0`
- VLK5V：关闭 -> `0`
- 升级协议类型：通用彩屏 -> `1`
- 烧录文件类型：APP -> `1`
- 帧ID：默认帧ID -> `1`

结果：

```text
CQ04000111
```

---

### 11.2 CAN 默认示例

条件：
- 通讯类型：CAN -> `2`
- 波特率：250k -> `5`
- 帧类型：标准帧 -> `1`
- 供电电压：24V -> `1`
- VLK5V：打开 -> `1`
- 升级协议类型：通用彩屏 -> `1`
- 烧录文件类型：APP -> `1`
- 帧ID：默认帧ID -> `1`

结果：

```text
CQ25111111
```

---

### 11.3 CAN 自定义帧ID示例

条件：
- 通讯类型：CAN -> `2`
- 波特率：250k -> `5`
- 帧类型：标准帧 -> `1`
- 供电电压：24V -> `1`
- VLK5V：打开 -> `1`
- 升级协议类型：通用彩屏 -> `1`
- 烧录文件类型：APP -> `1`
- 帧ID：自定义帧ID -> `2`
- 输入ID：`1801FFF4`

结果：

```text
CQ25111112-1801FFF4-1801FFF4
```
