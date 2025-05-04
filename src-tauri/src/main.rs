// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]
mod devices;
mod experiment;

use devices::{CLD1015, MPM210H};
use tauri::State;
use std::sync::Mutex;
use tracing_subscriber::fmt;
use tracing_appender::rolling;
use tracing::info;
use tracing::error;

struct AppState {
    cld1015: Mutex<CLD1015>,
    mpm210h: Mutex<MPM210H>,
}

/*#[tauri::command]
fn enable_tec(state: State<AppState>) -> Result<(), String> {
    state.cld1015.lock().unwrap().enable_tec().map_err(|e| e.to_string())
}*/

#[tauri::command]
fn connect_cld1015(state: State<AppState>) -> Result<String, String> {
    let mut cld = state.cld1015.lock().unwrap();
    let id = cld.connect().map_err(|e| e.to_string())?;

    state.cld1015.lock().unwrap().enable_tec().map_err(|e| e.to_string())?;

    Ok(id)
}

#[tauri::command]
fn connect_mpm210h(state: State<AppState>) -> Result<String, String> {
    let mut mpm = state.mpm210h.lock().unwrap();
    let id = mpm.connect().map_err(|e| e.to_string())?;

    // Enforce wavelength to 980 nm for CLD1015
    mpm.set_wavelength(980).map_err(|e| e.to_string())?;

    Ok(id)
}

#[tauri::command]
fn is_cld1015_connected(state: State<AppState>) -> bool {
    state.cld1015.lock().unwrap().is_connected()
}

#[tauri::command]
fn is_mpm210h_connected(state: State<AppState>) -> bool {
    state.mpm210h.lock().unwrap().is_connected()
}

#[tauri::command]
fn get_tec_state(state: State<AppState>) -> Result<bool, String> {
    state.cld1015.lock().unwrap().get_tec_state().map_err(|e| e.to_string())
}

/*#[tauri::command]
fn set_cld1015_current(state: State<AppState>, current_ma: f64) -> Result<(), String> {
    // Convert mA to A (since the device expects amps)
    let current_amps = current_ma / 1000.0;
    state.cld1015.lock().unwrap().set_current(current_amps).map_err(|e| e.to_string())
}*/

/*#[tauri::command]
fn get_cld1015_current(state: State<AppState>) -> Result<f64, String> {
    // Return current in mA
    state.cld1015.lock().unwrap().get_current()
        .map(|a| a * 1000.0) // Convert from A to mA
        .map_err(|e| e.to_string())
}*/

/*#[tauri::command]
fn set_cld1015_laser_output(state: State<AppState>, enabled: bool) -> Result<(), String> {
    let mut cld = state.cld1015.lock().unwrap();
    if enabled && !cld.get_tec_state().map_err(|e| e.to_string())? {
        return Err("Cannot enable laser: TEC is OFF".into());
    }
    cld.set_laser_output(enabled).map_err(|e| e.to_string())
}*/

#[tauri::command]
fn get_cld1015_error(state: State<AppState>) -> Result<String, String> {
    state.cld1015.lock().unwrap().get_error().map_err(|e| {
        error!("Failed to read CLD1015 error queue: {}", e);
        e.to_string()
    })
}

#[tauri::command]
fn clear_cld1015_error_queue(state: State<AppState>) -> Result<Vec<String>, String> {
    state.cld1015.lock().unwrap()
        .clear_error_queue()
        .map_err(|e| {
            error!("Failed to clear CLD1015 error queue: {}", e);
            e.to_string()
        })
}

#[tauri::command]
fn get_mpm210h_modules(state: State<AppState>) -> Result<String, String> {
    state.mpm210h.lock().unwrap().get_recognized_modules().map_err(|e| e.to_string())
}

/*#[tauri::command]
fn read_mpm210h_power(state: State<AppState>, module: u8) -> Result<String, String> {
    state.mpm210h.lock().unwrap().read_power(module).map_err(|e| e.to_string())
}*/

#[tauri::command]
fn get_mpm210h_wavelength(state: State<AppState>) -> Result<String, String> {
    state.mpm210h.lock().unwrap().get_wavelength().map_err(|e| e.to_string())
}

/*#[tauri::command]
fn set_mpm210h_wavelength(state: State<AppState>, wavelength: u32) -> Result<(), String> {
    state.mpm210h.lock().unwrap().set_wavelength(wavelength).map_err(|e| e.to_string())
}*/

#[tauri::command]
fn get_mpm210h_error(state: State<AppState>) -> Result<String, String> {
    state.mpm210h.lock().unwrap().get_error().map_err(|e| {
        error!("Failed to read MPM210H error queue: {}", e);
        e.to_string()
    })
}

#[tauri::command]
fn clear_mpm210h_error_queue(state: State<AppState>) -> Result<Vec<String>, String> {
    state.mpm210h.lock().unwrap().clear_error_queue().map_err(|e| {
        error!("Failed to clear MPM210H error queue: {}", e);
        e.to_string()
    })
}

#[tauri::command]
fn run_current_sweep(
    window: tauri::Window,
    state: State<AppState>,
    module: u8,
    start_ma: f64,
    stop_ma: f64,
    step_ma: f64,
) -> Result<String, String> {
    let path = experiment::run_current_sweep_with_live_plot(
        &mut state.cld1015.lock().unwrap(),
        &mut state.mpm210h.lock().unwrap(),
        module,
        start_ma,
        stop_ma,
        step_ma,
        20, // ms stabilization enough for cld1015 and mpm210h
        window,
    )?;

    Ok(path.to_string_lossy().into())
}

fn main() {
    // Rotate daily into "logs/app.log.YYYY-MM-DD"
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    // Initialize global logger
    fmt()
        .with_writer(non_blocking)
        .with_ansi(false) // Disable ANSI if viewing in plain file
        .with_level(true)
        .init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            cld1015: Mutex::new(CLD1015::new("USB0::4883::32847::M01053290::0::INSTR")),
            mpm210h: Mutex::new(MPM210H::new("192.168.1.161", 5000)),
        })
        .invoke_handler(tauri::generate_handler![
            //enable_tec,
            connect_cld1015,
            connect_mpm210h,
            is_cld1015_connected,
            is_mpm210h_connected,
            get_tec_state,
            // set_cld1015_current,
            // get_cld1015_current,
            //set_cld1015_laser_output,
            get_mpm210h_modules,
            //read_mpm210h_power,
            get_mpm210h_wavelength,
            //set_mpm210h_wavelength,
            get_cld1015_error,
            clear_cld1015_error_queue,
            get_mpm210h_error,
            clear_mpm210h_error_queue,
            run_current_sweep,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
