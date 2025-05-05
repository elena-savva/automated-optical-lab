# System Architecture Documentation

## Overview

The Automated Optical Lab Control Suite is a modular software system designed to control and automate optical measurement equipment. It combines a Rust backend for hardware communication with a Vue.js frontend for visualization and user interaction.

## Component Description

### 1. Frontend Layer (Vue.js)

**Components:**
- **MainView**: Primary layout container that organizes all subcomponents
- **DeviceStatus**: Displays connection status of optical devices
- **ControlsPanel**: Interface for configuring and starting experiments
- **LIPlot**: Real-time visualization of L-I curve measurements

**Data Flow:**
- User input captured through form elements and button clicks
- Device status updates received through Tauri events
- Measurement data streamed from backend via events
- Visualization updated in real-time

### 2. Tauri Bridge Layer

**Components:**
- **Command Handlers**: Functions mapped to frontend calls
- **Event System**: Bidirectional communication channel
- **State Management**: Shared application state between frontend and backend

**Data Flow:**
- Commands from frontend → backend processing
- Events from backend → frontend updates
- Shared state synchronized between layers

### 3. Backend Layer (Rust)

**Modules:**
- **Devices**: Hardware communication drivers
  - CLD1015: Laser diode controller driver
  - MPM210H: Multi-port power meter driver
- **Experiment**: Measurement orchestration logic
  - Data acquisition routines
  - Measurement sequencing
  - Safety validation
- **Data Storage**: CSV file management for measurement results

**Data Flow:**
- Command requests → device communication
- Measurement results → data storage and frontend events
- Error handling and status reporting

### 4. Hardware Interface Layer

**Devices:**
- **CLD1015**: Thorlabs laser diode controller with USB/VISA interface
- **MPM210H**: Santec multi-port optical power meter with TCP/IP interface

**Communication:**
- VISA protocol for USB instrumentation
- TCP/IP for network-enabled devices
- Standardized command sets for each device

## Data Flow Sequences

### Device Connection Sequence

1. User clicks "Connect" button in UI
2. Tauri command handler invokes device driver connection method
3. Driver attempts to establish communication with hardware
4. Connection status reported back to UI
5. UI updates device status display

### Measurement Sequence

1. User configures measurement parameters and clicks "Start"
2. Parameters validated and passed to experiment module
3. Experiment module performs safety checks
4. Current sweep executed with real-time data collection
5. Each data point emitted as event to frontend
6. Frontend updates visualization in real-time
7. Measurement data saved to CSV file
8. File path returned to frontend for reference

## Error Handling Strategy

1. **Frontend**: Input validation and user feedback
2. **Bridge**: Command validation and error propagation
3. **Backend**: 
   - Device-specific error handling
   - Safety check failures
   - Resource access errors
4. **Hardware**: Communication timeouts and protocol errors

## Security Considerations

1. **Device Safety**: Multiple checks to prevent hardware damage
2. **File System**: Limited access to specific directories
3. **Network**: Restricted to local network for device communication
