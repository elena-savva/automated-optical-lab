# Optical Lab Control Suite

A modular software architecture for automating optical laboratory experiments. Built with Rust and Vue.js, this system provides automated control of optical instruments via standard communication protocols.

## Features

- **Instrument Control**: Supports Thorlabs CLD1015 Laser Diode Controller (USB) and Santec MPM-210H Power Meter (TCP/IP)
- **Safety Mechanisms**: TEC validation, current limiting (1.5A max), automatic laser shutdown
- **Experiment Automation**: Configurable current sweeps with real-time power measurements
- **Data Management**: Automatic CSV logging with timestamps
- **Real-time Visualization**: Live L-I curve plotting during experiments

## Prerequisites

- Rust (latest stable)
- Node.js v16+
- VISA drivers for USB communication
- Compatible optical instruments

## Installation

```bash
git clone https://github.com/elena-savva/automated-optical-lab
cd optical-lab-suite
npm install
npm run tauri dev
```

## Usage

1. Connect optical instruments
2. Launch application and verify device connections
3. Configure current sweep parameters
4. Start experiment
5. Data automatically saved to `./logs/`

## Architecture

- **Backend**: Rust with Tauri
- **Frontend**: Vue 3 with TypeScript
- **Communication**: visa-rs (USB), TCP/IP sockets
- **Data Storage**: CSV files with structured logging

## Project Structure

```
optical-lab-suite/
├── src-tauri/          # Rust backend
│   └── src/
│       ├── devices/    # Instrument drivers
│       └── experiment/ # Experiment logic
├── src/                # Vue frontend
└── logs/               # Data output
```


## Contact

Elena Savva - e.savva@student.tue.nl

