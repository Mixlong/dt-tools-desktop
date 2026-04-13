use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    utils::config::Color,
    Emitter, Manager, WebviewWindow, WindowEvent,
};

mod unimaster;

fn ensure_serial_idle(serial_manager: &unimaster::SerialManager) -> Result<(), String> {
    if serial_manager.is_upgrade_active() {
        return Err("升级进行中，当前操作已被阻止，请等待升级完成后重试".to_string());
    }

    Ok(())
}

#[tauri::command]
fn restart_app(app: tauri::AppHandle) {
    app.restart();
}

#[tauri::command]
fn save_text_file(path: String, contents: String) -> Result<(), String> {
    if let Some(parent) = Path::new(&path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|error| format!("创建目录失败: {error}"))?;
        }
    }

    fs::write(&path, contents).map_err(|error| format!("保存文件失败: {error}"))
}

#[tauri::command]
fn frontend_log(level: String, message: String) {
    match level.as_str() {
        "error" => eprintln!("[frontend][error] {message}"),
        "warn" => eprintln!("[frontend][warn] {message}"),
        _ => eprintln!("[frontend][info] {message}"),
    }
}

#[tauri::command]
fn list_serial_ports() -> Result<Vec<unimaster::SerialPortInfo>, String> {
    unimaster::list_serial_ports()
}

#[tauri::command]
fn serial_status(
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::ConnectionStatus, String> {
    serial_manager.status()
}

#[tauri::command]
async fn connect_serial(
    port_name: String,
    baud_rate: u32,
    app: tauri::AppHandle,
) -> Result<unimaster::ConnectionStatus, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let started_at = std::time::Instant::now();
        eprintln!(
            "[perf][serial-connect][start] port={} baudRate={}",
            port_name, baud_rate
        );

        let serial_manager = app.state::<unimaster::SerialManager>();
        ensure_serial_idle(&serial_manager)?;
        let result = serial_manager.connect(port_name.clone(), baud_rate);

        match &result {
            Ok(status) => eprintln!(
                "[perf][serial-connect][ok] totalMs={} connected={} port={}",
                started_at.elapsed().as_millis(),
                status.connected,
                status.port_name.clone().unwrap_or_default()
            ),
            Err(error) => eprintln!(
                "[perf][serial-connect][fail] totalMs={} error={}",
                started_at.elapsed().as_millis(),
                error
            ),
        }

        result
    })
    .await
    .map_err(|error| format!("执行串口连接任务失败: {error}"))?
}

#[tauri::command]
async fn disconnect_serial(
    app: tauri::AppHandle,
) -> Result<unimaster::ConnectionStatus, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let serial_manager = app.state::<unimaster::SerialManager>();
        ensure_serial_idle(&serial_manager)?;
        serial_manager.disconnect()
    })
    .await
    .map_err(|error| format!("执行串口断开任务失败: {error}"))?
}

#[tauri::command]
fn send_raw_command(
    request: unimaster::RawCommandRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::FrameExchange, String> {
    ensure_serial_idle(&serial_manager)?;
    serial_manager.send_command(
        request.command,
        &request.payload,
        request.timeout_ms.unwrap_or(1500),
    )
}

#[tauri::command]
fn read_version_snapshot(
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::VersionSnapshot, String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::read_version_snapshot(&serial_manager)
}

#[tauri::command]
fn write_version_info(
    request: unimaster::WriteVersionInfoRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::write_version_info(&serial_manager, request)
}

#[tauri::command]
fn read_flags(
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<Vec<unimaster::FlagValue>, String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::read_flags(&serial_manager)
}

#[tauri::command]
fn write_flag(
    request: unimaster::WriteFlagRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::write_flag(&serial_manager, request)
}

#[tauri::command]
fn switch_language(
    language: u8,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::switch_language(&serial_manager, language)
}

#[tauri::command]
async fn set_meter_config_transport(
    request: unimaster::MeterTransportRequest,
    app: tauri::AppHandle,
) -> Result<unimaster::SimpleResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let serial_manager = app.state::<unimaster::SerialManager>();
        ensure_serial_idle(&serial_manager)?;
        unimaster::set_meter_config_transport(&serial_manager, request)
    })
    .await
    .map_err(|error| format!("执行配置链路初始化任务失败: {error}"))?
}

#[tauri::command]
async fn read_meter_config(
    request: unimaster::MeterConfigReadRequest,
    app: tauri::AppHandle,
) -> Result<unimaster::MeterConfigReadResponse, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let serial_manager = app.state::<unimaster::SerialManager>();
        ensure_serial_idle(&serial_manager)?;
        unimaster::read_meter_config(&serial_manager, request)
    })
    .await
    .map_err(|error| format!("执行读取配置任务失败: {error}"))?
}

#[tauri::command]
fn send_meter_config_heartbeat(
    request: unimaster::MeterHeartbeatRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<(), String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::send_meter_config_heartbeat(&serial_manager, request)
}

