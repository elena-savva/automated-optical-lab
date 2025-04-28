#![allow(unused)]

use std::ffi::CString;
use std::io::{BufRead, BufReader, Write};
use std::time::Duration;
use visa_rs::prelude::*;

pub struct CLD1015 {
    device: Option<Instrument>,
    resource_string: String,
}

// Helper function to convert IO errors to VISA errors
fn io_to_vs_err(err: std::io::Error) -> visa_rs::Error {
    visa_rs::io_to_vs_err(err)
}

impl CLD1015 {
    pub fn new(resource_string: &str) -> Self {
        CLD1015 {
            device: None,
            resource_string: resource_string.to_string(),
        }
    }

    pub fn connect(&mut self) -> visa_rs::Result<String> {
        let rm = DefaultRM::new()?;
        let resource = CString::new(self.resource_string.clone()).unwrap();
        let device = rm.open(
            &resource.into(),
            AccessMode::NO_LOCK,
            Duration::from_secs(1),
        )?;

        self.device = Some(device);
        
        // Identify the device
        self.query("*IDN?")
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    pub fn write(&mut self, command: &str) -> visa_rs::Result<()> {
        if let Some(device) = &mut self.device {
            let command_with_newline = format!("{}\n", command);
            device.write_all(command_with_newline.as_bytes()).map_err(io_to_vs_err)?;
            Ok(())
        } else {
            Err(visa_rs::io_to_vs_err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Device not connected",
            )))
        }
    }

    pub fn read(&mut self) -> visa_rs::Result<String> {
        if let Some(device) = &mut self.device {
            let mut response = String::new();
            BufReader::new(device).read_line(&mut response).map_err(io_to_vs_err)?;
            Ok(response.trim().to_string())
        } else {
            Err(visa_rs::io_to_vs_err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Device not connected",
            )))
        }
    }

    pub fn query(&mut self, command: &str) -> visa_rs::Result<String> {
        self.write(command)?;
        self.read()
    }

    pub fn get_tec_state(&mut self) -> visa_rs::Result<bool> {
        let response = self.query("OUTPut2:STATe?")?;
        Ok(response.eq_ignore_ascii_case("ON") || response == "1")
    }

    pub fn set_current_mode(&mut self) -> visa_rs::Result<()> {
        self.write("SOURce:FUNCtion:MODE CURRent")
    }

    pub fn set_current(&mut self, current_amps: f64) -> visa_rs::Result<()> {
        self.write(&format!("SOURce:CURRent:LEVel:IMMediate:AMPLitude {}", current_amps))
    }

    pub fn get_current(&mut self) -> visa_rs::Result<f64> {
        let response = self.query("SOURce:CURRent:LEVel:IMMediate:AMPLitude?")?;
        response.parse::<f64>().map_err(|_| visa_rs::io_to_vs_err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Failed to parse current value",
        )))
    }

    pub fn set_laser_output(&mut self, enabled: bool) -> visa_rs::Result<()> {
        let state = if enabled { "ON" } else { "OFF" };
        self.write(&format!("OUTPut:STATe {}", state))
    }

    pub fn get_laser_output(&mut self) -> visa_rs::Result<bool> {
        let response = self.query("OUTPut:STATe?")?;
        Ok(response.eq_ignore_ascii_case("ON") || response == "1")
    }
}
