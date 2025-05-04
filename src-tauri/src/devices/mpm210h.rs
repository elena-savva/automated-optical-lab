#![allow(unused)]

use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use thiserror::Error;
use tracing::{info, warn, error};

#[derive(Error, Debug)]
pub enum MPM210HError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Device not connected")]
    NotConnected,
}

pub type Result<T> = std::result::Result<T, MPM210HError>;

pub struct MPM210H {
    connection: Option<TcpStream>,
    address: String,
}

impl MPM210H {
    pub fn new(ip_address: &str, port: u16) -> Self {
        let address = format!("{}:{}", ip_address, port);
        info!("Initializing MPM210H with address: {}", address);
        MPM210H {
            connection: None,
            address,
        }
    }

    pub fn connect(&mut self) -> Result<String> {
        info!("Attempting to connect to MPM210H at {}", self.address);
        let socket_addr: SocketAddr = self.address.parse()
            .map_err(|e: std::net::AddrParseError| MPM210HError::ParseError(e.to_string()))?;
        
        let stream = TcpStream::connect(socket_addr)?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        
        self.connection = Some(stream);
        
        // Return the device identification
        let id = self.query("*IDN?")?;
        info!("MPM210H connected successfully. IDN: {}", id);
        Ok(id)
    }
    

    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    pub fn send_command(&mut self, command: &str) -> Result<()> {
        if let Some(stream) = &mut self.connection {
            let cmd = format!("{}\n", command);
            info!("Sending command to MPM210H: {}", command);
            stream.write_all(cmd.as_bytes())?;
            stream.flush()?;
            Ok(())
        } else {
            error!("Attempted to send command but MPM210H is not connected");
            Err(MPM210HError::NotConnected)
        }
    }

    pub fn read_response(&mut self) -> Result<String> {
        if let Some(stream) = &mut self.connection {
            let mut buf = [0_u8; 1024];
            let n = stream.read(&mut buf)?;
            let response = String::from_utf8_lossy(&buf[..n]).trim().to_string();
            info!("Received response from MPM210H: {}", response);
            Ok(response)
        } else {
            error!("Attempted to read from MPM210H but device is not connected");
            Err(MPM210HError::NotConnected)
        }
    }

    pub fn query(&mut self, command: &str) -> Result<String> {
        self.send_command(command)?;
        std::thread::sleep(Duration::from_millis(10)); // Add a small delay
        self.read_response()
    }

    pub fn get_recognized_modules(&mut self) -> Result<String> {
        self.query("IDIS?")
    }

    pub fn read_power(&mut self, module: u8) -> Result<String> {
        info!("Reading power from module {}", module);
        self.query(&format!("READ? {}", module))
    }

    pub fn get_wavelength(&mut self) -> Result<String> {
        self.query("WAV?")
    }

    pub fn set_wavelength(&mut self, wavelength: u32) -> Result<()> {
        info!("Setting MPM210H wavelength {}", wavelength);
        self.send_command(&format!("WAV {}", wavelength))
    }

    pub fn get_error(&mut self) -> Result<String> {
        let response = self.query("ERR?")?;
        info!("Queried MPM210H error queue: {}", response);
        Ok(response)
    }

    // Clear all errors (only if needed)
    pub fn clear_error_queue(&mut self) -> Result<Vec<String>> {
        let mut errors = Vec::new();
        loop {
            let response = self.query("ERR?")?;
            info!("Clearing error queue entry from MPM210H: {}", response);
            if response.trim().starts_with("0") || response.to_lowercase().contains("no error") {
                break;
            }
            errors.push(response);
        }
        Ok(errors)
    }    
    
}
