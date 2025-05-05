pub mod data;

use crate::devices::{CLD1015, MPM210H};
use data::MeasurementRecord;
use chrono::Utc;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use csv::Writer;
use tracing::{info, error};
use tauri::Window;
use tauri::Emitter;

/// Run a current sweep and collect measurements
pub fn run_current_sweep_with_live_plot(
    cld: &mut CLD1015,
    mpm: &mut MPM210H,
    module: u8,
    start_ma: f64,
    stop_ma: f64,
    step_ma: f64,
    stabilization_delay_ms: u64,
    window: Window,
) -> Result<PathBuf, String> {
    // Validate parameters
    if step_ma <= 0.0 || start_ma > stop_ma {
        return Err("Invalid sweep parameters".into());
    }

    // Safety: Ensure TEC is active
    let tec_on = cld.get_tec_state().map_err(|e| e.to_string())?;
    if !tec_on {
        return Err("TEC must be ON before starting the experiment".into());
    }

    // Perform zeroing before starting the sweep to ensure accurate measurements
    info!("Performing zeroing operation before sweep to remove electrical offsets");
    match mpm.perform_zeroing() {
        Ok(_) => info!("Zeroing command sent successfully"),
        Err(e) => {
            error!("Failed to perform zeroing: {}", e);
            return Err(format!("Failed to perform zeroing: {}", e));
        }
    }

    // Give time for the zeroing operation to complete (3 seconds as per documentation)
    std::thread::sleep(std::time::Duration::from_secs(3));
    info!("Zeroing completed, proceeding with sweep");

    //  laser off at the beginning
    cld.set_laser_output(false).ok();
    // ensure mpm210h is at the cld1015 wavelength
    mpm.set_wavelength(980).map_err(|e| e.to_string())?;

    cld.set_laser_output(true).map_err(|e| e.to_string())?; // turn laser on
    info!("Starting current sweep: {} mA to {} mA, step {} mA, module {}", start_ma, stop_ma, step_ma, module);

    let mut records = Vec::new();
    let mut current_ma = start_ma;

    while current_ma <= stop_ma {
        cld.set_current(current_ma / 1000.0).map_err(|e| e.to_string())?; // convert to A
        std::thread::sleep(std::time::Duration::from_millis(stabilization_delay_ms));

        let power = mpm.read_power(module).map_err(|e| e.to_string())?;
        let now = Utc::now().to_rfc3339();

        let record = MeasurementRecord {
            timestamp: now.clone(),
            current_ma,
            power_dbm: power.clone(),
            module,
        };

        window.emit("sweep-point", &record).unwrap_or_else(|e| {
            error!("Failed to emit sweep-point: {}", e);
        });

        records.push(record);
        current_ma += step_ma;
    }
    cld.set_laser_output(false).ok(); // turn laser off after sweep

    let path = save_measurements_to_csv(&records)
        .map_err(|e| format!("Failed to save CSV: {}", e))?;

    info!("Sweep completed. Data saved to: {:?}", path);

    Ok(path)
}

/// Save the measurement records to a timestamped CSV file
fn save_measurements_to_csv(data: &[MeasurementRecord]) -> io::Result<PathBuf> {
    let timestamp = chrono::Local::now()
        .format("experiment_data_%Y-%m-%d_%H-%M-%S.csv")
        .to_string();

    let mut path = std::env::current_dir()?;
    path.push("logs");
    std::fs::create_dir_all(&path)?;
    path.push(timestamp);

    let file = File::create(&path)?;
    let mut writer = Writer::from_writer(file);
    for record in data {
        writer.serialize(record)?;
    }
    writer.flush()?;

    Ok(path)
}
