use std::{
    io::ErrorKind,
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use serialport::{
    ClearBuffer, DataBits, FlowControl, Parity, SerialPort, SerialPortType, StopBits, UsbPortInfo,
};

const FRAME_START: u8 = 0x55;
const THREE_A_FRAME_START: u8 = 0x3A;
const THREE_A_DEVICE_ADDRESS: u8 = 0x1A;
const THREE_A_END_1: u8 = 0x0D;
const THREE_A_END_2: u8 = 0x0A;
const DEFAULT_TIMEOUT_MS: u64 = 1500;
const UPGRADE_ACCESS_WAIT_TIMEOUT_MS: u64 = 8_000;
const UPGRADE_ERASE_MAX_WAIT_MS: u64 = 60_000;
const UPGRADE_PRE_ERASE_SETTLE_MS: u64 = 320;
const UPGRADE_POST_ERASE_SETTLE_MS: u64 = 180;
const KAIYANG_POST_INIT_SETTLE_MS: u64 = 900;
const UPGRADE_ERASE_RETRY_ATTEMPTS: usize = 2;
const UPGRADE_ERASE_RETRY_DELAY_MS: u64 = 420;
const UPGRADE_CHUNK_RETRY_ATTEMPTS: usize = 3;
const UPGRADE_CHUNK_RETRY_DELAY_MS: u64 = 120;
const METER_CONFIG_INIT_ATTEMPTS: usize = 2;
const METER_CONFIG_INIT_TIMEOUT_MS: u64 = 1800;
const METER_CONFIG_ACTIVITY_DETECT_MS: u64 = 220;
const METER_CONFIG_READ_ATTEMPTS: usize = 4;
const METER_CONFIG_READ_TIMEOUT_MS: u64 = 1800;
const METER_CONFIG_PRE_READ_DRAIN_MAX_MS: u64 = 220;
const METER_CONFIG_PRE_READ_QUIET_MS: u64 = 45;
const VERSION_INFO_CODES: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const ALLOWED_USB_SERIAL_IDS: [(u16, u16); 1] = [
    // 当前已验证的适配器：Qinheng CH340 / USB Serial
    (0x1A86, 0x7523),
];
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
const PROGRAM_BURNING_BASE_URL: &str = "http://test-pucs.riding-evolved.com";
const PROGRAM_BURNING_SIGN_KEY: &str = "opeddsaeaddadbcabf";
const PROGRAM_BURNING_CLIENT_ID: &str = "c4d89e9ed4f9d1c8d3e8bcee0684f076";
const UPGRADE_CANCELLED_MESSAGE: &str = "升级已中断";

pub struct SerialManager {
    connection: Mutex<Option<SerialConnection>>,
    upgrade_active: AtomicBool,
    upgrade_cancel_requested: AtomicBool,
    upgrade_trace_logs: Mutex<Vec<String>>,
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
            upgrade_active: AtomicBool::new(false),
            upgrade_cancel_requested: AtomicBool::new(false),
            upgrade_trace_logs: Mutex::new(Vec::new()),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerialPortInfo {
    pub port_name: String,
    pub port_type: String,
    pub usb_vid: Option<u16>,
    pub usb_pid: Option<u16>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatus {
    pub connected: bool,
    pub port_name: Option<String>,
    pub baud_rate: Option<u32>,
}

fn port_type_label(port_type: &SerialPortType) -> String {
    match port_type {
        SerialPortType::UsbPort(_) => "UsbPort".to_string(),
        SerialPortType::PciPort => "PciPort".to_string(),
        SerialPortType::BluetoothPort => "BluetoothPort".to_string(),
        SerialPortType::Unknown => "Unknown".to_string(),
    }
}

fn allowed_usb_serial_rank(usb_port: &UsbPortInfo) -> Option<usize> {
    ALLOWED_USB_SERIAL_IDS
        .iter()
        .position(|(vid, pid)| usb_port.vid == *vid && usb_port.pid == *pid)
}

fn serial_port_usb_metadata(
    port_type: &SerialPortType,
) -> (Option<u16>, Option<u16>, Option<String>, Option<String>) {
    match port_type {
        SerialPortType::UsbPort(usb_port) => (
            Some(usb_port.vid),
            Some(usb_port.pid),
            usb_port.manufacturer.clone(),
            usb_port.product.clone(),
        ),
        _ => (None, None, None, None),
    }
}

fn is_usable_serial_port(port: &serialport::SerialPortInfo) -> bool {
    if port.port_name.trim().is_empty() {
        return false;
    }

    match &port.port_type {
        SerialPortType::UsbPort(_) => true,
        _ => false,
    }
}

fn serial_port_priority(port: &serialport::SerialPortInfo) -> usize {
    match &port.port_type {
        SerialPortType::UsbPort(usb_port) => {
            allowed_usb_serial_rank(usb_port).unwrap_or(ALLOWED_USB_SERIAL_IDS.len() + 1)
        }
        _ => ALLOWED_USB_SERIAL_IDS.len() + 1,
    }
}

fn collect_serial_ports() -> Result<Vec<serialport::SerialPortInfo>, String> {
    let mut ports =
        serialport::available_ports().map_err(|error| format!("读取串口列表失败: {error}"))?;

    // 优先白名单内的 USB 串口适配器，但也保留其他通用 USB 串口，
    // 避免更换芯片方案后端口在下拉框中直接消失。
    ports.retain(is_usable_serial_port);
    ports.sort_by(|left, right| {
        serial_port_priority(left)
            .cmp(&serial_port_priority(right))
            .then_with(|| left.port_name.cmp(&right.port_name))
    });

    Ok(ports)
}

fn map_serial_open_error(message: &str, port_name: &str) -> String {
    let normalized = message.to_ascii_lowercase();

    if normalized.contains("no such file or directory") {
        return format!(
            "打开串口失败: 当前端口 {port_name} 不存在，设备可能已拔出或端口名称已变化，请刷新端口后重试"
        );
    }

    if normalized.contains("device or resource busy")
        || normalized.contains("access denied")
        || normalized.contains("permission denied")
        || normalized.contains("exclusive lock")
        || normalized.contains("resource temporarily unavailable")
        || normalized.contains("port is busy")
    {
        return format!("打开串口失败: 当前端口 {port_name} 被其他程序占用，请关闭占用程序后重试");
    }

    format!("打开串口失败: {message}")
}

pub fn list_serial_port_names() -> Result<Vec<String>, String> {
    Ok(collect_serial_ports()?
        .into_iter()
        .map(|port| port.port_name)
        .collect())
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
    pub logs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterHeartbeatRequest {
    pub comm_type: u8,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterConfigReadRequest {
    pub comm_type: u8,
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
#[allow(dead_code)]
pub struct RealtimeInitRequest {
    pub comm_type: u8,
    pub baud_code: u8,
    pub frame_type: u8,
    pub power_voltage: u8,
    pub vlk5v_enabled: bool,
    pub protocol_type: u8,
    pub burn_file_type: u8,
    pub frame_id: Option<u8>,
    pub cq_code: Option<String>,
    pub model: String,
    pub file_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
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
    pub cq_code: Option<String>,
    pub model: Option<String>,
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UniMasterVersionUpgradeRequest {
    pub kind: String,
    pub file_name: String,
    pub data: Vec<u8>,
    pub chunks: Option<Vec<Vec<u8>>>,
    pub ui_version: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeProgressEvent {
    pub progress: u8,
    pub file_progress: u8,
    pub stage: String,
    pub kind: Option<String>,
    pub file_name: Option<String>,
    pub log: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ProgramBurningApiResponse<T> {
    code: u16,
    msg: String,
    data: T,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProgramBurningManifestItem {
    pub file: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    pub computer_name: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramBurningBundle {
    pub computer_name: String,
    pub files: Vec<ProgramBurningAsset>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgramBurningAsset {
    pub kind: String,
    pub file_name: String,
    pub source_url: String,
    pub size: usize,
    pub text: Option<String>,
    pub bytes: Option<Vec<u8>>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppWriteMode {
    AddressedHex,
    FrameNumberedBin,
}

#[derive(Debug)]
struct DataChunk {
    address: Option<u32>,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum UpgradeProtocol {
    Default,
    GaoBiao,
    KaiYang,
    Iot,
}

fn generate_program_burning_signature(url: &str, timestamp: u64) -> String {
    let raw = format!("url={url}||t={timestamp}||key={PROGRAM_BURNING_SIGN_KEY}");
    let first = format!("{:X}", md5::compute(raw));
    format!("{:X}", md5::compute(&first[8..16]))
}

fn map_program_burning_kind(value: &str) -> Option<&'static str> {
    match value.trim().to_uppercase().as_str() {
        "BOOT" => Some("boot"),
        "APP" => Some("app"),
        "UI" => Some("ui"),
        "JSON" | "CONFIG" => Some("config"),
        _ => None,
    }
}

fn resolve_program_burning_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("{PROGRAM_BURNING_BASE_URL}{url}")
    }
}

fn extract_program_burning_file_name(url: &str, fallback: &str) -> String {
    let sanitized = url.split('?').next().unwrap_or(url);
    sanitized
        .rsplit('/')
        .next()
        .filter(|item| !item.trim().is_empty())
        .unwrap_or(fallback)
        .to_string()
}

pub async fn load_program_burning_bundle(
    code_or_sn: String,
) -> Result<ProgramBurningBundle, String> {
    let code_or_sn = code_or_sn.trim();
    if code_or_sn.is_empty() {
        return Err("请输入识别码或 SN".to_string());
    }

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("系统时间异常: {error}"))?
        .as_secs();
    let manifest_path = format!("/common/downloadPass/{code_or_sn}");
    let sign = generate_program_burning_signature(&manifest_path, timestamp);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(|error| format!("创建在线升级客户端失败: {error}"))?;

    let manifest_response = client
        .get(format!("{PROGRAM_BURNING_BASE_URL}{manifest_path}"))
        .header("t", timestamp.to_string())
        .header("sign", sign)
        .header("id", PROGRAM_BURNING_CLIENT_ID)
        .send()
        .await
        .map_err(|error| format!("获取在线升级资源失败: {error}"))?;

    if !manifest_response.status().is_success() {
        return Err(format!(
            "获取在线升级资源失败，状态码 {}",
            manifest_response.status()
        ));
    }

    let payload = manifest_response
        .json::<ProgramBurningApiResponse<Vec<ProgramBurningManifestItem>>>()
        .await
        .map_err(|error| format!("解析在线升级资源失败: {error}"))?;

    if payload.code != 200 {
        return Err(if payload.msg.trim().is_empty() {
            "在线升级资源返回异常".to_string()
        } else {
            payload.msg
        });
    }

    let computer_name = payload
        .data
        .first()
        .and_then(|item| item.computer_name.clone())
        .unwrap_or_default()
        .trim()
        .to_string();
    let mut files = Vec::new();

    for item in payload.data {
        let Some(kind) = map_program_burning_kind(&item.resource_type) else {
            continue;
        };

        let resource_url = resolve_program_burning_url(&item.file);
        let file_name = extract_program_burning_file_name(&item.file, kind);
        let resource_response = client
            .get(&resource_url)
            .send()
            .await
            .map_err(|error| format!("下载 {file_name} 失败: {error}"))?;

        if !resource_response.status().is_success() {
            return Err(format!(
                "下载 {file_name} 失败，状态码 {}",
                resource_response.status()
            ));
        }

        if kind == "config" {
            let text = resource_response
                .text()
                .await
                .map_err(|error| format!("读取 {file_name} 内容失败: {error}"))?;
            let size = text.as_bytes().len();
            files.push(ProgramBurningAsset {
                kind: kind.to_string(),
                file_name,
                source_url: resource_url,
                size,
                text: Some(text),
                bytes: None,
            });
            continue;
        }

        let bytes = resource_response
            .bytes()
            .await
            .map_err(|error| format!("读取 {file_name} 数据失败: {error}"))?
            .to_vec();
        let size = bytes.len();
        files.push(ProgramBurningAsset {
            kind: kind.to_string(),
            file_name,
            source_url: resource_url,
            size,
            text: None,
            bytes: Some(bytes),
        });
    }

    Ok(ProgramBurningBundle {
        computer_name,
        files,
    })
}

impl SerialManager {
    pub fn is_upgrade_active(&self) -> bool {
        self.upgrade_active.load(Ordering::SeqCst)
    }

    pub fn is_upgrade_cancel_requested(&self) -> bool {
        self.upgrade_cancel_requested.load(Ordering::SeqCst)
    }

    pub fn begin_upgrade(&self) -> Result<(), String> {
        self.upgrade_active
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .map(|_| {
                self.upgrade_cancel_requested.store(false, Ordering::SeqCst);
                self.clear_upgrade_trace_logs();
            })
            .map_err(|_| "已有升级任务正在执行，请稍后重试".to_string())
    }

    pub fn end_upgrade(&self) {
        self.upgrade_cancel_requested.store(false, Ordering::SeqCst);
        self.upgrade_active.store(false, Ordering::SeqCst);
        self.clear_upgrade_trace_logs();
    }

    pub fn request_upgrade_cancel(&self) -> Result<(), String> {
        if !self.is_upgrade_active() {
            return Err("当前没有正在执行的升级任务".to_string());
        }
        self.upgrade_cancel_requested.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn push_upgrade_trace_logs(&self, logs: Vec<String>) {
        if logs.is_empty() || !self.is_upgrade_active() {
            return;
        }

        if let Ok(mut guard) = self.upgrade_trace_logs.lock() {
            guard.extend(logs);
        }
    }

    fn take_upgrade_trace_logs(&self) -> Vec<String> {
        if let Ok(mut guard) = self.upgrade_trace_logs.lock() {
            return std::mem::take(&mut *guard);
        }

        Vec::new()
    }

    fn clear_upgrade_trace_logs(&self) {
        if let Ok(mut guard) = self.upgrade_trace_logs.lock() {
            guard.clear();
        }
    }

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
        let mut port = serialport::new(&port_name, baud_rate)
            .data_bits(DataBits::Eight)
            .parity(Parity::None)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .timeout(Duration::from_millis(10))
            .open()
            .map_err(|error| map_serial_open_error(&error.to_string(), &port_name))?;

        port.write_data_terminal_ready(true).ok();
        port.write_request_to_send(true).ok();

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
        let port_name = conn.port_name.clone();
        let result = callback(conn.port.as_mut());

        if let Err(error) = &result {
            if should_drop_serial_connection(error) {
                let disconnect_log = format!(
                    "[serial][disconnect-on-error] port={} error={}",
                    port_name, error
                );
                eprintln!("{disconnect_log}");
                self.push_upgrade_trace_logs(vec![disconnect_log]);
                *guard = None;
                return Err(format!("串口连接已断开，请重新连接设备后重试: {error}"));
            }
        }

        result
    }

    pub fn send_command(
        &self,
        command: u8,
        payload: &[u8],
        timeout_ms: u64,
    ) -> Result<FrameExchange, String> {
        self.with_port(|port| {
            let request = build_frame(command, payload)?;
            let mut trace_logs = Vec::new();
            if should_trace_serial(command) {
                let tx_log = format!(
                    "[serial][55][tx][cmd=0x{command:02X}] {}",
                    bytes_to_hex(&request)
                );
                eprintln!("{tx_log}");
                trace_logs.push(tx_log);
                for line in build_indexed_frame_trace(command, &request) {
                    eprintln!("{line}");
                    trace_logs.push(line);
                }
            }
            port.clear(ClearBuffer::All).ok();
            if let Err(error) = port.write_all(&request) {
                self.push_upgrade_trace_logs(trace_logs);
                return Err(format!("发送失败: {error}"));
            }
            if let Err(error) = port.flush() {
                self.push_upgrade_trace_logs(trace_logs);
                return Err(format!("刷新串口失败: {error}"));
            }
            let response = match read_expected_frame(
                port,
                command,
                timeout_ms,
                Some(&self.upgrade_cancel_requested),
                Some(&mut trace_logs),
            ) {
                Ok(response) => response,
                Err(error) => {
                    self.push_upgrade_trace_logs(trace_logs);
                    return Err(error);
                }
            };
            self.push_upgrade_trace_logs(trace_logs);

            Ok(FrameExchange {
                command,
                request_hex: bytes_to_hex(&request),
                response_hex: bytes_to_hex(&response.raw),
                payload_hex: bytes_to_hex(payload),
                response_payload_hex: bytes_to_hex(&response.payload),
            })
        })
    }

    pub fn send_command_success(
        &self,
        command: u8,
        payload: &[u8],
        timeout_ms: u64,
    ) -> Result<bool, String> {
        self.send_command_success_with_options(command, payload, timeout_ms, true)
    }

    pub fn send_command_success_with_options(
        &self,
        command: u8,
        payload: &[u8],
        timeout_ms: u64,
        clear_before_send: bool,
    ) -> Result<bool, String> {
        self.with_port(|port| {
            let request = build_frame(command, payload)?;
            let mut trace_logs = Vec::new();
            if should_trace_serial(command) {
                let tx_log = format!(
                    "[serial][55][tx][cmd=0x{command:02X}] {}",
                    bytes_to_hex(&request)
                );
                eprintln!("{tx_log}");
                trace_logs.push(tx_log);
                for line in build_indexed_frame_trace(command, &request) {
                    eprintln!("{line}");
                    trace_logs.push(line);
                }
            }
            if clear_before_send {
                port.clear(ClearBuffer::All).ok();
            }
            if let Err(error) = port.write_all(&request) {
                self.push_upgrade_trace_logs(trace_logs);
                return Err(format!("发送失败: {error}"));
            }
            if let Err(error) = port.flush() {
                self.push_upgrade_trace_logs(trace_logs);
                return Err(format!("刷新串口失败: {error}"));
            }
            let response = match read_expected_frame(
                port,
                command,
                timeout_ms,
                Some(&self.upgrade_cancel_requested),
                Some(&mut trace_logs),
            ) {
                Ok(response) => response,
                Err(error) => {
                    self.push_upgrade_trace_logs(trace_logs);
                    return Err(error);
                }
            };
            self.push_upgrade_trace_logs(trace_logs);
            Ok(response.payload.first().copied().unwrap_or_default() != 0)
        })
    }
}

fn should_drop_serial_connection(message: &str) -> bool {
    let normalized = message.trim().to_ascii_lowercase();
    normalized.contains("broken pipe")
        || normalized.contains("device not configured")
        || normalized.contains("input/output error")
        || normalized.contains("i/o error")
        || normalized.contains("no such device")
        || normalized.contains("bad file descriptor")
}

pub fn list_serial_ports() -> Result<Vec<SerialPortInfo>, String> {
    Ok(collect_serial_ports()?
        .into_iter()
        .map(|port| {
            let (usb_vid, usb_pid, manufacturer, product) =
                serial_port_usb_metadata(&port.port_type);

            SerialPortInfo {
                port_name: port.port_name,
                port_type: port_type_label(&port.port_type),
                usb_vid,
                usb_pid,
                manufacturer,
                product,
            }
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

fn parse_hex_string_bytes(text: &str) -> Result<Vec<u8>, String> {
    let normalized = text.trim();
    if normalized.is_empty() {
        return Ok(Vec::new());
    }
    if normalized.len() % 2 != 0 {
        return Err("十六进制字符串长度不是偶数".to_string());
    }

    let mut bytes = Vec::new();
    for index in (0..normalized.len()).step_by(2) {
        let value = u8::from_str_radix(&normalized[index..index + 2], 16)
            .map_err(|error| format!("解析十六进制字符串失败: {error}"))?;
        bytes.push(value);
    }
    Ok(bytes)
}

fn build_unimaster_app_chunks(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
    let text = String::from_utf8_lossy(data);
    let lines: Vec<&str> = text.lines().collect();
    let mut chunks: Vec<Vec<u8>> = Vec::new();
    let mut upper_address = [0u8; 2];
    let mut current: Vec<u8> = Vec::new();
    let max_frame_len = 116usize;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if !line.starts_with(':') {
            break;
        }
        if line.len() <= 11 {
            continue;
        }

        let record = parse_hex_string_bytes(&line[1..])?;
        if record.len() < 5 {
            continue;
        }
        let record_type = record[3];
        let data_len = record[0] as usize;
        let data_pos = 4usize;
        let address_pos = 1usize;

        if record_type == 0x04 {
            upper_address[0] = *record.get(data_pos).unwrap_or(&0);
            upper_address[1] = *record.get(data_pos + 1).unwrap_or(&0);
            continue;
        }
        if matches!(record_type, 0x01 | 0x05) {
            continue;
        }

        if current.len() + data_len > max_frame_len && !current.is_empty() {
            chunks.push(current);
            current = Vec::new();
        }

        let address = record[address_pos..address_pos + 2].to_vec();
        if current.is_empty() {
            current.extend_from_slice(&upper_address);
            current.extend_from_slice(&address);
        }

        let payload = &record[data_pos..data_pos + data_len];
        current.extend_from_slice(payload);
    }

    if !current.is_empty() {
        chunks.push(current);
    }

    if chunks.is_empty() {
        return Err("UniMaster APP HEX 文件没有有效升级数据".to_string());
    }

    Ok(chunks)
}

fn build_unimaster_ui_text_chunks(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
    let text = String::from_utf8_lossy(data);
    let lines: Vec<&str> = text.lines().collect();
    let mut chunks: Vec<Vec<u8>> = Vec::new();
    let mut builder = String::new();
    let max_frame_len = 128usize;
    let mut empty_lines = 0usize;

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            empty_lines += 1;
            if empty_lines >= 5 {
                break;
            }
            continue;
        }
        empty_lines = 0;

        if line.starts_with("//") {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() < 2 {
            continue;
        }

        let address = parts[0].trim();
        let payload = parts[1].trim();
        if builder.len() + payload.len() > max_frame_len && !builder.is_empty() {
            chunks.push(parse_hex_string_bytes(&builder)?);
            builder.clear();
        }

        if builder.is_empty() {
            builder.push_str(address);
        }
        builder.push_str(payload);
    }

    if !builder.is_empty() {
        chunks.push(parse_hex_string_bytes(&builder)?);
    }

    if chunks.is_empty() {
        return Err("UniMaster UI TXT 文件没有有效升级数据".to_string());
    }

    Ok(chunks)
}

fn build_unimaster_ui_bin_chunks(data: &[u8]) -> Result<Vec<Vec<u8>>, String> {
    if data.is_empty() {
        return Err("UniMaster UI BIN 文件为空".to_string());
    }

    let mut chunks = Vec::new();
    let mut address = 0u32;
    for chunk in data.chunks(128) {
        let mut payload = address.to_be_bytes().to_vec();
        payload.extend_from_slice(chunk);
        chunks.push(payload);
        address += chunk.len() as u32;
    }
    Ok(chunks)
}

fn build_unimaster_version_chunks(
    kind: UpgradeKind,
    file_name: &str,
    data: &[u8],
) -> Result<Vec<Vec<u8>>, String> {
    let extension = parse_file_extension(file_name);
    match kind {
        UpgradeKind::App => build_unimaster_app_chunks(data),
        UpgradeKind::Ui => match extension.as_str() {
            "txt" => build_unimaster_ui_text_chunks(data),
            "bin" => build_unimaster_ui_bin_chunks(data),
            other => Err(format!("UniMaster UI 文件仅支持 txt/bin: {other}")),
        },
        _ => Err("UniMaster 版本管理升级仅支持 APP/UI".to_string()),
    }
}

fn unimaster_version_file_type(kind: UpgradeKind) -> Result<u8, String> {
    match kind {
        UpgradeKind::App => Ok(0x01),
        UpgradeKind::Ui => Ok(0x02),
        _ => Err("UniMaster 版本管理升级仅支持 APP/UI".to_string()),
    }
}

pub fn perform_unimaster_version_upgrade(
    manager: &SerialManager,
    request: UniMasterVersionUpgradeRequest,
) -> Result<UpgradeSummary, String> {
    let kind = parse_upgrade_kind(&request.kind)?;
    let file_type = unimaster_version_file_type(kind)?;
    let chunks = if let Some(chunks) = request.chunks.clone() {
        if chunks.is_empty() {
            build_unimaster_version_chunks(kind, &request.file_name, &request.data)?
        } else {
            chunks
        }
    } else {
        build_unimaster_version_chunks(kind, &request.file_name, &request.data)?
    };
    let mut logs = vec![format!("开始升级 {}", request.file_name)];

    let erase = manager.send_command(0xA1, &[file_type], 120_000)?;
    let erase_payload = hex_to_bytes(&erase.response_payload_hex)?;
    if erase_payload.len() < 2 || erase_payload[0] != file_type || erase_payload[1] != 0x01 {
        return Ok(UpgradeSummary {
            success: false,
            progress: 0,
            stage: format!("{} 擦除失败", request.file_name),
            logs,
        });
    }
    logs.push(format!("{} 擦除成功", request.file_name));

    let mut last_reported_percent = 0u8;
    for (index, chunk) in chunks.iter().enumerate() {
        let mut payload = vec![file_type];
        payload.extend_from_slice(chunk);
        logs.push(format!(
            "[UniMaster][A2][tx][{}/{}] len={} payload={}",
            index + 1,
            chunks.len(),
            payload.len(),
            bytes_to_hex(&payload)
        ));
        let response = manager.send_command(0xA2, &payload, DEFAULT_TIMEOUT_MS)?;
        let response_payload = hex_to_bytes(&response.response_payload_hex)?;
        logs.push(format!(
            "[UniMaster][A2][rx][{}/{}] payload={}",
            index + 1,
            chunks.len(),
            if response.response_payload_hex.is_empty() {
                "--".to_string()
            } else {
                response.response_payload_hex.clone()
            }
        ));
        let write_ok = response_payload.first().copied().unwrap_or_default() == 0x01;
        if !write_ok {
            logs.push(format!(
                "[UniMaster][A2][fail][{}/{}] raw-response={}",
                index + 1,
                chunks.len(),
                response.response_hex
            ));
            return Ok(UpgradeSummary {
                success: false,
                progress: ((index * 100) / chunks.len().max(1)) as u8,
                stage: format!("{} 写入失败", request.file_name),
                logs,
            });
        }

        let percent = (((index + 1) * 100) / chunks.len().max(1)) as u8;
        if percent >= last_reported_percent.saturating_add(10) || index + 1 == chunks.len() {
            last_reported_percent = percent;
            logs.push(format!(
                "{} 写入进度 {}%（{}/{}）",
                request.file_name,
                percent,
                index + 1,
                chunks.len()
            ));
        }
    }

    let mut finish_payload = vec![file_type];
    if file_type == 0x02 {
        let ui_version = request
            .ui_version
            .as_deref()
            .unwrap_or("")
            .trim()
            .as_bytes()
            .iter()
            .copied()
            .take(32)
            .collect::<Vec<_>>();
        finish_payload.extend_from_slice(&ui_version);
    }
    let finish = manager.send_command(0xA3, &finish_payload, DEFAULT_TIMEOUT_MS)?;
    let finish_payload = hex_to_bytes(&finish.response_payload_hex)?;
    if finish_payload.len() < 2 || finish_payload[0] != file_type || finish_payload[1] != 0x01 {
        return Ok(UpgradeSummary {
            success: false,
            progress: 100,
            stage: format!("{} 升级结束失败", request.file_name),
            logs,
        });
    }

    logs.push(format!("{} 升级完成", request.file_name));
    Ok(UpgradeSummary {
        success: true,
        progress: 100,
        stage: format!("{} 升级完成", request.file_name),
        logs,
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
    let started_at = Instant::now();
    let mut last_error = "串口已连接，但配置链路初始化未收到 0x37 响应".to_string();
    let payload = build_meter_config_transport_payload(&request)?;
    let legacy_payload = build_meter_config_transport_legacy_payload(&request);
    let payload_hex = bytes_to_hex(&payload);
    let legacy_payload_hex = bytes_to_hex(&legacy_payload);

    eprintln!(
        "[perf][config-init][start] commType=0x{:02X} baudCode=0x{:02X} frameType=0x{:02X} payloadLen={} legacyPayloadLen={} payload={} legacyPayload={}",
        request.comm_type,
        request.baud_code,
        request.frame_type,
        payload.len(),
        legacy_payload.len(),
        payload_hex,
        legacy_payload_hex
    );

    if detect_three_a_activity(manager, METER_CONFIG_ACTIVITY_DETECT_MS).unwrap_or(false) {
        eprintln!(
            "[perf][config-init][short-circuit] totalMs={} reason=detected-3a-activity",
            started_at.elapsed().as_millis()
        );

        return Ok(SimpleResult {
            success: true,
            message: "检测到设备已处于 3A 通讯状态，跳过 0x37 初始化".to_string(),
        });
    }

    for attempt in 0..METER_CONFIG_INIT_ATTEMPTS {
        let attempt_started_at = Instant::now();
        let mut tried_payloads = vec![format!("primary:{payload_hex}")];
        let response = match manager.send_command(0x37, &payload, METER_CONFIG_INIT_TIMEOUT_MS) {
            Ok(response) => Ok(response),
            Err(error) if payload != legacy_payload && error.contains("等待命令 0x37 响应超时") =>
            {
                tried_payloads.push(format!("legacy:{legacy_payload_hex}"));
                eprintln!(
                    "[perf][config-init][fallback-legacy] attempt={} attemptMs={} totalMs={} reason={} primaryPayload={} legacyPayload={}",
                    attempt + 1,
                    attempt_started_at.elapsed().as_millis(),
                    started_at.elapsed().as_millis(),
                    error,
                    payload_hex,
                    legacy_payload_hex
                );
                manager.send_command(0x37, &legacy_payload, METER_CONFIG_INIT_TIMEOUT_MS)
            }
            Err(error) => Err(error),
        };

        match response {
            Ok(response) => {
                let response_payload = hex_to_bytes(&response.response_payload_hex)?;
                let success = response_payload.first().copied().unwrap_or_default() == 1;
                let mut message = if success {
                    "仪表配置通讯初始化成功".to_string()
                } else {
                    "仪表配置通讯初始化失败".to_string()
                };

                if success {
                    if request.comm_type == 0x01 {
                        if let Err(error) = prime_meter_config_uart(manager) {
                            eprintln!(
                                "[perf][config-init][heartbeat-warn] attempt={} totalMs={} transport=uart error={}",
                                attempt + 1,
                                started_at.elapsed().as_millis(),
                                error
                            );
                            message = format!(
                                "仪表配置通讯初始化成功（UART 心跳建立失败，已忽略: {error}）"
                            );
                        }
                    } else if request.comm_type == 0x02 {
                        if let Err(error) = prime_meter_config_can(manager) {
                            eprintln!(
                                "[perf][config-init][heartbeat-warn] attempt={} totalMs={} transport=can error={}",
                                attempt + 1,
                                started_at.elapsed().as_millis(),
                                error
                            );
                            message = format!(
                                "仪表配置通讯初始化成功（CAN 心跳建立失败，已忽略: {error}）"
                            );
                        }
                    }
                }

                let result = SimpleResult { success, message };

                eprintln!(
                    "[perf][config-init][ok] attempt={} attemptMs={} totalMs={} success={} requestPayloads={} responseHex={} responsePayload={}",
                    attempt + 1,
                    attempt_started_at.elapsed().as_millis(),
                    started_at.elapsed().as_millis(),
                    result.success,
                    tried_payloads.join(" -> "),
                    response.response_hex,
                    response.response_payload_hex
                );

                return Ok(result);
            }
            Err(error) => {
                last_error = format!(
                    "串口已连接，但配置链路初始化失败: commType=0x{:02X}, baudCode=0x{:02X}, frameType=0x{:02X}, tried={}; {}",
                    request.comm_type,
                    request.baud_code,
                    request.frame_type,
                    tried_payloads.join(" -> "),
                    error
                );

                eprintln!(
                    "[perf][config-init][retry] attempt={} attemptMs={} totalMs={} requestPayloads={} error={}",
                    attempt + 1,
                    attempt_started_at.elapsed().as_millis(),
                    started_at.elapsed().as_millis(),
                    tried_payloads.join(" -> "),
                    last_error
                );

                if error.contains("等待命令 0x37 响应超时")
                    && detect_three_a_activity(manager, 600).unwrap_or(false)
                {
                    let result = SimpleResult {
                        success: true,
                        message: "检测到设备已处于 3A 通讯状态，跳过 0x37 初始化".to_string(),
                    };

                    eprintln!(
                        "[perf][config-init][fallback-ok] attempt={} totalMs={} success={}",
                        attempt + 1,
                        started_at.elapsed().as_millis(),
                        result.success
                    );

                    return Ok(result);
                }

                std::thread::sleep(Duration::from_millis(120));
            }
        }
    }

    eprintln!(
        "[perf][config-init][fail] totalMs={} error={}",
        started_at.elapsed().as_millis(),
        last_error
    );

    Err(last_error)
}

pub fn read_meter_config(
    manager: &SerialManager,
    request: MeterConfigReadRequest,
) -> Result<MeterConfigReadResponse, String> {
    let started_at = Instant::now();
    let mut logs = vec![format!(
        "开始读取配置：commType=0x{:02X}",
        request.comm_type
    )];
    eprintln!(
        "[perf][config-read][start] commType=0x{:02X}",
        request.comm_type
    );

    let payload = manager.with_port(|port| {
        let request_frame = build_3a_frame(0xC2, &[])?;
        let heartbeat_frame = if request.comm_type == 0x02 {
            Some(build_3a_frame(0x01, &[0x01])?)
        } else {
            None
        };
        let mut last_error = "等待 3A 命令 0xC3 响应超时".to_string();
        logs.push(format!("读取命令帧: {}", bytes_to_hex(&request_frame)));
        if let Some(frame) = &heartbeat_frame {
            logs.push(format!("CAN 心跳帧: {}", bytes_to_hex(frame)));
        }

        for attempt in 0..METER_CONFIG_READ_ATTEMPTS {
            let attempt_started_at = Instant::now();
            logs.push(format!(
                "读取尝试 {}/{}",
                attempt + 1,
                METER_CONFIG_READ_ATTEMPTS
            ));
            let drained = drain_serial_until_quiet(
                port,
                METER_CONFIG_PRE_READ_DRAIN_MAX_MS,
                METER_CONFIG_PRE_READ_QUIET_MS,
            )?;
            if drained > 0 && should_trace_serial(0xC3) {
                eprintln!(
                    "[serial][3A][pre-read-drain][attempt={}] discarded={}B",
                    attempt + 1,
                    drained
                );
            }
            if drained > 0 {
                logs.push(format!("预读取阶段丢弃历史数据 {}B", drained));
            }

            if let Some(heartbeat_frame) = &heartbeat_frame {
                if should_trace_serial(0x01) {
                    eprintln!(
                        "[serial][3A][tx][cmd=0x01] {}",
                        bytes_to_hex(heartbeat_frame)
                    );
                }
                port.write_all(heartbeat_frame)
                    .map_err(|error| format!("发送配置链路心跳失败: {error}"))?;
                port.flush()
                    .map_err(|error| format!("刷新串口失败: {error}"))?;
                logs.push("已发送 CAN 心跳".to_string());
            }

            if should_trace_serial(0xC2) {
                eprintln!(
                    "[serial][3A][tx][cmd=0xC2] {}",
                    bytes_to_hex(&request_frame)
                );
            }

            port.write_all(&request_frame)
                .map_err(|error| format!("发送 3A 指令失败: {error}"))?;
            port.flush()
                .map_err(|error| format!("刷新串口失败: {error}"))?;
            logs.push("已发送读取配置命令 0xC2".to_string());

            match read_meter_config_payload_compatible(port, METER_CONFIG_READ_TIMEOUT_MS) {
                Ok(payload) => {
                    logs.push(format!(
                        "读取成功：payload={}，耗时 {}",
                        bytes_to_hex(&payload),
                        format_elapsed(attempt_started_at.elapsed()),
                    ));
                    eprintln!(
                        "[perf][config-read][ok] attempt={} attemptMs={} totalMs={} payloadLen={}",
                        attempt + 1,
                        attempt_started_at.elapsed().as_millis(),
                        started_at.elapsed().as_millis(),
                        payload.len()
                    );
                    return Ok(payload);
                }
                Err(error) => {
                    last_error = error;
                    logs.push(format!(
                        "读取失败：第 {} 次尝试，{}，耗时 {}",
                        attempt + 1,
                        last_error,
                        format_elapsed(attempt_started_at.elapsed()),
                    ));
                    eprintln!(
                        "[perf][config-read][retry] attempt={} attemptMs={} totalMs={} error={}",
                        attempt + 1,
                        attempt_started_at.elapsed().as_millis(),
                        started_at.elapsed().as_millis(),
                        last_error
                    );

                    if attempt + 1 < METER_CONFIG_READ_ATTEMPTS {
                        std::thread::sleep(Duration::from_millis(80));
                    }
                }
            }
        }

        eprintln!(
            "[perf][config-read][fail] totalMs={} error={}",
            started_at.elapsed().as_millis(),
            last_error
        );

        Err(last_error)
    })?;
    logs.push(format!(
        "读取完成，总耗时 {}",
        format_elapsed(started_at.elapsed())
    ));

    Ok(MeterConfigReadResponse {
        hex: bytes_to_hex(&payload),
        bytes: payload,
        logs,
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

pub fn send_meter_config_heartbeat(
    manager: &SerialManager,
    request: MeterHeartbeatRequest,
) -> Result<(), String> {
    manager.with_port(|port| {
        let (command, payload) = if request.comm_type == 0x02 {
            (0x01, vec![0x01])
        } else {
            (0xAB, vec![0x01, 0x01, 0x00])
        };

        let frame = build_3a_frame(command, &payload)?;
        if should_trace_serial(command) {
            eprintln!(
                "[serial][3A][heartbeat][cmd=0x{command:02X}] {}",
                bytes_to_hex(&frame)
            );
        }

        port.write_all(&frame)
            .map_err(|error| format!("发送配置链路心跳失败: {error}"))?;
        port.flush()
            .map_err(|error| format!("刷新串口失败: {error}"))?;
        Ok(())
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
    parse_access_state_payload(&payload, "仪表已接入", "检测超时，请重新插拔仪表")
}

pub fn init_realtime_upgrade(
    manager: &SerialManager,
    request: RealtimeInitRequest,
) -> Result<SimpleResult, String> {
    let payload = build_upgrade_param_payload(&request)?;
    let response = manager.send_command(0xA6, &payload, DEFAULT_TIMEOUT_MS)?;
    let response_payload = hex_to_bytes(&response.response_payload_hex)?;
    let success = response_payload.first().copied().unwrap_or_default() == 1;
    let burn_type_label = match request.burn_file_type {
        0 => "BOOT",
        1 => "APP",
        2 => "UI",
        3 => "CFG",
        _ => "UNKNOWN",
    };
    let init_params = format_realtime_init_params(&request);
    let detail = format!(
        "命令 0xA6, 烧录类型={}, 响应载荷={}, 参数={}",
        burn_type_label,
        if response.response_payload_hex.is_empty() {
            "--".to_string()
        } else {
            response.response_payload_hex.clone()
        },
        init_params
    );

    Ok(SimpleResult {
        success,
        message: if success {
            format!("实时烧录参数初始化成功（{}）", detail)
        } else {
            format!("实时烧录参数初始化失败（{}）", detail)
        },
    })
}

fn format_realtime_init_params(request: &RealtimeInitRequest) -> String {
    let cq_code = request
        .cq_code
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("--");

    format!(
        "CQ={}, 通讯类型=0x{:02X}, 波特率=0x{:02X}, 帧类型=0x{:02X}, 供电电压=0x{:02X}, VLK5V={}, 协议类型=0x{:02X}, 烧录类型=0x{:02X}, 帧ID=0x{:02X}",
        cq_code,
        request.comm_type,
        request.baud_code,
        request.frame_type,
        request.power_voltage,
        if request.vlk5v_enabled { 1 } else { 0 },
        request.protocol_type,
        request.burn_file_type,
        request.frame_id.unwrap_or(0),
    )
}

pub fn perform_realtime_upgrade<F>(
    manager: &SerialManager,
    request: RealtimeUpgradeRequest,
    report_progress: F,
) -> Result<UpgradeSummary, String>
where
    F: FnMut(UpgradeProgressEvent),
{
    let mut logs = Vec::new();
    let mut report_progress = report_progress;
    let upgrade_started_at = Instant::now();

    let total_files = request.files.len().max(1);
    for (file_index, file) in request.files.iter().enumerate() {
        ensure_upgrade_not_cancelled(manager)?;
        let file_started_at = Instant::now();
        let kind = parse_upgrade_kind(&file.kind)?;
        let mut init_request = request.init.clone();
        init_request.burn_file_type = realtime_burn_type(kind);
        init_request.file_name = Some(file.file_name.clone());

        emit_upgrade_progress(
            &mut report_progress,
            file_index,
            total_files,
            file,
            0,
            format!("开始处理 {}", file.file_name),
            Some(format!("开始处理 {}", file.file_name)),
        );

        ensure_upgrade_not_cancelled(manager)?;
        let init_cq_code = init_request
            .cq_code
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string);
        let init_protocol_type = init_request.protocol_type;
        if let Some(cq_code) = init_cq_code {
            logs.push(format!("{} 使用 CQ 配置 {}", file.file_name, cq_code));
        }

        let protocol = parse_upgrade_protocol(init_protocol_type);
        if should_init_upgrade_before_access(protocol, kind) {
            let init_started_at = Instant::now();
            let init_result = match init_realtime_upgrade(manager, init_request.clone()) {
                Ok(result) => result,
                Err(error) => {
                    emit_pending_upgrade_trace_logs(
                        manager,
                        &mut report_progress,
                        file_index,
                        total_files,
                        file,
                        5,
                        "升级初始化失败",
                    );
                    return Err(error);
                }
            };
            emit_pending_upgrade_trace_logs(
                manager,
                &mut report_progress,
                file_index,
                total_files,
                file,
                5,
                "升级初始化中",
            );
            eprintln!(
                "[perf][upgrade][init] file={} kind={:?} ms={}",
                file.file_name,
                kind,
                init_started_at.elapsed().as_millis()
            );
            let init_log = format!(
                "{}: {}，耗时 {}",
                file.file_name,
                init_result.message,
                format_elapsed(init_started_at.elapsed()),
            );
            logs.push(init_log.clone());
            emit_upgrade_progress(
                &mut report_progress,
                file_index,
                total_files,
                file,
                5,
                init_result.message.clone(),
                Some(init_log),
            );
            if !init_result.success {
                emit_upgrade_progress(
                    &mut report_progress,
                    file_index,
                    total_files,
                    file,
                    5,
                    format!("{} 初始化失败", file.file_name),
                    Some(format!("{} 初始化失败", file.file_name)),
                );
                return Ok(UpgradeSummary {
                    success: false,
                    progress: ((file_index * 100) / total_files) as u8,
                    stage: format!("{} 初始化失败", file.file_name),
                    logs,
                });
            }
        } else {
            let init_skip_log = format!(
                "{} 使用开阳协议，等待 0x22 成功后再发送 0xA6 初始化",
                file.file_name
            );
            logs.push(init_skip_log.clone());
            emit_upgrade_progress(
                &mut report_progress,
                file_index,
                total_files,
                file,
                5,
                "开阳协议等待文件类型校验".to_string(),
                Some(init_skip_log),
            );
        }

        let screen_started_at = Instant::now();
        let screen = match set_realtime_screen(manager, 0x00) {
            Ok(result) => result,
            Err(error) => {
                emit_pending_upgrade_trace_logs(
                    manager,
                    &mut report_progress,
                    file_index,
                    total_files,
                    file,
                    8,
                    "切换实时界面失败",
                );
                return Err(error);
            }
        };
        emit_pending_upgrade_trace_logs(
            manager,
            &mut report_progress,
            file_index,
            total_files,
            file,
            8,
            "切换实时界面中",
        );
        eprintln!(
            "[perf][upgrade][screen] file={} ms={}",
            file.file_name,
            screen_started_at.elapsed().as_millis()
        );
        let screen_log = format!(
            "{}，耗时 {}",
            screen.message,
            format_elapsed(screen_started_at.elapsed()),
        );
        logs.push(screen_log.clone());
        emit_upgrade_progress(
            &mut report_progress,
            file_index,
            total_files,
            file,
            8,
            screen.message.clone(),
            Some(screen_log),
        );
        if !screen.success {
            return Ok(UpgradeSummary {
                success: false,
                progress: ((file_index * 100) / total_files) as u8,
                stage: "切换实时界面失败".to_string(),
                logs,
            });
        }

        if should_wait_for_passive_access_state(protocol, kind) {
            logs.push("等待仪表上报接入状态".to_string());
            emit_upgrade_progress(
                &mut report_progress,
                file_index,
                total_files,
                file,
                10,
                "等待仪表上报接入状态".to_string(),
                Some("等待仪表上报接入状态".to_string()),
            );

            let access_started_at = Instant::now();
            let access = match wait_for_passive_access_state(manager, UPGRADE_ACCESS_WAIT_TIMEOUT_MS) {
                Ok(result) => result,
                Err(error) => {
                    emit_pending_upgrade_trace_logs(
                        manager,
                        &mut report_progress,
                        file_index,
                        total_files,
                        file,
                        10,
                        "等待仪表接入失败",
                    );
                    return Err(error);
                }
            };
            emit_pending_upgrade_trace_logs(
                manager,
                &mut report_progress,
                file_index,
                total_files,
                file,
                10,
                "等待仪表接入中",
            );
            eprintln!(
                "[perf][upgrade][access] file={} ms={}",
                file.file_name,
                access_started_at.elapsed().as_millis()
            );
            let access_log = format!(
                "{}，耗时 {}",
                access.message,
                format_elapsed(access_started_at.elapsed()),
            );
            logs.push(access_log.clone());
            emit_upgrade_progress(
                &mut report_progress,
                file_index,
                total_files,
                file,
                10,
                access.message.clone(),
                Some(access_log),
            );
            if !access.success {
                return Ok(UpgradeSummary {
                    success: false,
                    progress: ((file_index * 100) / total_files) as u8,
                    stage: "等待仪表接入".to_string(),
                    logs,
                });
            }
        } else {
            let access_skip_log = format!(
                "{} 使用开阳协议，跳过 0x20 接入等待，直接发送 0x22",
                file.file_name
            );
            logs.push(access_skip_log.clone());
            emit_upgrade_progress(
                &mut report_progress,
                file_index,
                total_files,
                file,
                10,
                "开阳协议跳过接入检测".to_string(),
                Some(access_skip_log),
            );
        }

        let chunk_build_started_at = Instant::now();
        let chunks = if matches!(kind, UpgradeKind::Config) {
            Vec::new()
        } else {
            build_realtime_chunks(&file.file_name, &file.data, kind, protocol)?
        };
        let chunk_build_ms = chunk_build_started_at.elapsed().as_millis();
        let average_chunk_size =
            chunks.iter().map(|chunk| chunk.data.len()).sum::<usize>() / chunks.len().max(1);

        let erase_started_at = Instant::now();
        let skip_erase = match kind {
            UpgradeKind::App => protocol_skips_app_erase(protocol),
            UpgradeKind::Ui => protocol_skips_ui_erase(protocol),
            _ => false,
        };
        match kind {
            UpgradeKind::Boot => {
                if let Err(error) =
                    send_erase_command_with_retry(manager, 0xE0, &[], "BOOT 擦除", &mut logs)
                {
                    emit_pending_upgrade_trace_logs(
                        manager,
                        &mut report_progress,
                        file_index,
                        total_files,
                        file,
                        15,
                        "BOOT 擦除失败",
                    );
                    return Err(error);
                }
            }
            UpgradeKind::App => {
                if skip_erase {
                    logs.push(format!(
                        "{} 协议 APP 升级跳过 APP 擦除",
                        protocol_prep_label(protocol)
                    ));
                } else {
                    if let Err(error) =
                        send_erase_command_with_retry(manager, 0xA7, &[], "APP 擦除", &mut logs)
                    {
                        emit_pending_upgrade_trace_logs(
                            manager,
                            &mut report_progress,
                            file_index,
                            total_files,
                            file,
                            15,
                            "APP 擦除失败",
                        );
                        return Err(error);
                    }
                }
            }
            UpgradeKind::Ui => {
                if skip_erase {
                    logs.push(format!(
                        "{} 协议 UI 升级跳过 UI 擦除",
                        protocol_prep_label(protocol)
                    ));
                } else {
                    if let Err(error) =
                        send_erase_command_with_retry(manager, 0xA9, &[], "UI 擦除", &mut logs)
                    {
                        emit_pending_upgrade_trace_logs(
                            manager,
                            &mut report_progress,
                            file_index,
                            total_files,
                            file,
                            15,
                            "UI 擦除失败",
                        );
                        return Err(error);
                    }
                }
            }
            UpgradeKind::Config => {
                let payload = build_3a_frame(0xC0, &file.data)?;
                if let Err(error) =
                    send_ack_command(manager, 0xAD, &payload, "配置文件写入", &mut logs)
                {
                    emit_pending_upgrade_trace_logs(
                        manager,
                        &mut report_progress,
                        file_index,
                        total_files,
                        file,
                        100,
                        "配置文件写入失败",
                    );
                    return Err(error);
                }
                emit_pending_upgrade_trace_logs(
                    manager,
                    &mut report_progress,
                    file_index,
                    total_files,
                    file,
                    100,
                    "配置文件写入中",
                );
                let config_write_log = format!(
                    "{} 数据写入耗时 {}",
                    file.file_name,
                    format_elapsed(erase_started_at.elapsed()),
                );
                logs.push(config_write_log.clone());
                emit_upgrade_progress(
                    &mut report_progress,
                    file_index,
                    total_files,
                    file,
                    100,
                    format!("{} 写入完成", file.file_name),
                    Some(config_write_log),
                );
                let complete_log = format!(
                    "{} 已写入 ({}/{})",
                    file.file_name,
                    file_index + 1,
                    total_files
                );
                logs.push(complete_log);
                logs.push(format!(
                    "{} 全流程耗时 {}",
                    file.file_name,
                    format_elapsed(file_started_at.elapsed()),
                ));
                eprintln!(
                    "[perf][upgrade][config-complete] file={} totalMs={}",
                    file.file_name,
                    file_started_at.elapsed().as_millis()
                );
                continue;
            }
        }
        emit_pending_upgrade_trace_logs(
            manager,
            &mut report_progress,
            file_index,
            total_files,
            file,
            15,
            "擦除处理中",
        );
        eprintln!(
            "[perf][upgrade][erase] file={} kind={:?} ms={}",
            file.file_name,
            kind,
            erase_started_at.elapsed().as_millis()
        );
        let erase_log = format!(
            "{} {}耗时 {}",
            file.file_name,
            if skip_erase { "预处理" } else { "擦除" },
            format_elapsed(erase_started_at.elapsed()),
        );
        logs.push(erase_log.clone());

        emit_upgrade_progress(
            &mut report_progress,
            file_index,
            total_files,
            file,
            15,
            if skip_erase {
                format!("{} 预处理完成", file.file_name)
            } else {
                format!("{} 擦除完成", file.file_name)
            },
            Some(erase_log),
        );
        std::thread::sleep(Duration::from_millis(UPGRADE_POST_ERASE_SETTLE_MS));

        ensure_upgrade_not_cancelled(manager)?;
        if skip_erase {
            let info_started_at = Instant::now();
            if let Err(error) = send_protocol_prepare_command(
                manager,
                protocol,
                kind,
                &file.file_name,
                &file.data,
                chunks.len(),
                &mut logs,
            ) {
                emit_pending_upgrade_trace_logs(
                    manager,
                    &mut report_progress,
                    file_index,
                    total_files,
                    file,
                    15,
                    "协议预处理失败",
                );
                return Err(error);
            }
            if should_send_upgrade_param_after_protocol_prepare(protocol, kind) {
                if let Err(error) = send_upgrade_param_command_with_logs(
                    manager,
                    0xA6,
                    &init_request,
                    "实时烧录参数初始化",
                    &mut logs,
                ) {
                    emit_pending_upgrade_trace_logs(
                        manager,
                        &mut report_progress,
                        file_index,
                        total_files,
                        file,
                        15,
                        "实时烧录参数初始化失败",
                    );
                    return Err(error);
                }
            }
            emit_pending_upgrade_trace_logs(
                manager,
                &mut report_progress,
                file_index,
                total_files,
                file,
                15,
                "协议预处理中",
            );
            let info_log = format!(
                "{} {}耗时 {}",
                file.file_name,
                protocol_prep_label(protocol),
                format_elapsed(info_started_at.elapsed()),
            );
            logs.push(info_log.clone());
            emit_upgrade_progress(
                &mut report_progress,
                file_index,
                total_files,
                file,
                15,
                format!("{} {}完成", file.file_name, protocol_prep_label(protocol)),
                Some(info_log),
            );
            if protocol == UpgradeProtocol::KaiYang {
                logs.push(format!(
                    "{} 开阳协议等待设备进入可写状态 {}ms",
                    file.file_name,
                    KAIYANG_POST_INIT_SETTLE_MS
                ));
                std::thread::sleep(Duration::from_millis(KAIYANG_POST_INIT_SETTLE_MS));
            }
        }
        eprintln!(
            "[perf][upgrade][chunk-build] file={} kind={:?} bytes={} chunks={} avgChunk={} ms={}",
            file.file_name,
            kind,
            file.data.len(),
            chunks.len(),
            average_chunk_size,
            chunk_build_ms
        );
        let chunk_summary = format!(
            "{} 分包 {} 个，平均 {} 字节/包",
            file.file_name,
            chunks.len(),
            average_chunk_size
        );
        logs.push(chunk_summary.clone());
        emit_upgrade_progress(
            &mut report_progress,
            file_index,
            total_files,
            file,
            15,
            format!("{} 分包完成", file.file_name),
            Some(chunk_summary),
        );
        let mut last_reported_progress = 15u8;
        let mut last_logged_percent = 0u8;
        let write_started_at = Instant::now();
        for (chunk_index, chunk) in chunks.iter().enumerate() {
            ensure_upgrade_not_cancelled(manager)?;
            let (command, payload) = match kind {
                UpgradeKind::Boot => (0xE1, chunk.data.clone()),
                UpgradeKind::App => (
                    0xA8,
                    build_app_write_payload(chunk, chunk_index, &file.file_name),
                ),
                UpgradeKind::Ui => (0xAA, build_ui_write_payload(chunk, chunk_index, protocol)),
                UpgradeKind::Config => unreachable!(),
            };

            let write_ok = match send_upgrade_chunk_with_retry(
                manager,
                command,
                &payload,
                protocol,
                kind,
                chunk_index + 1,
                chunks.len(),
            ) {
                Ok(value) => {
                    emit_pending_upgrade_trace_logs(
                        manager,
                        &mut report_progress,
                        file_index,
                        total_files,
                        file,
                        last_reported_progress,
                        "写入分包中",
                    );
                    value
                }
                Err(error) => {
                    emit_pending_upgrade_trace_logs(
                        manager,
                        &mut report_progress,
                        file_index,
                        total_files,
                        file,
                        last_reported_progress,
                        "写入分包失败",
                    );
                    let detail = format!(
                        "{} 第 {}/{} 包写入失败: {}",
                        file.file_name,
                        chunk_index + 1,
                        chunks.len(),
                        error
                    );
                    logs.push(detail.clone());
                    return Err(detail);
                }
            };

            if !write_ok {
                let detail = format!(
                    "{} 第 {}/{} 包写入失败: 设备未确认",
                    file.file_name,
                    chunk_index + 1,
                    chunks.len()
                );
                logs.push(detail.clone());
                return Ok(UpgradeSummary {
                    success: false,
                    progress: ((file_index * 100) / total_files) as u8,
                    stage: detail,
                    logs,
                });
            }

            let chunk_progress =
                (15 + (((chunk_index + 1) * 80) / chunks.len().max(1)) as u8).min(95);

            if chunk_progress > last_reported_progress {
                last_reported_progress = chunk_progress;
                emit_upgrade_progress(
                    &mut report_progress,
                    file_index,
                    total_files,
                    file,
                    chunk_progress,
                    format!("正在写入 {}", file.file_name),
                    None,
                );
            }

            let write_percent = (((chunk_index + 1) * 100) / chunks.len().max(1)) as u8;
            if chunk_progress > last_logged_percent {
                last_logged_percent = chunk_progress;
                let elapsed = write_started_at.elapsed();
                let average_ms = elapsed.as_millis() / (chunk_index + 1) as u128;
                let progress_log = format!(
                    "{} 总进度 {}%（写入 {}%，{}/{}），已用 {}，平均 {} ms/包",
                    file.file_name,
                    chunk_progress,
                    write_percent,
                    chunk_index + 1,
                    chunks.len(),
                    format_elapsed(elapsed),
                    average_ms,
                );
                logs.push(progress_log.clone());
                emit_upgrade_progress(
                    &mut report_progress,
                    file_index,
                    total_files,
                    file,
                    chunk_progress,
                    format!("正在写入 {}", file.file_name),
                    Some(progress_log),
                );
            }
        }

        let write_elapsed = write_started_at.elapsed();
        eprintln!(
            "[perf][upgrade][write] file={} kind={:?} chunks={} totalMs={} avgMsPerChunk={}",
            file.file_name,
            kind,
            chunks.len(),
            write_elapsed.as_millis(),
            write_elapsed.as_millis() / chunks.len().max(1) as u128,
        );
        let write_summary = format!(
            "{} 数据写入耗时 {}，平均 {} ms/包",
            file.file_name,
            format_elapsed(write_elapsed),
            write_elapsed.as_millis() / chunks.len().max(1) as u128,
        );
        logs.push(write_summary.clone());
        emit_upgrade_progress(
            &mut report_progress,
            file_index,
            total_files,
            file,
            95,
            format!("正在写入 {}", file.file_name),
            Some(write_summary),
        );

        ensure_upgrade_not_cancelled(manager)?;
        let finish_started_at = Instant::now();
        if let Err(error) = send_presence_command(manager, 0xAB, &[], "升级数据写入完成", &mut logs)
        {
            emit_pending_upgrade_trace_logs(
                manager,
                &mut report_progress,
                file_index,
                total_files,
                file,
                95,
                "升级完成确认失败",
            );
            return Err(error);
        }
        emit_pending_upgrade_trace_logs(
            manager,
            &mut report_progress,
            file_index,
            total_files,
            file,
            95,
            "升级完成确认中",
        );
        eprintln!(
            "[perf][upgrade][finish] file={} ms={}",
            file.file_name,
            finish_started_at.elapsed().as_millis()
        );
        let complete_log = format!(
            "{} 已写入 ({}/{})",
            file.file_name,
            file_index + 1,
            total_files
        );
        logs.push(complete_log.clone());
        let file_summary = format!(
            "{} 全流程耗时 {}",
            file.file_name,
            format_elapsed(file_started_at.elapsed()),
        );
        logs.push(file_summary.clone());
        eprintln!(
            "[perf][upgrade][file-complete] file={} totalMs={}",
            file.file_name,
            file_started_at.elapsed().as_millis()
        );
        emit_upgrade_progress(
            &mut report_progress,
            file_index,
            total_files,
            file,
            100,
            format!("{} 写入完成", file.file_name),
            Some(file_summary),
        );
    }
    report_progress(UpgradeProgressEvent {
        progress: 100,
        file_progress: 100,
        stage: "实时升级完成".to_string(),
        kind: None,
        file_name: None,
        log: Some("实时升级完成".to_string()),
    });
    Ok(UpgradeSummary {
        success: true,
        progress: 100,
        stage: "实时升级完成".to_string(),
        logs,
    })
    .inspect(|_| {
        eprintln!(
            "[perf][upgrade][all-complete] files={} totalMs={}",
            total_files,
            upgrade_started_at.elapsed().as_millis()
        );
    })
}

fn emit_pending_upgrade_trace_logs<F>(
    manager: &SerialManager,
    report_progress: &mut F,
    file_index: usize,
    total_files: usize,
    file: &UpgradeFile,
    file_progress: u8,
    stage: &str,
) where
    F: FnMut(UpgradeProgressEvent),
{
    for line in manager.take_upgrade_trace_logs() {
        emit_upgrade_progress(
            report_progress,
            file_index,
            total_files,
            file,
            file_progress,
            stage.to_string(),
            Some(line),
        );
    }
}

fn emit_upgrade_progress<F>(
    report_progress: &mut F,
    file_index: usize,
    total_files: usize,
    file: &UpgradeFile,
    file_progress: u8,
    stage: String,
    log: Option<String>,
) where
    F: FnMut(UpgradeProgressEvent),
{
    let overall_progress =
        ((file_index * 100) + usize::from(file_progress)).min(total_files * 100) / total_files;

    report_progress(UpgradeProgressEvent {
        progress: overall_progress as u8,
        file_progress,
        stage,
        kind: Some(file.kind.clone()),
        file_name: Some(file.file_name.clone()),
        log,
    });
}

fn format_elapsed(duration: Duration) -> String {
    let millis = duration.as_millis();
    if millis >= 1_000 {
        format!("{:.2}s", millis as f64 / 1_000.0)
    } else {
        format!("{millis}ms")
    }
}

pub fn prepare_offline_upgrade(
    manager: &SerialManager,
    request: OfflinePrepareRequest,
) -> Result<UpgradeSummary, String> {
    let mut logs = Vec::new();
    let mut clear_mask = 0u8;
    for file in &request.files {
        clear_mask |= match parse_upgrade_kind(&file.kind)? {
            UpgradeKind::Boot => 0b1000,
            UpgradeKind::App => 0b0100,
            UpgradeKind::Ui => 0b0010,
            UpgradeKind::Config => 0b0001,
        };
    }

    let first_file_name = request
        .files
        .first()
        .map(|file| file.file_name.as_str())
        .ok_or_else(|| "请至少选择一个离线烧录文件".to_string())?;
    match send_legacy_upgrade_param_command(manager, 0x17, first_file_name) {
        Ok(true) => logs.push("写入离线烧录参数成功".to_string()),
        Ok(false) | Err(_) => {
            logs.push("标准离线烧录参数格式未通过，尝试旧版兼容格式".to_string());
            send_offline_upgrade_param_command_with_logs(
                manager,
                0x17,
                &request,
                first_file_name,
                "写入离线烧录参数",
                &mut logs,
            )?;
        }
    }

    let screen = set_realtime_screen(manager, 0x01)?;
    logs.push(screen.message.clone());
    if !screen.success {
        return Err(screen.message);
    }

    send_bitmask_command(manager, 0x16, clear_mask, "清空升级缓冲区", &mut logs)?;

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
    if !manager.send_command_success(command, payload, command_timeout_ms(command))? {
        return Err(format!("{title}失败"));
    }
    logs.push(format!("{title}成功"));
    Ok(())
}

fn send_erase_command_with_retry(
    manager: &SerialManager,
    command: u8,
    payload: &[u8],
    title: &str,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    std::thread::sleep(Duration::from_millis(UPGRADE_PRE_ERASE_SETTLE_MS));

    let deadline = Instant::now() + Duration::from_millis(UPGRADE_ERASE_MAX_WAIT_MS);
    let mut last_error = None;

    for attempt in 0..UPGRADE_ERASE_RETRY_ATTEMPTS {
        let remaining_ms = deadline
            .saturating_duration_since(Instant::now())
            .as_millis() as u64;

        if remaining_ms == 0 {
            last_error = Some(format!("{title}超时（等待超过 60 秒）"));
            break;
        }

        match manager.send_command_success_with_options(command, payload, remaining_ms, true) {
            Ok(true) => {
                if attempt > 0 {
                    logs.push(format!("{title}成功（第 {} 次尝试）", attempt + 1));
                } else {
                    logs.push(format!("{title}成功"));
                }
                return Ok(());
            }
            Ok(false) => {
                last_error = Some(format!("{title}失败"));
            }
            Err(error) => {
                last_error = Some(error);
            }
        }

        if attempt + 1 < UPGRADE_ERASE_RETRY_ATTEMPTS {
            logs.push(format!(
                "{title}第 {} 次尝试失败，等待仪表稳定后重试",
                attempt + 1
            ));
            let retry_delay_ms = UPGRADE_ERASE_RETRY_DELAY_MS.min(
                deadline
                    .saturating_duration_since(Instant::now())
                    .as_millis() as u64,
            );
            if retry_delay_ms == 0 {
                last_error = Some(format!("{title}超时（等待超过 60 秒）"));
                break;
            }
            std::thread::sleep(Duration::from_millis(retry_delay_ms));
        }
    }

    Err(last_error.unwrap_or_else(|| format!("{title}失败")))
}

fn send_presence_command(
    manager: &SerialManager,
    command: u8,
    payload: &[u8],
    title: &str,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    manager.send_command(command, payload, command_timeout_ms(command))?;
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

fn send_upgrade_param_command(
    manager: &SerialManager,
    command: u8,
    request: &RealtimeInitRequest,
) -> Result<bool, String> {
    let payload = build_upgrade_param_payload(request)?;
    let response = manager.send_command(command, &payload, DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    Ok(payload.first().copied().unwrap_or_default() == 1)
}

fn send_legacy_upgrade_param_command(
    manager: &SerialManager,
    command: u8,
    file_name: &str,
) -> Result<bool, String> {
    let payload = build_legacy_upgrade_param_payload(file_name)?;
    let response = manager.send_command(command, &payload, DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    Ok(payload.first().copied().unwrap_or_default() == 1)
}

fn send_upgrade_param_command_with_logs(
    manager: &SerialManager,
    command: u8,
    request: &RealtimeInitRequest,
    title: &str,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    let success = send_upgrade_param_command(manager, command, request)?;
    if !success {
        return Err(format!("{title}失败"));
    }
    logs.push(format!("{title}成功"));
    Ok(())
}

fn send_offline_upgrade_param_command(
    manager: &SerialManager,
    command: u8,
    request: &OfflinePrepareRequest,
    file_name: &str,
) -> Result<bool, String> {
    let payload = build_offline_upgrade_param_payload(request, file_name)?;
    let response = manager.send_command(command, &payload, DEFAULT_TIMEOUT_MS)?;
    let payload = hex_to_bytes(&response.response_payload_hex)?;
    Ok(payload.first().copied().unwrap_or_default() == 1)
}

fn send_offline_upgrade_param_command_with_logs(
    manager: &SerialManager,
    command: u8,
    request: &OfflinePrepareRequest,
    file_name: &str,
    title: &str,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    let success = send_offline_upgrade_param_command(manager, command, request, file_name)?;
    if !success {
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

fn parse_access_state_payload(
    payload: &[u8],
    success_message: &str,
    failure_message: &str,
) -> Result<SimpleResult, String> {
    let status = payload.first().copied().unwrap_or(0xFF);
    Ok(SimpleResult {
        success: status != 0x00,
        message: if status == 0x00 {
            failure_message
        } else {
            success_message
        }
        .to_string(),
    })
}

fn ensure_upgrade_not_cancelled(manager: &SerialManager) -> Result<(), String> {
    if manager.is_upgrade_cancel_requested() {
        return Err(UPGRADE_CANCELLED_MESSAGE.to_string());
    }
    Ok(())
}

fn wait_for_passive_access_state(
    manager: &SerialManager,
    timeout_ms: u64,
) -> Result<SimpleResult, String> {
    manager.with_port(|port| {
        let mut trace_logs = Vec::new();
        let frame = match read_expected_frame(
            port,
            0x20,
            timeout_ms,
            Some(&manager.upgrade_cancel_requested),
            Some(&mut trace_logs),
        ) {
            Ok(frame) => frame,
            Err(error) => {
                manager.push_upgrade_trace_logs(trace_logs);
                return Err(error);
            }
        };
        manager.push_upgrade_trace_logs(trace_logs);
        parse_access_state_payload(
            &frame.payload,
            "收到仪表接入上报",
            "收到仪表未接入上报，请重新插拔仪表",
        )
    })
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
        let mut trace_logs = Vec::new();
        if should_trace_serial(command) {
            let tx_log = format!(
                "[serial][3A][tx][cmd=0x{command:02X}] {}",
                bytes_to_hex(&request)
            );
            eprintln!("{tx_log}");
            trace_logs.push(tx_log);
        }
        port.clear(ClearBuffer::All).ok();
        if let Err(error) = port.write_all(&request) {
            manager.push_upgrade_trace_logs(trace_logs);
            return Err(format!("发送 3A 指令失败: {error}"));
        }
        if let Err(error) = port.flush() {
            manager.push_upgrade_trace_logs(trace_logs);
            return Err(format!("刷新串口失败: {error}"));
        }
        let response = match read_expected_3a_frame(
            port,
            expected_response,
            timeout_ms,
            Some(&manager.upgrade_cancel_requested),
            Some(&mut trace_logs),
        ) {
            Ok(response) => response,
            Err(error) => {
                manager.push_upgrade_trace_logs(trace_logs);
                return Err(error);
            }
        };
        manager.push_upgrade_trace_logs(trace_logs);
        Ok(response)
    })
}

fn prime_meter_config_uart(manager: &SerialManager) -> Result<(), String> {
    let mut last_error = "UART 心跳初始化失败".to_string();
    for _ in 0..5 {
        match send_3a_command(manager, 0xAB, &[0x01, 0x01, 0x00], 0xAB, 300) {
            Ok(_) => return Ok(()),
            Err(error) => {
                last_error = error;
                std::thread::sleep(Duration::from_millis(120));
            }
        }
    }

    Err(last_error)
}

fn prime_meter_config_can(manager: &SerialManager) -> Result<(), String> {
    let mut last_error = "CAN 心跳初始化失败".to_string();
    for _ in 0..5 {
        match send_3a_command(manager, 0x01, &[0x01], 0x01, 300) {
            Ok(_) => return Ok(()),
            Err(error) => {
                last_error = error;
                std::thread::sleep(Duration::from_millis(120));
            }
        }
    }

    Err(last_error)
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
    cancel_requested: Option<&AtomicBool>,
    mut trace_logs: Option<&mut Vec<String>>,
) -> Result<Frame, String> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms.max(100));
    let mut buffer = Vec::new();
    while Instant::now() < deadline {
        if cancel_requested.is_some_and(|flag| flag.load(Ordering::SeqCst)) {
            return Err(UPGRADE_CANCELLED_MESSAGE.to_string());
        }
        let chunk_start = buffer.len();
        let appended = append_serial_bytes(port, &mut buffer)?;
        if appended > 0 {
            trace_serial_chunk(
                "55",
                expected_command,
                &buffer[chunk_start..],
                trace_logs.as_deref_mut(),
            );
        }
        while let Some(frame) = extract_frame(&mut buffer)? {
            if should_trace_serial(expected_command) {
                let rx_log = format!(
                    "[serial][55][rx-frame][cmd=0x{:02X}] {}",
                    frame.command,
                    bytes_to_hex(&frame.raw)
                );
                eprintln!("{rx_log}");
                if let Some(logs) = trace_logs.as_deref_mut() {
                    logs.push(rx_log);
                }
            }
            if frame.command == expected_command {
                return Ok(frame);
            }
        }
    }
    if should_trace_serial(expected_command) && !buffer.is_empty() {
        let timeout_log = format!(
            "[serial][55][timeout][expect=0x{expected_command:02X}] buffered={}",
            bytes_to_hex(&buffer)
        );
        eprintln!("{timeout_log}");
        if let Some(logs) = trace_logs.as_deref_mut() {
            logs.push(timeout_log);
        }
    }
    Err(format!("等待命令 0x{expected_command:02X} 响应超时"))
}

fn read_expected_3a_frame(
    port: &mut dyn SerialPort,
    expected_command: u8,
    timeout_ms: u64,
    cancel_requested: Option<&AtomicBool>,
    mut trace_logs: Option<&mut Vec<String>>,
) -> Result<ThreeAFrame, String> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms.max(100));
    let mut buffer = Vec::new();
    while Instant::now() < deadline {
        if cancel_requested.is_some_and(|flag| flag.load(Ordering::SeqCst)) {
            return Err(UPGRADE_CANCELLED_MESSAGE.to_string());
        }
        let chunk_start = buffer.len();
        let appended = append_serial_bytes(port, &mut buffer)?;
        if appended > 0 {
            trace_serial_chunk(
                "3A",
                expected_command,
                &buffer[chunk_start..],
                trace_logs.as_deref_mut(),
            );
        }
        while let Some(frame) = extract_3a_frame(&mut buffer)? {
            if should_trace_serial(expected_command) {
                let rx_log = format!(
                    "[serial][3A][rx-frame][cmd=0x{:02X}] {}",
                    frame.command,
                    bytes_to_hex(&frame.payload)
                );
                eprintln!("{rx_log}");
                if let Some(logs) = trace_logs.as_deref_mut() {
                    logs.push(rx_log);
                }
            }
            if frame.command == expected_command {
                return Ok(frame);
            }
        }
    }

    if should_trace_serial(expected_command) && !buffer.is_empty() {
        let timeout_log = format!(
            "[serial][3A][timeout][expect=0x{expected_command:02X}] buffered={}",
            bytes_to_hex(&buffer)
        );
        eprintln!("{timeout_log}");
        if let Some(logs) = trace_logs.as_deref_mut() {
            logs.push(timeout_log);
        }
    }
    Err(format!("等待 3A 命令 0x{expected_command:02X} 响应超时"))
}

fn read_meter_config_payload_compatible(
    port: &mut dyn SerialPort,
    timeout_ms: u64,
) -> Result<Vec<u8>, String> {
    let deadline = Instant::now() + Duration::from_millis(timeout_ms.max(100));
    let mut buffer = Vec::new();

    while Instant::now() < deadline {
        let chunk_start = buffer.len();
        let appended = append_serial_bytes(port, &mut buffer)?;
        if appended > 0 {
            trace_serial_chunk("3A", 0xC3, &buffer[chunk_start..], None);
        }

        while let Some(frame) = extract_3a_frame(&mut buffer)? {
            if should_trace_serial(0xC3) {
                eprintln!(
                    "[serial][3A][rx-frame][cmd=0x{:02X}] {}",
                    frame.command,
                    bytes_to_hex(&frame.payload)
                );
            }
            if frame.command == 0xC3 {
                return Ok(frame.payload);
            }
        }

        if let Some(payload) = extract_legacy_meter_config_payload(&buffer) {
            if should_trace_serial(0xC3) {
                eprintln!(
                    "[serial][3A][legacy-c3][payload] {}",
                    bytes_to_hex(&payload)
                );
            }
            buffer.clear();
            return Ok(payload);
        }
    }

    if should_trace_serial(0xC3) && !buffer.is_empty() {
        eprintln!(
            "[serial][3A][timeout][expect=0xC3] buffered={}",
            bytes_to_hex(&buffer)
        );
    }

    Err("等待 3A 命令 0xC3 响应超时".to_string())
}

fn extract_legacy_meter_config_payload(buffer: &[u8]) -> Option<Vec<u8>> {
    const LEGACY_CONFIG_PAYLOAD_LEN: usize = 49;

    let start_index = buffer.windows(4).position(|window| {
        window[0] == THREE_A_FRAME_START && window[1] == THREE_A_DEVICE_ADDRESS && window[2] == 0xC3
    })?;

    let length = *buffer.get(start_index + 3)? as usize;
    if length < LEGACY_CONFIG_PAYLOAD_LEN {
        return None;
    }

    let payload_start = start_index + 4;
    let payload_end = payload_start + LEGACY_CONFIG_PAYLOAD_LEN;
    if buffer.len() < payload_end {
        return None;
    }

    Some(buffer[payload_start..payload_end].to_vec())
}

fn detect_three_a_activity(manager: &SerialManager, timeout_ms: u64) -> Result<bool, String> {
    manager.with_port(|port| {
        let deadline = Instant::now() + Duration::from_millis(timeout_ms.max(100));
        let mut buffer = Vec::new();

        while Instant::now() < deadline {
            let appended = append_serial_bytes(port, &mut buffer)?;
            if appended == 0 {
                continue;
            }

            while let Some(frame) = extract_3a_frame(&mut buffer)? {
                if should_trace_serial(frame.command) {
                    eprintln!(
                        "[serial][3A][detected][cmd=0x{:02X}] {}",
                        frame.command,
                        bytes_to_hex(&frame.payload)
                    );
                }
                return Ok(true);
            }
        }

        Ok(false)
    })
}

fn append_serial_bytes(port: &mut dyn SerialPort, buffer: &mut Vec<u8>) -> Result<usize, String> {
    let mut chunk = [0u8; 256];
    match port.read(&mut chunk) {
        Ok(0) => Ok(0),
        Ok(read) => {
            buffer.extend_from_slice(&chunk[..read]);
            Ok(read)
        }
        Err(error) if error.kind() == ErrorKind::TimedOut => Ok(0),
        Err(error) => Err(format!("读取串口数据失败: {error}")),
    }
}

fn trace_serial_chunk(
    protocol: &str,
    expected_command: u8,
    chunk: &[u8],
    trace_logs: Option<&mut Vec<String>>,
) {
    if chunk.is_empty() || !should_trace_serial(expected_command) {
        return;
    }

    let chunk_log = format!(
        "[serial][{protocol}][rx-chunk][expect=0x{expected_command:02X}] {}",
        bytes_to_hex(chunk)
    );
    eprintln!("{chunk_log}");
    if let Some(logs) = trace_logs {
        logs.push(chunk_log);
    }
}

fn drain_serial_until_quiet(
    port: &mut dyn SerialPort,
    max_wait_ms: u64,
    quiet_ms: u64,
) -> Result<usize, String> {
    let deadline = Instant::now() + Duration::from_millis(max_wait_ms.max(20));
    let quiet_window = Duration::from_millis(quiet_ms.max(10));
    let mut total_drained = 0usize;
    let mut last_rx_at = Instant::now();

    loop {
        let mut chunk = [0u8; 256];
        match port.read(&mut chunk) {
            Ok(0) => {}
            Ok(read) => {
                total_drained += read;
                last_rx_at = Instant::now();
                continue;
            }
            Err(error) if error.kind() == ErrorKind::TimedOut => {}
            Err(error) => return Err(format!("读取串口数据失败: {error}")),
        }

        let now = Instant::now();
        if total_drained == 0 || now.duration_since(last_rx_at) >= quiet_window || now >= deadline {
            return Ok(total_drained);
        }
    }
}

fn should_trace_serial(command: u8) -> bool {
    if std::env::var("DT_TRACE_SERIAL")
        .map(|value| value == "1")
        .unwrap_or(false)
    {
        return true;
    }

    matches!(
        command,
        0x14 | 0x16
            | 0x17
            | 0x21
            | 0x22
            | 0x20
            | 0x30
            | 0x34
            | 0x37
            | 0xA6
            | 0xA7
            | 0xA8
            | 0xA9
            | 0xAA
            | 0xAB
            | 0xAD
            | 0xAF
            | 0xC0
            | 0xC1
            | 0xE0
            | 0xE1
    )
}

fn build_legacy_upgrade_param_payload(file_name: &str) -> Result<Vec<u8>, String> {
    let normalized = Path::new(file_name)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or(file_name)
        .trim()
        .to_string();

    if normalized.is_empty() {
        return Err("升级文件名为空".to_string());
    }

    let bytes = normalized.into_bytes();
    if bytes.len() > u8::MAX as usize {
        return Err("升级文件名过长".to_string());
    }

    Ok(bytes)
}

fn build_model_cq_payload(model: &str, cq_code: &str) -> Result<Vec<u8>, String> {
    let normalized_model = model.trim();
    if normalized_model.is_empty() {
        return Err("APP/UI 升级缺少型号".to_string());
    }

    let normalized_cq_code = cq_code.trim();
    if normalized_cq_code.is_empty() {
        return Err("APP/UI 升级缺少 CQ 配置串".to_string());
    }

    let combined_cq_code = format!(
        "{}_{}",
        normalized_model.to_ascii_uppercase(),
        normalized_cq_code.to_ascii_uppercase()
    );
    let cq_bytes = combined_cq_code.as_bytes();
    if cq_bytes.len() > u8::MAX as usize {
        return Err("型号 + CQ 配置串过长".to_string());
    }

    Ok(cq_bytes.to_vec())
}

fn build_upgrade_param_payload(request: &RealtimeInitRequest) -> Result<Vec<u8>, String> {
    let burn_file_type = request.burn_file_type;
    let maybe_cq_code = request
        .cq_code
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    if matches!(burn_file_type, 1 | 2) {
        let cq_code = maybe_cq_code.ok_or_else(|| "APP/UI 升级缺少 CQ 配置串".to_string())?;
        return build_model_cq_payload(&request.model, cq_code);
    }

    let file_name = request
        .file_name
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or(request.model.as_str());
    build_legacy_upgrade_param_payload(file_name)
}

fn build_meter_config_transport_payload(
    request: &MeterTransportRequest,
) -> Result<Vec<u8>, String> {
    let maybe_cq_code = request
        .cq_code
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let maybe_model = request
        .model
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    if let (Some(model), Some(cq_code)) = (maybe_model, maybe_cq_code) {
        return build_model_cq_payload(model, cq_code);
    }

    Ok(build_meter_config_transport_legacy_payload(request))
}

fn build_meter_config_transport_legacy_payload(request: &MeterTransportRequest) -> Vec<u8> {
    vec![request.comm_type, request.baud_code, request.frame_type]
}

fn build_offline_upgrade_param_payload(
    request: &OfflinePrepareRequest,
    file_name: &str,
) -> Result<Vec<u8>, String> {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();

    if extension.is_empty() {
        return Err("离线烧录文件后缀为空".to_string());
    }

    let file_type_code = match extension.as_str() {
        "txt" => 0,
        "bin" => 1,
        "json" => 2,
        "ini" => 3,
        "hex" => 4,
        _ => return Err(format!("不支持的离线烧录文件后缀: {extension}")),
    };

    let power = if request.power_voltage == 0xFF {
        1
    } else {
        request.power_voltage
    };
    let communicate_type = if request.comm_type == 0x02 { 1 } else { 0 };
    let mut payload = vec![power, communicate_type, file_type_code];
    payload.extend_from_slice(extension.as_bytes());
    payload.extend_from_slice(request.model.trim().as_bytes());
    Ok(payload)
}

fn command_timeout_ms(command: u8) -> u64 {
    match command {
        0xAB => 120_000,
        0xE0 | 0xA7 | 0xA9 => 120_000,
        _ => DEFAULT_TIMEOUT_MS,
    }
}

fn upgrade_chunk_timeout_ms(
    command: u8,
    protocol: UpgradeProtocol,
    kind: UpgradeKind,
    chunk_number: usize,
) -> u64 {
    match command {
        0xA8 if protocol == UpgradeProtocol::KaiYang && kind == UpgradeKind::App && chunk_number == 1 => 6_000,
        0xAA if protocol == UpgradeProtocol::KaiYang && kind == UpgradeKind::Ui && chunk_number == 1 => 6_000,
        0xAA => 3_000,
        0xA8 | 0xE1 => 2_000,
        _ => DEFAULT_TIMEOUT_MS,
    }
}

fn send_upgrade_chunk_with_retry(
    manager: &SerialManager,
    command: u8,
    payload: &[u8],
    protocol: UpgradeProtocol,
    kind: UpgradeKind,
    chunk_number: usize,
    total_chunks: usize,
) -> Result<bool, String> {
    let timeout_ms = upgrade_chunk_timeout_ms(command, protocol, kind, chunk_number);
    let mut last_error = None;

    for attempt in 0..UPGRADE_CHUNK_RETRY_ATTEMPTS {
        match manager.send_command_success_with_options(command, payload, timeout_ms, attempt > 0) {
            Ok(true) => return Ok(true),
            Ok(false) => {
                if attempt + 1 == UPGRADE_CHUNK_RETRY_ATTEMPTS {
                    return Ok(false);
                }
                eprintln!(
                    "[serial][55][retry][cmd=0x{command:02X}] chunk={}/{} attempt={} reason=nack",
                    chunk_number,
                    total_chunks,
                    attempt + 1
                );
            }
            Err(error) => {
                if error.contains("请先连接串口适配器") {
                    return Err(last_error.unwrap_or(error));
                }
                if attempt + 1 == UPGRADE_CHUNK_RETRY_ATTEMPTS {
                    return Err(last_error.unwrap_or(error));
                }
                eprintln!(
                    "[serial][55][retry][cmd=0x{command:02X}] chunk={}/{} attempt={} reason={}",
                    chunk_number,
                    total_chunks,
                    attempt + 1,
                    error
                );
                last_error = Some(error);
            }
        }

        std::thread::sleep(Duration::from_millis(UPGRADE_CHUNK_RETRY_DELAY_MS));
    }

    Err(last_error.unwrap_or_else(|| "升级分包写入失败".to_string()))
}

fn extract_frame(buffer: &mut Vec<u8>) -> Result<Option<Frame>, String> {
    loop {
        let Some(start_index) = buffer.iter().position(|value| *value == FRAME_START) else {
            buffer.clear();
            return Ok(None);
        };

        if start_index > 0 {
            buffer.drain(..start_index);
        }

        if buffer.len() < 4 {
            return Ok(None);
        }

        let length = buffer[2] as usize;
        let total = length + 4;
        if buffer.len() < total {
            return Ok(None);
        }

        let raw = buffer[..total].to_vec();
        let expected = raw[..total - 1].iter().fold(0u8, |acc, value| acc ^ value);
        if raw[total - 1] != expected {
            buffer.drain(..1);
            continue;
        }

        let frame = Frame {
            command: raw[1],
            payload: raw[3..total - 1].to_vec(),
            raw,
        };
        buffer.drain(..total);
        return Ok(Some(frame));
    }
}

fn extract_3a_frame(buffer: &mut Vec<u8>) -> Result<Option<ThreeAFrame>, String> {
    loop {
        let Some(start_index) = buffer
            .iter()
            .position(|value| *value == THREE_A_FRAME_START)
        else {
            buffer.clear();
            return Ok(None);
        };

        if start_index > 0 {
            buffer.drain(..start_index);
        }

        if buffer.len() < 8 {
            return Ok(None);
        }

        let length = buffer[3] as usize;
        let malformed_total = length + 7;
        let total = length + 8;
        if buffer.len() < malformed_total {
            return Ok(None);
        }

        if buffer[1] != THREE_A_DEVICE_ADDRESS {
            buffer.drain(..1);
            continue;
        }

        if buffer.len() >= total {
            let raw = buffer[..total].to_vec();

            if raw[total - 2] == THREE_A_END_1 && raw[total - 1] == THREE_A_END_2 {
                let checksum = u16::from_le_bytes([raw[total - 4], raw[total - 3]]);
                let expected = raw[1..total - 4]
                    .iter()
                    .fold(0u16, |acc, value| acc.wrapping_add(*value as u16));
                if checksum == expected {
                    let frame = ThreeAFrame {
                        command: raw[2],
                        payload: raw[4..total - 4].to_vec(),
                    };
                    buffer.drain(..total);
                    return Ok(Some(frame));
                }
            }
        }

        // 某些设备偶发丢失结尾的 0x0A，但长度、校验和 0x0D 仍然正确。
        // 在这种情况下先按“缺失尾字节”的 3A 帧容错解析，避免整包 C3 配置报废。
        let malformed_raw = buffer[..malformed_total].to_vec();
        if malformed_raw[malformed_total - 1] != THREE_A_END_1 {
            buffer.drain(..1);
            continue;
        }

        let checksum = u16::from_le_bytes([
            malformed_raw[malformed_total - 3],
            malformed_raw[malformed_total - 2],
        ]);
        let expected = malformed_raw[1..malformed_total - 3]
            .iter()
            .fold(0u16, |acc, value| acc.wrapping_add(*value as u16));
        if checksum != expected {
            buffer.drain(..1);
            continue;
        }

        let frame = ThreeAFrame {
            command: malformed_raw[2],
            payload: malformed_raw[4..malformed_total - 3].to_vec(),
        };
        buffer.drain(..malformed_total);
        return Ok(Some(frame));
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|value| format!("{value:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn printable_byte_label(value: u8) -> char {
    if value.is_ascii_graphic() || value == b' ' {
        value as char
    } else {
        '?'
    }
}

fn build_indexed_frame_trace(command: u8, frame: &[u8]) -> Vec<String> {
    if command != 0xA6 {
        return Vec::new();
    }

    let mut lines = vec![format!(
        "[serial][55][tx-index][cmd=0x{command:02X}] len={}",
        frame.len()
    )];
    for (index, value) in frame.iter().enumerate() {
        lines.push(format!(
            "[serial][55][tx-index][cmd=0x{command:02X}][{index}] 0x{value:02X} '{}'",
            printable_byte_label(*value)
        ));
    }

    lines
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

fn parse_upgrade_protocol(protocol_type: u8) -> UpgradeProtocol {
    match protocol_type {
        0x03 => UpgradeProtocol::GaoBiao,
        0x05 => UpgradeProtocol::KaiYang,
        0x09 => UpgradeProtocol::Iot,
        _ => UpgradeProtocol::Default,
    }
}

fn protocol_skips_app_erase(protocol: UpgradeProtocol) -> bool {
    matches!(
        protocol,
        UpgradeProtocol::Iot | UpgradeProtocol::KaiYang | UpgradeProtocol::GaoBiao
    )
}

fn protocol_skips_ui_erase(protocol: UpgradeProtocol) -> bool {
    matches!(
        protocol,
        UpgradeProtocol::KaiYang | UpgradeProtocol::GaoBiao
    )
}

fn uses_frame_number_ui_payload(protocol: UpgradeProtocol) -> bool {
    matches!(protocol, UpgradeProtocol::KaiYang)
}

fn protocol_prep_label(protocol: UpgradeProtocol) -> &'static str {
    match protocol {
        UpgradeProtocol::Iot => "IOT 固件信息校验",
        UpgradeProtocol::KaiYang => "开阳升级文件类型下发",
        UpgradeProtocol::GaoBiao => "高标升级文件类型下发",
        UpgradeProtocol::Default => "预处理",
    }
}

fn should_send_upgrade_param_after_protocol_prepare(
    protocol: UpgradeProtocol,
    kind: UpgradeKind,
) -> bool {
    matches!(kind, UpgradeKind::App | UpgradeKind::Ui)
        && matches!(
            protocol,
            UpgradeProtocol::KaiYang | UpgradeProtocol::GaoBiao
        )
}

fn should_init_upgrade_before_access(protocol: UpgradeProtocol, kind: UpgradeKind) -> bool {
    !(protocol == UpgradeProtocol::KaiYang && matches!(kind, UpgradeKind::App | UpgradeKind::Ui))
}

fn should_wait_for_passive_access_state(protocol: UpgradeProtocol, kind: UpgradeKind) -> bool {
    !(protocol == UpgradeProtocol::KaiYang && matches!(kind, UpgradeKind::App | UpgradeKind::Ui))
}

fn pad_bytes_to_word(bytes: &[u8], fill: u8) -> Vec<u8> {
    let mut normalized = bytes.to_vec();
    let padding = (4 - (normalized.len() % 4)) % 4;
    if padding > 0 {
        normalized.extend(std::iter::repeat(fill).take(padding));
    }
    normalized
}

fn compute_crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0u8;

    for value in bytes {
        crc ^= *value;
        for _ in 0..8 {
            if crc & 0x01 != 0 {
                crc = (crc >> 1) ^ 0x8C;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}

fn parse_hex_firmware_entries(data: &[u8]) -> Result<Vec<(u32, Vec<u8>)>, String> {
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

    entries.sort_by_key(|(address, _)| *address);
    Ok(entries)
}

fn build_hex_firmware_bytes(data: &[u8]) -> Result<Vec<u8>, String> {
    let entries = parse_hex_firmware_entries(data)?;

    let base_address = entries
        .first()
        .map(|(address, _)| *address)
        .ok_or_else(|| "HEX 文件没有有效数据记录".to_string())?;
    let end_address = entries
        .iter()
        .map(|(address, bytes)| address + bytes.len() as u32)
        .max()
        .ok_or_else(|| "HEX 文件没有有效数据记录".to_string())?;

    let mut image = vec![0xFF; (end_address - base_address) as usize];
    for (address, bytes) in entries {
        let offset = (address - base_address) as usize;
        image[offset..offset + bytes.len()].copy_from_slice(&bytes);
    }

    Ok(image)
}

fn build_iot_firmware_info_payload(
    file_name: &str,
    data: &[u8],
    total_frames: usize,
) -> Result<Vec<u8>, String> {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let firmware_bytes = match extension.as_str() {
        "hex" => build_hex_firmware_bytes(data)?,
        "bin" => data.to_vec(),
        _ => return Err(format!("IOT APP 仅支持 bin/hex 文件: {extension}")),
    };

    if firmware_bytes.is_empty() {
        return Err("固件文件为空".to_string());
    }

    let crc = compute_crc8(&pad_bytes_to_word(&firmware_bytes, 0xFF));
    let mut payload = vec![crc];
    payload.extend_from_slice(&(total_frames as u32).to_be_bytes());
    Ok(payload)
}

fn send_iot_firmware_info_command(
    manager: &SerialManager,
    file_name: &str,
    data: &[u8],
    total_frames: usize,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    let payload = build_iot_firmware_info_payload(file_name, data, total_frames)?;
    let response = manager.send_command(0x21, &payload, DEFAULT_TIMEOUT_MS)?;
    let response_payload = hex_to_bytes(&response.response_payload_hex)?;
    if response_payload.first().copied().unwrap_or_default() != 1 {
        return Err("IOT 固件信息校验失败".to_string());
    }

    let crc = payload.first().copied().unwrap_or_default();
    let total_frames = u32::from_be_bytes([payload[1], payload[2], payload[3], payload[4]]);
    logs.push(format!(
        "IOT 固件信息校验成功，CRC8=0x{crc:02X}，总帧数={total_frames}"
    ));
    Ok(())
}

fn build_kaiyang_file_type_payload(
    kind: UpgradeKind,
    total_frames: usize,
) -> Result<Vec<u8>, String> {
    let file_type = match kind {
        UpgradeKind::App => 3,
        UpgradeKind::Ui => 1,
        _ => return Err("开阳协议仅支持 APP/UI 文件类型下发".to_string()),
    };
    if total_frames > 0x00FF_FFFF {
        return Err("开阳协议总帧数超出 3 字节范围".to_string());
    }

    Ok(vec![
        file_type,
        ((total_frames >> 16) & 0xFF) as u8,
        ((total_frames >> 8) & 0xFF) as u8,
        (total_frames & 0xFF) as u8,
    ])
}

fn build_gaobiao_file_type_payload(kind: UpgradeKind) -> Result<Vec<u8>, String> {
    let file_type = match kind {
        UpgradeKind::App => 0,
        UpgradeKind::Ui => 1,
        _ => return Err("高标协议仅支持 APP/UI 文件类型下发".to_string()),
    };
    Ok(vec![file_type])
}

fn send_protocol_prepare_command(
    manager: &SerialManager,
    protocol: UpgradeProtocol,
    kind: UpgradeKind,
    file_name: &str,
    data: &[u8],
    total_frames: usize,
    logs: &mut Vec<String>,
) -> Result<(), String> {
    match protocol {
        UpgradeProtocol::Iot => {
            send_iot_firmware_info_command(manager, file_name, data, total_frames, logs)
        }
        UpgradeProtocol::KaiYang => {
            let payload = build_kaiyang_file_type_payload(kind, total_frames)?;
            let response = manager.send_command(0x22, &payload, DEFAULT_TIMEOUT_MS)?;
            let response_payload = hex_to_bytes(&response.response_payload_hex)?;
            if response_payload.first().copied().unwrap_or_default() != 1 {
                return Err("开阳升级文件类型下发失败".to_string());
            }
            logs.push(format!("开阳升级文件类型下发成功，总帧数={total_frames}"));
            Ok(())
        }
        UpgradeProtocol::GaoBiao => {
            let payload = build_gaobiao_file_type_payload(kind)?;
            let response = manager.send_command(0x23, &payload, DEFAULT_TIMEOUT_MS)?;
            let response_payload = hex_to_bytes(&response.response_payload_hex)?;
            if response_payload.first().copied().unwrap_or_default() != 1 {
                return Err("高标升级文件类型下发失败".to_string());
            }
            logs.push("高标升级文件类型下发成功".to_string());
            Ok(())
        }
        UpgradeProtocol::Default => Ok(()),
    }
}

fn parse_file_extension(file_name: &str) -> String {
    Path::new(file_name)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase()
}

fn resolve_app_write_mode(file_name: &str) -> Result<AppWriteMode, String> {
    match parse_file_extension(file_name).as_str() {
        "hex" => Ok(AppWriteMode::AddressedHex),
        "bin" => Ok(AppWriteMode::FrameNumberedBin),
        extension => Err(format!("APP 文件仅支持 .hex / .bin: {extension}")),
    }
}

fn build_app_frame_number_chunks(data: &[u8]) -> Result<Vec<DataChunk>, String> {
    build_sequential_chunks(data, 128, false)
}

fn build_app_addressed_hex_chunks(data: &[u8]) -> Result<Vec<DataChunk>, String> {
    build_hex_chunks(data, 128)
}

fn build_app_chunks(file_name: &str, data: &[u8]) -> Result<Vec<DataChunk>, String> {
    match resolve_app_write_mode(file_name)? {
        AppWriteMode::AddressedHex => build_app_addressed_hex_chunks(data),
        AppWriteMode::FrameNumberedBin => build_app_frame_number_chunks(data),
    }
}

fn build_app_binary_like_chunks(file_name: &str, data: &[u8]) -> Result<Vec<DataChunk>, String> {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let bytes = match extension.as_str() {
        "hex" => build_hex_firmware_bytes(data)?,
        "bin" => data.to_vec(),
        _ => return Err(format!("APP 文件格式不支持帧序号写入: {extension}")),
    };

    // 帧序号协议下 APP 负载为 4 字节帧号 + 最多 128 字节数据。
    // 最后一帧保持原始长度，不补 0xFF，兼容最新协议文档里
    // “开阳升级协议最后一帧升级数据的 FF 去掉”的要求。
    build_sequential_chunks(&bytes, 128, false)
}

fn build_ui_frame_number_chunks(file_name: &str, data: &[u8]) -> Result<Vec<DataChunk>, String> {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let bytes = match extension.as_str() {
        "txt" => parse_hex_text_bytes(data)?,
        "bin" => data.to_vec(),
        _ => return Err(format!("UI 文件格式不支持帧序号写入: {extension}")),
    };

    build_sequential_chunks(&bytes, 128, false)
}

fn build_realtime_chunks(
    file_name: &str,
    data: &[u8],
    kind: UpgradeKind,
    protocol: UpgradeProtocol,
) -> Result<Vec<DataChunk>, String> {
    match kind {
        UpgradeKind::App => build_app_chunks(file_name, data),
        UpgradeKind::Ui if uses_frame_number_ui_payload(protocol) => {
            build_ui_frame_number_chunks(file_name, data)
        }
        _ => build_chunks(file_name, data, kind),
    }
}

fn build_app_write_payload(chunk: &DataChunk, chunk_index: usize, file_name: &str) -> Vec<u8> {
    if resolve_app_write_mode(file_name).ok() == Some(AppWriteMode::FrameNumberedBin) {
        let mut payload = (chunk_index as u32).to_be_bytes().to_vec();
        payload.extend_from_slice(&chunk.data);
        return payload;
    }

    if let Some(address) = chunk.address {
        let mut payload = address.to_be_bytes().to_vec();
        payload.extend_from_slice(&chunk.data);
        return payload;
    }

    chunk.data.clone()
}

fn build_ui_write_payload(
    chunk: &DataChunk,
    chunk_index: usize,
    protocol: UpgradeProtocol,
) -> Vec<u8> {
    if uses_frame_number_ui_payload(protocol) {
        let mut payload = (chunk_index as u32).to_be_bytes().to_vec();
        payload.extend_from_slice(&chunk.data);
        return payload;
    }

    let address = chunk.address.unwrap_or_default();
    let mut payload = address.to_be_bytes().to_vec();
    payload.extend_from_slice(&chunk.data);
    payload
}

fn build_chunks(file_name: &str, data: &[u8], kind: UpgradeKind) -> Result<Vec<DataChunk>, String> {
    let extension = parse_file_extension(file_name);

    match (kind, extension.as_str()) {
        (UpgradeKind::Ui, "txt") => {
            build_sequential_chunks(&parse_hex_text_bytes(data)?, 128, true)
        }
        (_, "txt") => build_sequential_chunks(&parse_hex_text_bytes(data)?, 128, true),
        (UpgradeKind::Boot, _) => build_sequential_chunks(data, 128, true),
        (UpgradeKind::Ui, _) => build_sequential_chunks(data, 128, true),
        (UpgradeKind::App, _) => build_app_binary_like_chunks(file_name, data),
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
    let entries = parse_hex_firmware_entries(data)?;

    let mut merged_entries: Vec<(u32, Vec<u8>)> = Vec::new();
    for (address, bytes) in entries {
        if let Some((base_address, merged_bytes)) = merged_entries.last_mut() {
            let next_address = *base_address + merged_bytes.len() as u32;
            if address == next_address {
                merged_bytes.extend_from_slice(&bytes);
                continue;
            }
        }

        merged_entries.push((address, bytes));
    }

    let mut chunks = Vec::new();
    for (base_address, bytes) in merged_entries {
        for (offset, chunk) in bytes.chunks(chunk_size).enumerate() {
            chunks.push(DataChunk {
                address: Some(base_address + (offset * chunk_size) as u32),
                data: chunk.to_vec(),
            });
        }
    }
    Ok(chunks)
}
