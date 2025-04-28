// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod devices;

use devices::{CLD1015, MPM210H};
use tauri::State;
use std::sync::Mutex;

struct AppState {
    cld1015: Mutex<CLD1015>,
    mpm210h: Mutex<MPM210H>,
}

#[tauri::command]
fn connect_cld1015(state: State<AppState>) -> Result<String, String> {
    let mut cld = state.cld1015.lock().unwrap();
    cld.connect().map_err(|e| e.to_string())
}

#[tauri::command]
fn connect_mpm210h(state: State<AppState>) -> Result<String, String> {
    let mut mpm = state.mpm210h.lock().unwrap();
    mpm.connect().map_err(|e| e.to_string())
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
fn get_cld1015_tec_state(state: State<AppState>) -> Result<bool, String> {
    state.cld1015.lock().unwrap().get_tec_state().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_cld1015_current(state: State<AppState>, current_ma: f64) -> Result<(), String> {
    // Convert mA to A (since the device expects amps)
    let current_amps = current_ma / 1000.0;
    state.cld1015.lock().unwrap().set_current(current_amps).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_cld1015_current(state: State<AppState>) -> Result<f64, String> {
    // Return current in mA
    state.cld1015.lock().unwrap().get_current()
        .map(|a| a * 1000.0) // Convert from A to mA
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn set_cld1015_laser_output(state: State<AppState>, enabled: bool) -> Result<(), String> {
    state.cld1015.lock().unwrap().set_laser_output(enabled).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_mpm210h_modules(state: State<AppState>) -> Result<String, String> {
    state.mpm210h.lock().unwrap().get_recognized_modules().map_err(|e| e.to_string())
}

#[tauri::command]
fn read_mpm210h_power(state: State<AppState>, module: u8) -> Result<String, String> {
    state.mpm210h.lock().unwrap().read_power(module).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_mpm210h_wavelength(state: State<AppState>) -> Result<String, String> {
    state.mpm210h.lock().unwrap().get_wavelength().map_err(|e| e.to_string())
}

#[tauri::command]
fn set_mpm210h_wavelength(state: State<AppState>, wavelength: u32) -> Result<(), String> {
    state.mpm210h.lock().unwrap().set_wavelength(wavelength).map_err(|e| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            cld1015: Mutex::new(CLD1015::new("USB0::4883::32847::M01053290::0::INSTR")),
            mpm210h: Mutex::new(MPM210H::new("192.168.1.161", 5000)),
        })
        .invoke_handler(tauri::generate_handler![
            connect_cld1015,
            connect_mpm210h,
            is_cld1015_connected,
            is_mpm210h_connected,
            get_cld1015_tec_state,
            set_cld1015_current,
            get_cld1015_current,
            set_cld1015_laser_output,
            get_mpm210h_modules,
            read_mpm210h_power,
            get_mpm210h_wavelength,
            set_mpm210h_wavelength,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
