# Experiment Module Documentation

## Overview

The Experiment module is the core component that coordinates device control, data acquisition, and visualization during optical measurements. It handles the communication between the CLD1015 Laser Diode Controller and the MPM210H Power Meter, manages the experiment workflow, and ensures data is properly captured and stored.

## Key Components

### 1. Data Structures
- `MeasurementRecord`: Represents a single data point in a sweep measurement, containing:
  - `timestamp`: UTC ISO timestamp for the measurement
  - `current_ma`: Laser input current in milliamperes
  - `power_dbm`: Measured optical power from the MPM-210H
  - `module`: Port/module ID on the MPM-210H

### 2. Primary Functions

#### `run_current_sweep_with_live_plot`
```rust
pub fn run_current_sweep_with_live_plot(
    cld: &mut CLD1015,
    mpm: &mut MPM210H,
    module: u8,
    start_ma: f64,
    stop_ma: f64,
    step_ma: f64,
    stabilization_delay_ms: u64,
    window: Window,
) -> Result<PathBuf, String>
```

**Purpose:**  
Performs a current sweep by incrementally changing the laser diode current and measuring the corresponding optical power at each step.

**Algorithm:**
1. Validate sweep parameters (step > 0, start < stop)
2. Verify TEC is active for safety
3. Configure devices:
   - Turn laser off initially
   - Set MPM-210H wavelength to 980nm (CLD1015 wavelength)
   - Turn laser on
4. For each current value from start to stop:
   - Set the laser current
   - Wait for stabilization (user-configurable delay)
   - Measure optical power
   - Emit event to update the UI in real-time
   - Store the measurement record
5. Turn laser off when sweep completes
6. Save data to CSV file
7. Return path to the saved file

**Safety Features:**
- Ensures TEC is on before starting, preventing laser damage
- Disables laser output at start and end of sweep
- Validates input parameters before proceeding

#### `save_measurements_to_csv`
```rust
fn save_measurements_to_csv(data: &[MeasurementRecord]) -> io::Result<PathBuf>
```

**Purpose:**  
Saves measurement records to a timestamped CSV file for later analysis.

**Implementation Details:**
1. Creates a filename with current timestamp
2. Ensures logs directory exists, creating it if necessary
3. Writes records using the CSV writer
4. Returns the path to the created file

## Error Handling

The experiment module uses a detailed error handling approach:
- Parameter validation with descriptive error messages
- Hardware safety checks before proceeding with measurement
- Propagation of device-specific errors with context
- Proper error logging for debugging

## Data Flow

1. **Configuration**: User input from UI → Tauri command handler → Experiment module
2. **Execution**: Experiment module → Device drivers → Hardware
3. **Data Acquisition**: Hardware → Device drivers → Experiment module → UI events
4. **Storage**: Experiment module → CSV file
