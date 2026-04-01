use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};

use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

mod unimaster;

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
fn connect_serial(
    port_name: String,
    baud_rate: u32,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::ConnectionStatus, String> {
    serial_manager.connect(port_name, baud_rate)
}

#[tauri::command]
fn disconnect_serial(
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::ConnectionStatus, String> {
    serial_manager.disconnect()
}

#[tauri::command]
fn send_raw_command(
    request: unimaster::RawCommandRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::FrameExchange, String> {
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
    unimaster::read_version_snapshot(&serial_manager)
}

#[tauri::command]
fn write_version_info(
    request: unimaster::WriteVersionInfoRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    unimaster::write_version_info(&serial_manager, request)
}

#[tauri::command]
fn read_flags(
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<Vec<unimaster::FlagValue>, String> {
    unimaster::read_flags(&serial_manager)
}

#[tauri::command]
fn write_flag(
    request: unimaster::WriteFlagRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    unimaster::write_flag(&serial_manager, request)
}

#[tauri::command]
fn switch_language(
    language: u8,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    unimaster::switch_language(&serial_manager, language)
}

#[tauri::command]
fn set_meter_config_transport(
    request: unimaster::MeterTransportRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    unimaster::set_meter_config_transport(&serial_manager, request)
}

#[tauri::command]
fn read_meter_config(
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::MeterConfigReadResponse, String> {
    unimaster::read_meter_config(&serial_manager)
}

#[tauri::command]
fn write_meter_config(
    bytes: Vec<u8>,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    unimaster::write_meter_config(&serial_manager, bytes)
}

#[tauri::command]
fn set_realtime_screen(
    screen: u8,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    unimaster::set_realtime_screen(&serial_manager, screen)
}

#[tauri::command]
fn read_access_state(
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    unimaster::read_access_state(&serial_manager)
}

#[tauri::command]
fn init_realtime_upgrade(
    request: unimaster::RealtimeInitRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::SimpleResult, String> {
    unimaster::init_realtime_upgrade(&serial_manager, request)
}

#[tauri::command]
fn perform_realtime_upgrade(
    request: unimaster::RealtimeUpgradeRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::UpgradeSummary, String> {
    unimaster::perform_realtime_upgrade(&serial_manager, request)
}

#[tauri::command]
fn prepare_offline_upgrade(
    request: unimaster::OfflinePrepareRequest,
    serial_manager: tauri::State<unimaster::SerialManager>,
) -> Result<unimaster::UpgradeSummary, String> {
    unimaster::prepare_offline_upgrade(&serial_manager, request)
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
            write_meter_config,
            set_realtime_screen,
            read_access_state,
            init_realtime_upgrade,
            perform_realtime_upgrade,
            prepare_offline_upgrade
        ])
        .setup(|app| {
            app.manage(AppState::default());

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
