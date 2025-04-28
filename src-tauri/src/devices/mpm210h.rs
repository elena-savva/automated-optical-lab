#![allow(unused)]

use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use thiserror::Error;

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
        MPM210H {
            connection: None,
            address: format!("{}:{}", ip_address, port),
        }
    }

    pub fn connect(&mut self) -> Result<String> {
        let socket_addr: SocketAddr = self.address.parse()
            .map_err(|e: std::net::AddrParseError| MPM210HError::ParseError(e.to_string()))?;
        
        let stream = TcpStream::connect(socket_addr)?;
        stream.set_read_timeout(Some(Duration::from_secs(2)))?;
        
        self.connection = Some(stream);
        
        // Return the device identification
        self.query("*IDN?")
    }
    

    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    pub fn send_command(&mut self, command: &str) -> Result<()> {
        if let Some(stream) = &mut self.connection {
            let cmd = format!("{}\n", command);
            stream.write_all(cmd.as_bytes())?;
            stream.flush()?;
            Ok(())
        } else {
            Err(MPM210HError::NotConnected)
        }
    }

    pub fn read_response(&mut self) -> Result<String> {
        if let Some(stream) = &mut self.connection {
            let mut buf = [0_u8; 1024];
            let n = stream.read(&mut buf)?;
            Ok(String::from_utf8_lossy(&buf[..n]).trim().to_string())
        } else {
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
        self.query(&format!("READ? {}", module))
    }

    pub fn get_wavelength(&mut self) -> Result<String> {
        self.query("WAV?")
    }

    pub fn set_wavelength(&mut self, wavelength: u32) -> Result<()> {
        self.send_command(&format!("WAV {}", wavelength))
    }
}