#[tauri::command]
async fn write_meter_config(
    bytes: Vec<u8>,
    app: tauri::AppHandle,
) -> Result<unimaster::SimpleResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let serial_manager = app.state::<unimaster::SerialManager>();
        ensure_serial_idle(&serial_manager)?;
        unimaster::write_meter_config(&serial_manager, bytes)
    })
    .await
    .map_err(|error| format!("执行写入配置任务失败: {error}"))?
}

#[tauri::command]
fn set_realtime_screen(
    screen: u8,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::set_realtime_screen(&serial_manager, screen)
}

#[tauri::command]
fn read_access_state(
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::read_access_state(&serial_manager)
}

#[tauri::command]
fn init_realtime_upgrade(
    request: unimaster::RealtimeInitRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    ensure_serial_idle(&serial_manager)?;
    unimaster::init_realtime_upgrade(&serial_manager, request)
}

#[tauri::command]
async fn perform_realtime_upgrade(
    request: unimaster::RealtimeUpgradeRequest,
    app: tauri::AppHandle,
) -> Result<unimaster::UpgradeSummary, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let app_handle = app.clone();
        let serial_manager = app.state::<unimaster::SerialManager>();
        serial_manager.begin_upgrade()?;
        let result = unimaster::perform_realtime_upgrade(&serial_manager, request, move |event| {
            let _ = app_handle.emit("upgrade-progress", event);
        });
        serial_manager.end_upgrade();
        result
    })
    .await
    .map_err(|error| format!("执行实时升级任务失败: {error}"))?
}

#[tauri::command]
async fn prepare_offline_upgrade(
    request: unimaster::OfflinePrepareRequest,
    app: tauri::AppHandle,
) -> Result<unimaster::UpgradeSummary, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let serial_manager = app.state::<unimaster::SerialManager>();
        ensure_serial_idle(&serial_manager)?;
        unimaster::prepare_offline_upgrade(&serial_manager, request)
    })
    .await
    .map_err(|error| format!("执行离线烧录任务失败: {error}"))?
}

#[tauri::command]
async fn load_program_burning_bundle(
    code_or_sn: String,
) -> Result<unimaster::ProgramBurningBundle, String> {
    unimaster::load_program_burning_bundle(code_or_sn).await
}

#[derive(Default)]
struct AppState {
    is_quitting: AtomicBool,
}

fn get_tray_icon() -> Option<Image<'static>> {
    #[cfg(target_os = "macos")]
    {
        return Image::from_bytes(include_bytes!("../icons/tray-mac-18.png")).ok();
    }

    #[cfg(target_os = "windows")]
    {
        return Image::from_bytes(include_bytes!("../icons/tray-win-16.png")).ok();
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        None
    }
}

fn configure_main_window_appearance(window: &WebviewWindow) {
    let _ = window.set_background_color(Some(Color(0, 0, 0, 0)));
    let _ = window.set_shadow(false);

    #[cfg(target_os = "macos")]
    {
        let _ = window.with_webview(|webview| unsafe {
            let ns_window: &objc2_app_kit::NSWindow = &*webview.ns_window().cast();
            let wk_webview: &objc2_web_kit::WKWebView = &*webview.inner().cast();
            let clear = objc2_app_kit::NSColor::clearColor();

            ns_window.setBackgroundColor(Some(&clear));
            ns_window.setOpaque(false);
            ns_window.setHasShadow(false);
            wk_webview.setUnderPageBackgroundColor(Some(&clear));
        });
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(unimaster::SerialManager::default())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                let is_quitting = window
                    .app_handle()
                    .state::<AppState>()
                    .is_quitting
                    .load(Ordering::Relaxed);
                if !is_quitting {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            restart_app,
            save_text_file,
            frontend_log,
            list_serial_ports,
            serial_status,
            connect_serial,
            disconnect_serial,
            send_raw_command,
            read_version_snapshot,
            write_version_info,
            read_flags,
            write_flag,
            switch_language,
            set_meter_config_transport,
            read_meter_config,
            send_meter_config_heartbeat,
            write_meter_config,
            set_realtime_screen,
            read_access_state,
            init_realtime_upgrade,
            perform_realtime_upgrade,
            prepare_offline_upgrade,
            load_program_burning_bundle
        ])
        .setup(|app| {
            app.manage(AppState::default());

            if let Some(window) = app.get_webview_window("main") {
                configure_main_window_appearance(&window);
            }

            // USB 串口热插拔监听后台线程
            {
                let app_handle = app.handle().clone();
                thread::spawn(move || {
                    let mut last_ports = unimaster::list_serial_port_names().unwrap_or_default();
                    last_ports.sort();

                    loop {
                        thread::sleep(Duration::from_millis(1500));

                        let mut current_ports = unimaster::list_serial_port_names().unwrap_or_default();
                        current_ports.sort();

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

            let show_item = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &quit_item])?;
            let app_handle = app.handle().clone();
            let mut tray_builder = TrayIconBuilder::with_id("main-tray")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.state::<AppState>()
                            .is_quitting
                            .store(true, Ordering::Relaxed);
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(move |_tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                });

            if let Some(icon) = get_tray_icon().or_else(|| app.default_window_icon().cloned()) {
                tray_builder = tray_builder.icon(icon);
            }

            tray_builder.build(app)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Reopen { .. } = event {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        });
}
