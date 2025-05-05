# Device Driver Documentation

## Overview

This document provides detailed information about the device drivers used in the Automated Optical Lab Control Suite. The system uses two primary devices:

1. **CLD1015** - Thorlabs Laser Diode Controller
2. **MPM210H** - Santec Multi-Port Optical Power Meter

Each device has a corresponding Rust driver module that handles communication, command formatting, and error handling.

## CLD1015 Laser Diode Controller

### External Dependencies

**Required Drivers:**
- **National Instruments VISA** (or compatible VISA implementation)
  - Version 19.0 or newer recommended
  - Download from [NI VISA Download Page](https://www.ni.com/en-us/support/downloads/drivers/download.ni-visa.html)
  - Must be installed before connecting the device

The CLD1015 driver relies on the external VISA driver to communicate with the hardware. Without this driver, the application will fail to establish communication with the laser controller.

### Installation Verification

After installing the required VISA driver:
1. Connect the CLD1015 to your computer via USB
2. Open the NI MAX (Measurement & Automation Explorer) or equivalent
3. Verify the device appears under "Devices and Interfaces"
4. Note the VISA resource string (e.g., `USB0::4883::32847::M01053290::0::INSTR`)
5. Use this string in your application configuration

### Communication Protocol

The CLD1015 uses the VISA (Virtual Instrument Software Architecture) protocol over USB for communication. The driver utilizes the `visa-rs` crate to establish and maintain this connection.

### Driver Interface

```rust
pub struct CLD1015 {
    device: Option<Instrument>,
    resource_string: String,
}
```

**Key Methods:**

- `new(resource_string: &str) -> Self`: Creates a new instance with the specified VISA resource string
- `connect() -> visa_rs::Result<String>`: Establishes connection and returns device identification
- `enable_tec() -> visa_rs::Result<()>`: Enables the thermoelectric cooler
- `get_tec_state() -> visa_rs::Result<bool>`: Gets the current TEC state
- `set_current(current_amps: f64) -> visa_rs::Result<()>`: Sets the laser current
- `set_laser_output(enabled: bool) -> visa_rs::Result<()>`: Enables/disables laser output
- `get_error() -> visa_rs::Result<String>`: Gets error information from the device

### Safety Mechanisms

1. **TEC Validation**: The driver prevents enabling the laser without first enabling the TEC
2. **Current Limiting**: A hard limit of 1.5A is enforced to prevent laser damage
3. **Error Checking**: All commands verify success and propagate errors

### Example Usage

```rust
let mut cld = CLD1015::new("USB0::4883::32847::M01053290::0::INSTR");
cld.connect()?;
cld.enable_tec()?;
cld.set_current(0.050)?; // 50mA
cld.set_laser_output(true)?;
```

### Error Handling

The driver converts device-specific errors and IO errors to VISA result types, providing consistent error handling throughout the application. All errors are logged for debugging purposes.

## MPM210H Multi-Port Optical Power Meter

### External Dependencies

**Required Network Configuration:**
- Local network access with TCP/IP connectivity
- Static IP configuration recommended
- No additional driver software required, but proper network setup is essential

Unlike the CLD1015, the MPM210H does not require special driver software, but does require proper network configuration. The device must be accessible on the network with a known IP address (default: 192.168.1.161).

### Network Configuration Verification

1. Connect the MPM210H to the local network
2. Set your computer's network configuration to the same subnet
   - Example: If MPM210H is 192.168.1.161, configure PC with 192.168.1.x address
   - Use subnet mask 255.255.255.0
3. Verify connectivity with ping test:
   ```
   ping 192.168.1.161
   ```
4. If ping fails, check network cables and configuration

### Communication Protocol

The MPM210H uses TCP/IP socket communication for control and data acquisition. The driver implements a custom protocol parser for this device.

### Driver Interface

```rust
pub struct MPM210H {
    connection: Option<TcpStream>,
    address: String,
}
```

**Key Methods:**

- `new(ip_address: &str, port: u16) -> Self`: Creates a new instance with the specified IP address and port
- `connect() -> Result<String>`: Establishes connection and returns device identification
- `send_command(command: &str) -> Result<()>`: Sends a raw command to the device
- `read_response() -> Result<String>`: Reads a response from the device
- `query(command: &str) -> Result<String>`: Sends a command and reads the response
- `read_power(module: u8) -> Result<String>`: Reads optical power from specified module
- `set_wavelength(wavelength: u32) -> Result<()>`: Sets the measurement wavelength

### Command Structure

Commands follow the format:
```
COMMAND<wsp>[<Parameter1>][,<Parameter2>][,<Parameter3>]
```

For example:
- `WAV 1550` - Sets wavelength to 1550nm
- `READ? 0` - Reads power values from module 0

### Error Handling

The driver defines a custom error type `MPM210HError` that captures:
- IO errors during communication
- Parse errors for invalid responses
- Connection state errors

All errors are logged with appropriate context for debugging.

### Example Usage

```rust
let mut mpm = MPM210H::new("192.168.1.161", 5000);
mpm.connect()?;
mpm.set_wavelength(980)?;
let power = mpm.read_power(1)?;
```

## Integration Between Devices

### Wavelength Synchronization

The MPM210H wavelength must be set to match the emission wavelength of the CLD1015 (typically 980nm) for accurate power measurements. This synchronization is handled in the experiment module.

### Measurement Timing

Proper timing between setting the laser current and measuring optical power is critical:
1. Set CLD1015 current
2. Wait for stabilization delay (typically 10-20ms)
3. Read power from MPM210H

### Safety Interlocks

The system implements software safety interlocks:
1. TEC must be enabled before laser activation
2. Laser is disabled at the beginning and end of measurements
3. Current limits are enforced at the driver level

## Extension Guide

### Adding New Device Drivers

To add support for a new device:

1. Create a new module under `src-tauri/src/devices/`
2. Implement a struct with connection management and command methods
3. Define appropriate error types and conversion functions
4. Add the device to the application state in `main.rs`
5. Add command handlers for the new device

### Required Traits

New device drivers should implement:
- Connection management (connect/disconnect)
- Error handling with logging
- Resource cleanup
- Status reporting

## Communication Troubleshooting

### CLD1015 Issues

- **Connection Failure**: Verify VISA resource string and USB connection
- **Command Errors**: Check command syntax against device documentation
- **Timeout Errors**: Increase timeout in device settings

### MPM210H Issues

- **Connection Failure**: Verify IP address, port, and network connectivity
- **Command Errors**: Ensure command format matches device expectations
- **Data Format Errors**: Check response parsing logic
