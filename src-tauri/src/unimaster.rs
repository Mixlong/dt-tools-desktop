use std::{
    path::Path,
    sync::Mutex,
    time::{Duration, Instant},
};

use serde::{Deserialize, Serialize};
use serialport::{ClearBuffer, SerialPort};

const FRAME_START: u8 = 0x55;
const THREE_A_FRAME_START: u8 = 0x3A;
const THREE_A_DEVICE_ADDRESS: u8 = 0x1A;
const THREE_A_END_1: u8 = 0x0D;
const THREE_A_END_2: u8 = 0x0A;
const DEFAULT_TIMEOUT_MS: u64 = 1500;
const VERSION_INFO_CODES: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const FLAG_LABELS: [&str; 16] = [
    "老化进入标志",
    "老化",
    "FCT",
    "半成品",
    "FQC",
    "打标",
    "OQC",
    "IPQC",
    "蓝牙",
    "标志 9",
    "标志 10",
    "标志 11",
    "标志 12",
    "标志 13",
    "标志 14",
    "标志 15",
];

pub struct SerialManager {
    connection: Mutex<Option<SerialConnection>>,
}

struct SerialConnection {
    port_name: String,
    baud_rate: u32,
    port: Box<dyn SerialPort>,
}

impl Default for SerialManager {
    fn default() -> Self {
        Self {
            connection: Mutex::new(None),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerialPortInfo {
    pub port_name: String,
    pub port_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatus {
    pub connected: bool,
    pub port_name: Option<String>,
    pub baud_rate: Option<u32>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FrameExchange {
    pub command: u8,
    pub request_hex: String,
    pub response_hex: String,
    pub payload_hex: String,
    pub response_payload_hex: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleResult {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterConfigReadResponse {
    pub bytes: Vec<u8>,
    pub hex: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionValue {
    pub code: u8,
    pub label: String,
    pub value: String,
    pub raw_hex: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlagValue {
    pub index: u8,
    pub label: String,
    pub value: u32,
    pub hex: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionSnapshot {
    pub app_version: String,
    pub ui_version: String,
    pub version_items: Vec<VersionValue>,
    pub flags: Vec<FlagValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawCommandRequest {
    pub command: u8,
    pub payload: Vec<u8>,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeInitRequest {
    pub comm_type: u8,
    pub baud_code: u8,
    pub frame_type: u8,
    pub power_voltage: u8,
    pub vlk5v_enabled: bool,
    pub protocol_type: u8,
    pub burn_file_type: u8,
    pub model: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OfflinePrepareRequest {
    pub power_voltage: u8,
    pub comm_type: u8,
    pub boot_file_type: u8,
    pub app_file_type: u8,
    pub ui_file_type: u8,
    pub config_file_type: u8,
    pub model: String,
    pub config_comm_type: u8,
    pub config_baud_code: u8,
    pub config_frame_type: u8,
    pub ui_version: String,
    pub files: Vec<UpgradeFile>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteVersionInfoRequest {
    pub code: u8,
    pub value_text: Option<String>,
    pub value_hex: Option<Vec<u8>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteFlagRequest {
    pub position: u8,
    pub value: u32,
    pub shutdown_after_write: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterTransportRequest {
    pub comm_type: u8,
    pub baud_code: u8,
    pub frame_type: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimeUpgradeRequest {
    pub init: RealtimeInitRequest,
    pub files: Vec<UpgradeFile>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeFile {
    pub kind: String,
    pub file_name: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeSummary {
    pub success: bool,
    pub progress: u8,
    pub stage: String,
    pub logs: Vec<String>,
}

#[derive(Debug)]
struct Frame {
    command: u8,
    payload: Vec<u8>,
    raw: Vec<u8>,
}

#[derive(Debug)]
struct ThreeAFrame {
    command: u8,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum UpgradeKind {
    Boot,
    App,
    Ui,
    Config,
}

#[derive(Debug)]
struct DataChunk {
    address: Option<u32>,
    data: Vec<u8>,
}

impl SerialManager {
    pub fn status(&self) -> Result<ConnectionStatus, String> {
        let guard = self
            .connection
            .lock()
            .map_err(|_| "串口状态锁定失败".to_string())?;
        Ok(match guard.as_ref() {
            Some(conn) => ConnectionStatus {
                connected: true,
                port_name: Some(conn.port_name.clone()),
                baud_rate: Some(conn.baud_rate),
            },
            None => ConnectionStatus {
                connected: false,
                port_name: None,
                baud_rate: None,
            },
        })
    }

    pub fn connect(&self, port_name: String, baud_rate: u32) -> Result<ConnectionStatus, String> {
        let port = serialport::new(&port_name, baud_rate)
            .timeout(Duration::from_millis(100))
            .open()
            .map_err(|error| format!("打开串口失败: {error}"))?;

        let mut guard = self
            .connection
            .lock()
            .map_err(|_| "串口连接锁定失败".to_string())?;
        *guard = Some(SerialConnection {
            port_name: port_name.clone(),
            baud_rate,
            port,
        });

        Ok(ConnectionStatus {
            connected: true,
            port_name: Some(port_name),
            baud_rate: Some(baud_rate),
        })
    }

    pub fn disconnect(&self) -> Result<ConnectionStatus, String> {
        let mut guard = self
            .connection
            .lock()
            .map_err(|_| "串口断开锁定失败".to_string())?;
        *guard = None;
        Ok(ConnectionStatus {
            connected: false,
            port_name: None,
            baud_rate: None,
        })
    }

    fn with_port<T, F>(&self, callback: F) -> Result<T, String>
    where
        F: FnOnce(&mut dyn SerialPort) -> Result<T, String>,
    {
        let mut guard = self
            .connection
            .lock()
            .map_err(|_| "串口访问锁定失败".to_string())?;
        let conn = guard
            .as_mut()
            .ok_or_else(|| "请先连接串口适配器".to_string())?;
        callback(conn.port.as_mut())
    }

    pub fn send_command(
        &self,
        command: u8,
        payload: &[u8],
        timeout_ms: u64,
    ) -> Result<FrameExchange, String> {
        self.with_port(|port| {
            let request = build_frame(command, payload)?;
            port.clear(ClearBuffer::All).ok();
            port.write_all(&request)
                .map_err(|error| format!("发送失败: {error}"))?;
            port.flush()
                .map_err(|error| format!("刷新串口失败: {error}"))?;
            let response = read_expected_frame(port, command, timeout_ms)?;

            Ok(FrameExchange {
                command,
                request_hex: bytes_to_hex(&request),
                response_hex: bytes_to_hex(&response.raw),
                payload_hex: bytes_to_hex(payload),
                response_payload_hex: bytes_to_hex(&response.payload),
            })
        })
    }
}

pub fn list_serial_ports() -> Result<Vec<SerialPortInfo>, String> {
    let ports =
        serialport::available_ports().map_err(|error| format!("读取串口列表失败: {error}"))?;
    Ok(ports
        .into_iter()
        .map(|port| SerialPortInfo {
            port_name: port.port_name,
            port_type: format!("{:?}", port.port_type),
        })
        .collect())
}

pub fn read_version_snapshot(manager: &SerialManager) -> Result<VersionSnapshot, String> {
    let frame = handshake_versions(manager)?;
    let payload = hex_to_bytes(&frame.response_payload_hex)?;
    let (app_version, ui_version) = parse_version_response(&payload)?;

    let mut version_items = Vec::new();
    for code in VERSION_INFO_CODES {
        if let Ok(exchange) = manager.send_command(0xB1, &[code], DEFAULT_TIMEOUT_MS) {
            let payload = hex_to_bytes(&exchange.response_payload_hex)?;
            if payload.len() >= 2 {
                let length = payload[1] as usize;
                if payload.len() >= 2 + length {
                    let value_bytes = &payload[2..2 + length];
                    version_items.push(VersionValue {
                        code,
                        label: version_code_label(code).to_string(),
                        value: decode_version_item_value(code, value_bytes),
                        raw_hex: bytes_to_hex(value_bytes),
                    });
                }
            }
        }
    }

    let flags = read_flags(manager).unwrap_or_default();

    Ok(VersionSnapshot {
        app_version,
        ui_version,
        version_items,
        flags,
    })
}

pub fn write_version_info(
    manager: &SerialManager,
    request: WriteVersionInfoRequest,
) -> Result<SimpleResult, String> {
    let mut value = match (request.code, request.value_hex, request.value_text) {
        (5 | 6, Some(bytes), _) => bytes,
        (_, _, Some(text)) => text.into_bytes(),
        _ => return Err("请提供版本信息内容".to_string()),
    };

    if matches!(request.code, 5 | 6) && value.len() != 4 {
        return Err("BLE 升级标志/BLE_CRC 必须为 4 个字节".to_string());
    }
    if !matches!(request.code, 5 | 6) && value.len() > 64 {
        return Err("字符串信息长度不能超过 64 字节".to_string());
    }

    let mut payload = vec![request.code, value.len() as u8];
    payload.append(&mut value);
    let response = manager.send_command(0xB0, &payload, DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    let success = payload.get(1).copied().unwrap_or_default() != 0;
    Ok(SimpleResult {
        success,
        message: if success {
            "写入成功"
        } else {
            "写入失败"
        }
        .to_string(),
    })
}

pub fn read_flags(manager: &SerialManager) -> Result<Vec<FlagValue>, String> {
    let response = manager.send_command(0xB2, &[], DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    if payload.is_empty() {
        return Ok(Vec::new());
    }
    let count = payload[0] as usize;
    let mut flags = Vec::new();
    for index in 0..count.min(16) {
        let start = 1 + index * 4;
        let end = start + 4;
        if end <= payload.len() {
            let raw = &payload[start..end];
            flags.push(FlagValue {
                index: index as u8,
                label: FLAG_LABELS[index].to_string(),
                value: u32::from_be_bytes([raw[0], raw[1], raw[2], raw[3]]),
                hex: bytes_to_hex(raw),
            });
        }
    }
    Ok(flags)
}

pub fn write_flag(
    manager: &SerialManager,
    request: WriteFlagRequest,
) -> Result<SimpleResult, String> {
    if request.position > 15 {
        return Err("标志位只能是 0 到 15".to_string());
    }
    let mut payload = vec![request.position];
    payload.extend_from_slice(&request.value.to_be_bytes());
    payload.push(if request.shutdown_after_write { 1 } else { 0 });
    let response = manager.send_command(0xB3, &payload, DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    let success = payload.first().copied().unwrap_or_default() != 0;
    Ok(SimpleResult {
        success,
        message: if success {
            "写入成功"
        } else {
            "写入失败"
        }
        .to_string(),
    })
}

pub fn switch_language(manager: &SerialManager, language: u8) -> Result<SimpleResult, String> {
    let response = manager.send_command(0xB4, &[language], DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    let success = payload.first().copied().unwrap_or_default() == 1;
    Ok(SimpleResult {
        success,
        message: if success {
            "切换成功"
        } else {
            "切换失败"
        }
        .to_string(),
    })
}

pub fn set_meter_config_transport(
    manager: &SerialManager,
    request: MeterTransportRequest,
) -> Result<SimpleResult, String> {
    let payload = [request.comm_type, request.baud_code, request.frame_type];
    let mut last_error = "串口已连接，但配置链路初始化未收到 0x37 响应".to_string();
    let mut response = None;

    for _ in 0..3 {
        match manager.send_command(0x37, &payload, DEFAULT_TIMEOUT_MS + 1000) {
            Ok(exchange) => {
                response = Some(exchange);
                break;
            }
            Err(error) => {
                last_error = format!(
                    "串口已连接，但配置链路初始化失败: commType=0x{:02X}, baudCode=0x{:02X}, frameType=0x{:02X}; {}",
                    request.comm_type, request.baud_code, request.frame_type, error
                );
                std::thread::sleep(Duration::from_millis(120));
            }
        }
    }

    let response = response.ok_or(last_error)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    let success = payload.first().copied().unwrap_or_default() == 1;
    Ok(SimpleResult {
        success,
        message: if success {
            "仪表配置通讯初始化成功"
        } else {
            "仪表配置通讯初始化失败"
        }
        .to_string(),
    })
}

pub fn read_meter_config(manager: &SerialManager) -> Result<MeterConfigReadResponse, String> {
    let frame = send_3a_command(manager, 0xC2, &[], 0xC3, DEFAULT_TIMEOUT_MS)?;
    Ok(MeterConfigReadResponse {
        hex: bytes_to_hex(&frame.payload),
        bytes: frame.payload,
    })
}

pub fn write_meter_config(manager: &SerialManager, bytes: Vec<u8>) -> Result<SimpleResult, String> {
    if bytes.len() != 54 {
        return Err("仪表参数配置长度必须是 54 字节".to_string());
    }

    let frame = send_3a_command(manager, 0xC0, &bytes, 0xC1, DEFAULT_TIMEOUT_MS)?;
    let success = frame.payload.first().copied().unwrap_or_default() != 0;

    Ok(SimpleResult {
        success,
        message: if success {
            "仪表参数写入成功"
        } else {
            "仪表参数写入失败"
        }
        .to_string(),
    })
}

pub fn set_realtime_screen(manager: &SerialManager, screen: u8) -> Result<SimpleResult, String> {
    let response = manager.send_command(0xAF, &[screen], DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    let success = payload.first().copied().unwrap_or_default() == 1;
    Ok(SimpleResult {
        success,
        message: if success {
            "界面切换成功"
        } else {
            "界面切换失败"
        }
        .to_string(),
    })
}

pub fn read_access_state(manager: &SerialManager) -> Result<SimpleResult, String> {
    let response = manager.send_command(0x20, &[], DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    let message = match payload.first().copied().unwrap_or(0xFF) {
        0x01 => "仪表已接入",
        0x00 => "检测超时，请重新插拔仪表",
        _ => "未接入仪表",
    };
    Ok(SimpleResult {
        success: payload.first().copied().unwrap_or_default() == 0x01,
        message: message.to_string(),
    })
}

pub fn init_realtime_upgrade(
    manager: &SerialManager,
    request: RealtimeInitRequest,
) -> Result<SimpleResult, String> {
    let model = request.model.as_bytes();
    if model.len() > u8::MAX as usize {
        return Err("机型名称过长".to_string());
    }
    let mut payload = vec![
        request.comm_type,
        request.baud_code,
        request.frame_type,
        request.power_voltage,
        if request.vlk5v_enabled { 1 } else { 0 },
        request.protocol_type,
        request.burn_file_type,
        model.len() as u8,
    ];
    payload.extend_from_slice(model);
    let response = manager.send_command(0xA6, &payload, DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    let success = payload.first().copied().unwrap_or_default() == 1;
    Ok(SimpleResult {
        success,
        message: if success {
            "实时烧录参数初始化成功"
        } else {
            "实时烧录参数初始化失败"
        }
        .to_string(),
    })
}

pub fn perform_realtime_upgrade(
    manager: &SerialManager,
    request: RealtimeUpgradeRequest,
) -> Result<UpgradeSummary, String> {
    let mut logs = Vec::new();

    let total_files = request.files.len().max(1);
    for (file_index, file) in request.files.iter().enumerate() {
        let kind = parse_upgrade_kind(&file.kind)?;
        let mut init_request = request.init.clone();
        init_request.burn_file_type = realtime_burn_type(kind);

        let init_result = init_realtime_upgrade(manager, init_request)?;
        logs.push(format!("{}: {}", file.file_name, init_result.message));
        if !init_result.success {
            return Ok(UpgradeSummary {
                success: false,
                progress: ((file_index * 100) / total_files) as u8,
                stage: format!("{} 初始化失败", file.file_name),
                logs,
            });
        }

        let access = read_access_state(manager)?;
        logs.push(access.message.clone());
        if !access.success {
            return Ok(UpgradeSummary {
                success: false,
                progress: ((file_index * 100) / total_files) as u8,
                stage: "等待仪表接入".to_string(),
                logs,
            });
        }

        logs.push(format!("开始处理 {}", file.file_name));

        match kind {
            UpgradeKind::Boot => send_ack_command(manager, 0xE0, &[], "BOOT 擦除", &mut logs)?,
            UpgradeKind::App => send_ack_command(manager, 0xA7, &[], "APP 擦除", &mut logs)?,
            UpgradeKind::Ui => send_ack_command(manager, 0xA9, &[], "UI 擦除", &mut logs)?,
            UpgradeKind::Config => {
                let payload = build_3a_frame(0xC0, &file.data)?;
                send_ack_command(manager, 0xAD, &payload, "配置文件写入", &mut logs)?;
                logs.push(format!(
                    "{} 已写入 ({}/{})",
                    file.file_name,
                    file_index + 1,
                    total_files
                ));
                continue;
            }
        }

        let chunks = build_chunks(&file.file_name, &file.data, kind)?;
        for (chunk_index, chunk) in chunks.iter().enumerate() {
            let (command, payload) = match kind {
                UpgradeKind::Boot => (0xE1, chunk.data.clone()),
                UpgradeKind::App => {
                    if let Some(address) = chunk.address {
                        let mut payload = address.to_be_bytes().to_vec();
                        payload.extend_from_slice(&chunk.data);
                        (0xA8, payload)
                    } else {
                        (0xA8, chunk.data.clone())
                    }
                }
                UpgradeKind::Ui => {
                    let address = chunk.address.unwrap_or_default();
                    let mut payload = address.to_be_bytes().to_vec();
                    payload.extend_from_slice(&chunk.data);
                    (0xAA, payload)
                }
                UpgradeKind::Config => unreachable!(),
            };

            let response = manager.send_command(command, &payload, DEFAULT_TIMEOUT_MS)?;
            let response_payload = hex_to_bytes(&response.response_payload_hex)?;
            if response_payload.first().copied().unwrap_or_default() == 0 {
                return Ok(UpgradeSummary {
                    success: false,
                    progress: ((file_index * 100) / total_files) as u8,
                    stage: format!("{} 写入失败", file.file_name),
                    logs,
                });
            }
            logs.push(format!(
                "{} 分片 {}/{} 写入成功",
                file.file_name,
                chunk_index + 1,
                chunks.len()
            ));
        }

        send_ack_command(manager, 0xAB, &[], "升级数据写入完成", &mut logs)?;
        logs.push(format!(
            "{} 已写入 ({}/{})",
            file.file_name,
            file_index + 1,
            total_files
        ));
    }
    Ok(UpgradeSummary {
        success: true,
        progress: 100,
        stage: "实时升级完成".to_string(),
        logs,
    })
}

pub fn prepare_offline_upgrade(
    manager: &SerialManager,
    request: OfflinePrepareRequest,
) -> Result<UpgradeSummary, String> {
    let mut logs = Vec::new();
    let mut clear_mask = 0u8;
    for file in &request.files {
        clear_mask |= match parse_upgrade_kind(&file.kind)? {
            UpgradeKind::Boot => 0b0001,
            UpgradeKind::App => 0b0010,
            UpgradeKind::Ui => 0b0100,
            UpgradeKind::Config => 0b1000,
        };
    }
    send_bitmask_command(manager, 0x16, clear_mask, "清空升级缓冲区", &mut logs)?;

    let model = request.model.as_bytes();
    if model.len() > 120 {
        return Err("机型名称过长".to_string());
    }
    let mut dut_payload = vec![
        request.power_voltage,
        request.comm_type,
        request.boot_file_type,
        request.app_file_type,
        request.ui_file_type,
        request.config_file_type,
    ];
    dut_payload.extend_from_slice(model);
    send_ack_command(manager, 0x17, &dut_payload, "写入 DUT 机型信息", &mut logs)?;

    let baud_payload = vec![
        request.config_comm_type,
        request.config_baud_code,
        request.config_frame_type,
    ];
    send_ack_command(manager, 0x36, &baud_payload, "写入配置波特率", &mut logs)?;

    if !request.ui_version.trim().is_empty() {
        send_ack_command(
            manager,
            0x35,
            request.ui_version.as_bytes(),
            "写入 UI 版本号",
            &mut logs,
        )?;
    }

    let total_files = request.files.len().max(1);
    let mut dispatch_mask = 0u8;
    for (file_index, file) in request.files.iter().enumerate() {
        let kind = parse_upgrade_kind(&file.kind)?;
        dispatch_mask |= match kind {
            UpgradeKind::Boot => 0b1000,
            UpgradeKind::App => 0b0100,
            UpgradeKind::Ui => 0b0010,
            UpgradeKind::Config => 0b0001,
        };

        if kind == UpgradeKind::Config {
            let title = format!("写入 {}", file.file_name);
            send_ack_command(manager, 0x30, &file.data, &title, &mut logs)?;
        } else {
            let chunks = build_chunks(&file.file_name, &file.data, kind)?;

            for chunk in &chunks {
                let address = chunk.address.unwrap_or_default();
                let mut payload = address.to_be_bytes().to_vec();
                payload.extend_from_slice(&chunk.data);
                let command = match kind {
                    UpgradeKind::Boot => 0x32,
                    UpgradeKind::App => 0x33,
                    UpgradeKind::Ui => 0x31,
                    UpgradeKind::Config => unreachable!(),
                };
                let title = format!("写入 {}", file.file_name);
                send_ack_command(manager, command, &payload, &title, &mut logs)?;
            }
        }

        let finish_item = match kind {
            UpgradeKind::Boot => 0,
            UpgradeKind::App => 1,
            UpgradeKind::Ui => 2,
            UpgradeKind::Config => 3,
        };
        let file_name = file.file_name.as_bytes();
        let mut finish_payload = vec![finish_item, file_name.len() as u8];
        finish_payload.extend_from_slice(file_name);
        send_finish_item_command(
            manager,
            0x34,
            &finish_payload,
            finish_item,
            &format!("结束 {}", file.file_name),
            &mut logs,
        )?;

        logs.push(format!(
            "{} 已同步到离线烧录器 ({}/{})",
            file.file_name,
            file_index + 1,
            total_files
        ));
    }

    send_bitmask_command(manager, 0x14, dispatch_mask, "下发升级项", &mut logs)?;

    Ok(UpgradeSummary {
        success: true,
        progress: 100,
        stage: "离线烧录包准备完成".to_string(),
        logs,
    })
}

fn send_ack_command(
    manager: &SerialManager,
    command: u8,
    payload: &[u8],
    title: &str,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    let response = manager.send_command(command, payload, DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    if payload.first().copied().unwrap_or_default() == 0 {
        return Err(format!("{title}失败"));
    }
    logs.push(format!("{title}成功"));
    Ok(())
}

fn send_bitmask_command(
    manager: &SerialManager,
    command: u8,
    mask: u8,
    title: &str,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    let response = manager.send_command(command, &[mask], DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    if payload.len() < 2 || payload[1] & mask != mask {
        return Err(format!("{title}失败"));
    }
    logs.push(format!("{title}成功"));
    Ok(())
}

fn send_finish_item_command(
    manager: &SerialManager,
    command: u8,
    payload: &[u8],
    item: u8,
    title: &str,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    let response = manager.send_command(command, payload, DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    if payload.len() < 2 || payload[0] != item || payload[1] == 0 {
        return Err(format!("{title}失败"));
    }
    logs.push(format!("{title}成功"));
    Ok(())
}

fn handshake_versions(manager: &SerialManager) -> Result<FrameExchange, String> {
    let mut last_error = "未收到版本信息响应".to_string();
    for _ in 0..10 {
        match manager.send_command(0xA0, &[], 200) {
            Ok(exchange) => return Ok(exchange),
            Err(error) => last_error = error,
        }
        std::thread::sleep(Duration::from_millis(100));
    }
    Err(last_error)
}

fn parse_version_response(payload: &[u8]) -> Result<(String, String), String> {
    if payload.len() < 6 {
        return Err("版本响应长度不足".to_string());
    }
    Ok((
        format_version_triplet(&payload[..3]),
        format_version_triplet(&payload[3..6]),
    ))
}

fn version_code_label(code: u8) -> &'static str {
    match code {
        0 => "DT SN",
        1 => "客户 SN",
        2 => "HW",
        3 => "BOOT",
        4 => "APP",
        5 => "BLE 升级标志",
        6 => "BLE_CRC",
        7 => "QR_CODE",
        8 => "UI",
        _ => "未知类型",
    }
}

fn decode_version_item_value(code: u8, bytes: &[u8]) -> String {
    if matches!(code, 5 | 6) && bytes.len() == 4 {
        return format!(
            "0x{:08X}",
            u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
        );
    }
    String::from_utf8_lossy(bytes)
        .trim_matches(char::from(0))
        .trim()
        .to_string()
}

fn format_version_triplet(bytes: &[u8]) -> String {
    let mut parts = bytes
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>();
    while parts.len() < 3 {
        parts.push("0".to_string());
    }
    parts.join(".")
}

fn realtime_burn_type(kind: UpgradeKind) -> u8 {
    match kind {
        UpgradeKind::Boot => 0,
        UpgradeKind::App => 1,
        UpgradeKind::Ui => 2,
        UpgradeKind::Config => 3,
    }
}

fn send_3a_command(
    manager: &SerialManager,
    command: u8,
    payload: &[u8],
    expected_response: u8,
    timeout_ms: u64,
) -> Result<ThreeAFrame, String> {
    manager.with_port(|port| {
        let request = build_3a_frame(command, payload)?;
        port.clear(ClearBuffer::All).ok();
        port.write_all(&request)
            .map_err(|error| format!("发送 3A 指令失败: {error}"))?;
        port.flush()
            .map_err(|error| format!("刷新串口失败: {error}"))?;
        read_expected_3a_frame(port, expected_response, timeout_ms)
    })
}

fn build_frame(command: u8, payload: &[u8]) -> Result<Vec<u8>, String> {
    if payload.len() > u8::MAX as usize {
        return Err("负载长度不能超过 255 字节".to_string());
    }
    let mut frame = vec![FRAME_START, command, payload.len() as u8];
    frame.extend_from_slice(payload);
    let checksum = frame.iter().fold(0u8, |acc, value| acc ^ value);
    frame.push(checksum);
    Ok(frame)
}

fn build_3a_frame(command: u8, payload: &[u8]) -> Result<Vec<u8>, String> {
    if payload.len() > u8::MAX as usize {
        return Err("3A 指令负载长度不能超过 255 字节".to_string());
    }

    let mut frame = vec![
        THREE_A_FRAME_START,
        THREE_A_DEVICE_ADDRESS,
        command,
        payload.len() as u8,
    ];
    frame.extend_from_slice(payload);

    let checksum = frame[1..]
        .iter()
        .fold(0u16, |acc, value| acc.wrapping_add(*value as u16));
    frame.push((checksum & 0xff) as u8);
    frame.push((checksum >> 8) as u8);
    frame.push(THREE_A_END_1);
    frame.push(THREE_A_END_2);

    Ok(frame)
}

fn read_expected_frame(
    port: &mut dyn SerialPort,
    expected_command: u8,
    timeout_ms: u64,
) -> Result<Frame, String> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms.max(100));
    while Instant::now() < deadline {
        match read_frame(port) {
            Ok(frame) if frame.command == expected_command => return Ok(frame),
            Ok(_) => continue,
            Err(error) if error.contains("timed out") => continue,
            Err(error) => return Err(error),
        }
    }
    Err(format!("等待命令 0x{expected_command:02X} 响应超时"))
}

fn read_expected_3a_frame(
    port: &mut dyn SerialPort,
    expected_command: u8,
    timeout_ms: u64,
) -> Result<ThreeAFrame, String> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms.max(100));
    while Instant::now() < deadline {
        match read_3a_frame(port) {
            Ok(frame) if frame.command == expected_command => return Ok(frame),
            Ok(_) => continue,
            Err(error) if error.contains("timed out") => continue,
            Err(error) => return Err(error),
        }
    }

    Err(format!("等待 3A 命令 0x{expected_command:02X} 响应超时"))
}

fn read_frame(port: &mut dyn SerialPort) -> Result<Frame, String> {
    let mut start = [0u8; 1];
    loop {
        port.read_exact(&mut start)
            .map_err(|error| format!("读取帧头失败: {error}"))?;
        if start[0] == FRAME_START {
            break;
        }
    }

    let mut header = [0u8; 2];
    port.read_exact(&mut header)
        .map_err(|error| format!("读取帧头失败: {error}"))?;
    let command = header[0];
    let length = header[1] as usize;
    let mut payload = vec![0u8; length];
    if length > 0 {
        port.read_exact(&mut payload)
            .map_err(|error| format!("读取负载失败: {error}"))?;
    }
    let mut checksum = [0u8; 1];
    port.read_exact(&mut checksum)
        .map_err(|error| format!("读取校验失败: {error}"))?;

    let mut raw = vec![FRAME_START, command, length as u8];
    raw.extend_from_slice(&payload);
    let expected = raw.iter().fold(0u8, |acc, value| acc ^ value);
    if checksum[0] != expected {
        return Err(format!(
            "校验失败，期望 0x{expected:02X}，收到 0x{:02X}",
            checksum[0]
        ));
    }
    raw.push(checksum[0]);
    Ok(Frame {
        command,
        payload,
        raw,
    })
}

fn read_3a_frame(port: &mut dyn SerialPort) -> Result<ThreeAFrame, String> {
    let mut start = [0u8; 1];
    loop {
        port.read_exact(&mut start)
            .map_err(|error| format!("读取 3A 帧头失败: {error}"))?;
        if start[0] == THREE_A_FRAME_START {
            break;
        }
    }

    let mut header = [0u8; 3];
    port.read_exact(&mut header)
        .map_err(|error| format!("读取 3A 帧头失败: {error}"))?;

    let device_address = header[0];
    if device_address != THREE_A_DEVICE_ADDRESS {
        return Err(format!("3A 设备地址错误: 0x{device_address:02X}"));
    }

    let command = header[1];
    let length = header[2] as usize;
    let mut payload = vec![0u8; length];
    if length > 0 {
        port.read_exact(&mut payload)
            .map_err(|error| format!("读取 3A 负载失败: {error}"))?;
    }

    let mut trailer = [0u8; 4];
    port.read_exact(&mut trailer)
        .map_err(|error| format!("读取 3A 校验失败: {error}"))?;

    let checksum = u16::from_le_bytes([trailer[0], trailer[1]]);
    if trailer[2] != THREE_A_END_1 || trailer[3] != THREE_A_END_2 {
        return Err("3A 帧结束符错误".to_string());
    }

    let mut raw = vec![THREE_A_FRAME_START, device_address, command, length as u8];
    raw.extend_from_slice(&payload);
    let expected = raw[1..]
        .iter()
        .fold(0u16, |acc, value| acc.wrapping_add(*value as u16));
    if checksum != expected {
        return Err(format!(
            "3A 校验失败，期望 0x{expected:04X}，收到 0x{checksum:04X}"
        ));
    }

    Ok(ThreeAFrame {
        command,
        payload,
    })
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|value| format!("{value:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn hex_to_bytes(text: &str) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::new();
    for token in text.split_whitespace() {
        let value = u8::from_str_radix(token, 16)
            .map_err(|error| format!("解析十六进制失败 {token}: {error}"))?;
        bytes.push(value);
    }
    Ok(bytes)
}

fn parse_upgrade_kind(text: &str) -> Result<UpgradeKind, String> {
    match text.to_ascii_lowercase().as_str() {
        "boot" => Ok(UpgradeKind::Boot),
        "app" => Ok(UpgradeKind::App),
        "ui" => Ok(UpgradeKind::Ui),
        "config" => Ok(UpgradeKind::Config),
        _ => Err(format!("不支持的升级类型: {text}")),
    }
}

fn build_chunks(file_name: &str, data: &[u8], kind: UpgradeKind) -> Result<Vec<DataChunk>, String> {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match (kind, extension.as_str()) {
        (UpgradeKind::App, "hex") => build_hex_chunks(data, 112),
        (_, "txt") => build_sequential_chunks(&parse_hex_text_bytes(data)?, 128, true),
        (UpgradeKind::Boot, _) => build_sequential_chunks(data, 128, true),
        (UpgradeKind::Ui, _) => build_sequential_chunks(data, 128, true),
        (UpgradeKind::App, _) => build_sequential_chunks(data, 116, false),
        (UpgradeKind::Config, _) => Err("配置文件不需要分包".to_string()),
    }
}

fn build_sequential_chunks(
    bytes: &[u8],
    chunk_size: usize,
    include_address: bool,
) -> Result<Vec<DataChunk>, String> {
    if bytes.is_empty() {
        return Err("升级文件为空".to_string());
    }
    let mut chunks = Vec::new();
    for (index, chunk) in bytes.chunks(chunk_size).enumerate() {
        chunks.push(DataChunk {
            address: if include_address {
                Some((index * chunk_size) as u32)
            } else {
                None
            },
            data: chunk.to_vec(),
        });
    }
    Ok(chunks)
}

fn parse_hex_text_bytes(data: &[u8]) -> Result<Vec<u8>, String> {
    let text = String::from_utf8_lossy(data);
    let mut hex = String::new();
    for ch in text.chars() {
        if ch.is_ascii_hexdigit() {
            hex.push(ch);
        }
    }
    if hex.len() % 2 != 0 {
        return Err("TXT 文件中的十六进制数据长度不是偶数".to_string());
    }

    let mut bytes = Vec::new();
    for index in (0..hex.len()).step_by(2) {
        let value = u8::from_str_radix(&hex[index..index + 2], 16)
            .map_err(|error| format!("解析 TXT 十六进制失败: {error}"))?;
        bytes.push(value);
    }
    Ok(bytes)
}

fn build_hex_chunks(data: &[u8], chunk_size: usize) -> Result<Vec<DataChunk>, String> {
    let text = String::from_utf8_lossy(data);
    let mut upper = 0u32;
    let mut entries: Vec<(u32, Vec<u8>)> = Vec::new();

    for (line_index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if !line.starts_with(':') {
            return Err(format!("HEX 第 {} 行格式错误", line_index + 1));
        }

        let raw = (1..line.len())
            .step_by(2)
            .map(|index| {
                u8::from_str_radix(&line[index..index + 2], 16)
                    .map_err(|error| format!("HEX 第 {} 行解析失败: {error}", line_index + 1))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if raw.len() < 5 {
            return Err(format!("HEX 第 {} 行长度不足", line_index + 1));
        }
        let byte_count = raw[0] as usize;
        if raw.len() != byte_count + 5 {
            return Err(format!("HEX 第 {} 行长度不匹配", line_index + 1));
        }
        let checksum = raw.iter().fold(0u8, |acc, value| acc.wrapping_add(*value));
        if checksum != 0 {
            return Err(format!("HEX 第 {} 行校验失败", line_index + 1));
        }

        let address = u16::from_be_bytes([raw[1], raw[2]]) as u32;
        let record_type = raw[3];
        let payload = &raw[4..4 + byte_count];

        match record_type {
            0x00 => entries.push((upper + address, payload.to_vec())),
            0x01 => break,
            0x02 => {
                if payload.len() == 2 {
                    upper = u16::from_be_bytes([payload[0], payload[1]]) as u32;
                    upper <<= 4;
                }
            }
            0x04 => {
                if payload.len() == 2 {
                    upper = u16::from_be_bytes([payload[0], payload[1]]) as u32;
                    upper <<= 16;
                }
            }
            _ => {}
        }
    }

    if entries.is_empty() {
        return Err("HEX 文件没有有效数据记录".to_string());
    }

    let mut chunks = Vec::new();
    for (base_address, bytes) in entries {
        for (offset, chunk) in bytes.chunks(chunk_size).enumerate() {
            chunks.push(DataChunk {
                address: Some(base_address + (offset * chunk_size) as u32),
                data: chunk.to_vec(),
            });
        }
    }
    Ok(chunks)
}
