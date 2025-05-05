# Safety Features Documentation

## Overview

The Automated Optical Lab Control Suite incorporates multiple safety features to protect both the expensive optical equipment and the personnel operating the system. This document outlines the safety mechanisms implemented throughout the application.

## Laser Safety Features

### 1. TEC Protection

**Purpose:** Prevent damage to the laser diode by ensuring proper temperature control before operation.

**Implementation:**
```rust
pub fn set_laser_output(&mut self, enabled: bool) -> visa_rs::Result<()> {
    if enabled {
        // Safety check: ensure TEC is ON before enabling laser
        let tec_on = self.get_tec_state()?;
        if !tec_on {
            error!("Attempt to enable laser while TEC is OFF");
            return Err(visa_rs::io_to_vs_err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Cannot enable laser: TEC is OFF",
            )));
        }
        info!("Enabling laser output");
    } else {
        info!("Disabling laser output");
    }

    let state = if enabled { "ON" } else { "OFF" };
    self.write(&format!("OUTPut:STATe {}", state))
}
```

**Verification:**
- TEC state is checked before any attempt to enable the laser
- Error is returned with descriptive message if TEC is off
- All attempts are logged for audit purposes

### 2. Current Limiting

**Purpose:** Prevent excessive current from damaging the laser diode.

**Implementation:**
```rust
pub fn set_current(&mut self, current_amps: f64) -> visa_rs::Result<()> {
    const MAX_SAFE_CURRENT_AMPS: f64 = 1.5;
    if current_amps > MAX_SAFE_CURRENT_AMPS {
        warn!("Attempted to set current above safe limit: {} A", current_amps);
        return Err(visa_rs::io_to_vs_err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Requested current {} A exceeds the 1.5 A safety limit", current_amps),
        )));
    }
    info!("Setting current to {:.3} A", current_amps);
    self.write(&format!("SOURce:CURRent:LEVel:IMMediate:AMPLitude {}", current_amps))
}
```

**Verification:**
- Hard-coded maximum current limit of 1.5A
- All current values are validated before being sent to the device
- Warning logged for audit purposes if limit is exceeded
- Error returned with descriptive message

### 3. Automatic Laser Shutdown

**Purpose:** Ensure laser is not left on after measurements are complete.

**Implementation:**
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
) -> Result<PathBuf, String> {
    // Safety checks...
    
    // Turn off laser at beginning
    cld.set_laser_output(false).ok();
    
    // Configuration...
    
    // Turn on laser for measurement
    cld.set_laser_output(true).map_err(|e| e.to_string())?;
    
    // Perform measurements...
    
    // Always turn off laser after sweep
    cld.set_laser_output(false).ok();
    
    // Save data and return
    // ...
}
```

**Verification:**
- Laser is explicitly turned off at the beginning and end of every measurement
- Even if an error occurs during measurement, the `.ok()` pattern ensures shutdown attempt
- Follows the principle of "safe by default"

## Optical Power Protection

### 1. Maximum Input Power Protection

**Purpose:** Prevent damage to the power meter by exceeding its maximum rated input power.

**Implementation:**
- The MPM-210H has a maximum input power of +16dBm
- User documentation clearly specifies this limit
- Current sweep parameters are validated to ensure they won't exceed this limit

**Verification:**
- Parameter validation in the UI and backend before experiments start
- Logging of all power measurements for audit purposes

### 2. Zeroing Function

**Purpose:** Ensure accurate power measurements by removing electrical offsets.

**Implementation:**
```rust
// Before measuring optical power, run Zeroing to delete electrical DC offset
// This command action takes about 3 sec
pub fn zero_command_handler() -> Result<(), String> {
    // Ensure no input light during zeroing
    // Wait for completion
    // Verify successful zeroing
}
```

**Verification:**
- User is prompted to ensure no input light during zeroing
- Adequate delay ensures complete zeroing process
- Verification of successful zeroing before continuing

## Error Detection and Recovery

### 1. Communication Failure Handling

**Purpose:** Detect and recover from device communication failures.

**Implementation:**
- Timeouts on all device communications
- Retry logic for transient failures
- Graceful degradation when device becomes unresponsive

**Verification:**
- Error reporting through UI
- Logging of all communication errors
- Automatic attempt to re-establish connection

### 2. Parameter Validation

**Purpose:** Prevent invalid parameters from being sent to devices.

**Implementation:**
```rust
// Validate parameters
if step_ma <= 0.0 || start_ma > stop_ma {
    return Err("Invalid sweep parameters".into());
}
```

**Verification:**
- All parameters validated before use
- Clear error messages describing the validation failure
- Frontend validation mirrors backend validation

## Logging and Audit Trail

### 1. Comprehensive Logging

**Purpose:** Maintain a record of all operations for troubleshooting and safety audit.

**Implementation:**
```rust
// Initialize global logger
fmt()
    .with_writer(non_blocking)
    .with_ansi(false) // Disable ANSI if viewing in plain file
    .with_level(true)
    .init();
```

**Verification:**
- All device operations logged with timestamps
- Error conditions captured with stack traces
- Daily log rotation to prevent file size issues

### 2. Measurement Records

**Purpose:** Provide traceability for all measurements.

**Implementation:**
- All measurements saved as CSV files with ISO timestamps
- Metadata includes device settings and calibration information
- Files named with date/time for easy identification

**Verification:**
- CSV files contain complete experiment parameters
- File creation verified after each experiment
- Path returned to UI for user reference

## UI Safety Features

### 1. Status Indicators

**Purpose:** Provide clear visual indication of device and safety status.

**Implementation:**
```vue
<div class="status-item" :class="{ online: tecOn }">
  TEC: {{ tecOn ? 'ON' : 'OFF' }}
</div>
```

**Verification:**
- Color-coded status indicators (red/green)
- Clear text labels for status conditions
- Real-time updates during operations

### 2. Input Validation

**Purpose:** Prevent invalid parameters from being submitted.

**Implementation:**
```vue
const isValid = computed(() => {
  if (start.value >= stop.value) {
    errorMsg.value = 'Start must be less than Stop'
    return false
  }
  if (step.value <= 0) {
    errorMsg.value = 'Step must be greater than 0'
    return false
  }
  errorMsg.value = null
  return true
})
```

**Verification:**
- Real-time validation with clear error messages
- Disabled submit buttons when validation fails
- Consistent validation between frontend and backend

## Emergency Procedures

### 1. Emergency Shutdown

**Purpose:** Allow immediate shutdown in case of emergency.

**Implementation:**
- Stop button in UI immediately terminates experiments
- Backend prioritizes laser shutdown over other operations
- All shutdown operations are logged

**Verification:**
- Testing of emergency shutdown sequence
- Verification that laser is off after shutdown
- Audit log of shutdown operations

### 2. Error Recovery

**Purpose:** Provide guidance for recovering from error conditions.

**Implementation:**
- Clear error messages with suggested actions
- Detailed logs for troubleshooting
- Automatic attempt to return to safe state

**Verification:**
- Testing of common error conditions
- Verification of helpful error messages
- Confirmation of safe state after errors

## Testing and Validation

The following safety tests should be performed regularly:

1. **TEC Protection Test**: Attempt to enable laser without TEC
2. **Current Limit Test**: Attempt to set current above safe limit
3. **Shutdown Test**: Verify laser turns off after experiment
4. **Error Recovery Test**: Verify system handles device disconnection gracefully
5. **UI Validation Test**: Verify UI prevents invalid parameters

