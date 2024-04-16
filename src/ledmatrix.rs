#![allow(dead_code)]
use std::{thread, time::{Duration, SystemTime}};
use serialport::{SerialPortInfo, SerialPortType};
use crate::matrix;

const BRIGHTNESS_CMD:u8 = 0x00;
const PATTERN_CMD:u8 = 0x01;
const BOOTLOADER_CMD:u8 = 0x02;
const SLEEP_CMD:u8 = 0x03;
const ANIMATE_CMD:u8 = 0x04;
const PANIC_CMD:u8 = 0x05;
const DRAW_CMD:u8 = 0x06;
const SET_COL:u8 = 0x07;
const COMMIT_COL:u8 = 0x08;

const CHECKFW_CMD:u8 =  0x20;

const CMD_START: [u8;2] = [0x32, 0xAC];

pub struct LedMatrix {
    port: Box<dyn serialport::SerialPort>,
    pub port_info :SerialPortInfo
}

impl LedMatrix {

    ///
    /// Find LED matricies connected to the laptop.
    /// Searches for serial ports connected with the LED matrix' product ID & vendor ID
    /// 
    pub fn detect() -> Vec<SerialPortInfo> {
        let sports = serialport::available_ports().expect("No ports found!");

        // Loop through all available serial ports, save ports that match the LED matrix product name
        let mut found_ledmat: Vec<SerialPortInfo> = vec![];
        for ref sp in sports {
            // println!("{:?}", sp.port_type);
            match sp.port_type {
                SerialPortType::UsbPort(ref info) => {
                    let info_c = info.clone();
                    if info_c.vid == 12972 && info_c.pid == 32 {
                        found_ledmat.push(sp.clone());
                    }
                },
                _ => {}
            }
        }

        found_ledmat
    }

    /// 
    /// Creates and connects to an LED matrix 
    /// 
    pub fn new(portinfo: SerialPortInfo) -> LedMatrix {

        let port0builder = serialport::new(portinfo.port_name.to_string(), 115_200);
        let port0 = port0builder.open().expect("Failed to open serial port");
        
        LedMatrix {
            port: port0,
            port_info: portinfo
        }
    }

    ///
    /// Send a command to the LED matrix module.
    /// 1. Send the bytes 0x32 0xAC to initiate a command
    /// 2. Send the command byte (as listed above)
    /// 3. Send further parameters for the command
    /// 
    pub fn sendcommand(&mut self, cmd: u8, params: Option<&[u8]>) {
        let mut buffer: Vec<u8> = vec![];
        buffer.extend_from_slice(CMD_START.as_slice());
        buffer.push(cmd);
        match params {
            Some(p) => buffer.extend_from_slice(p),
            None => {}
        };
    
        self.port.write(buffer.as_slice()).expect("Failed to send command");
        self.port.flush().unwrap();
    }

    ///
    /// Read back a set amount of bytes from the serial port. Returns Err if
    /// nothing is read and the port times out
    /// 
    pub fn serialread(&mut self, numbytes: usize, timeout: Duration) -> Result<Vec<u8>, &'static str> {

        let start_t = SystemTime::now();
    
        // Wait for bytes to be available
        while self.port.bytes_to_read().unwrap() < 1 {
            
            if start_t.elapsed().unwrap() > timeout {
                return Err("Serial read timed out")
            }
            thread::sleep(Duration::from_millis(10));
        }
    
        let mut buffer: Vec<u8> = vec![0;numbytes];
    
        while self.port.bytes_to_read().unwrap() > 0 {
            self.port.read(buffer.as_mut_slice()).unwrap();
        }
    
        Ok(buffer)
    }

    ///
    /// Get the current firmware version of the LED matrix module.
    /// 
    pub fn get_fw_version(&mut self) -> String {
        self.sendcommand(CHECKFW_CMD, None);
        let bytes = self.serialread(32, Duration::from_secs(5)).unwrap_or(vec![0]);
        if bytes.len() < 3 {
            return "".to_string();
        }
    
        let major = bytes[0];
        let minor = (bytes[1] & 0xF0) >> 4;
        let patch = bytes[1] & 0x0F;
        let pre_release = bytes[2] == 1;
    
        let version = format!("{}.{}.{} Pre Release: {}", major, minor, patch, pre_release);
    
        version
    }

    ///
    /// Tell the module to wake up
    /// 
    pub fn wake(&mut self) {
        self.sendcommand(SLEEP_CMD, Some(&[0]));
    }

    ///
    /// Tell the module to go to sleep
    /// 
    pub fn sleep(&mut self) {
        self.sendcommand(SLEEP_CMD, Some(&[1]));
    }

    ///
    /// Draw a matrix using only ON/OFF commands. Each bit sent in the parameters
    /// is a LED, so a matrix needs to be encoded from a 9x34 array to a 39 byte array.
    /// There is no brightness control with this method.
    /// 
    /// This allows for faster framerates than draw_matrix (with brightnesses) since its
    /// ~0.4% of the data (1/255)
    /// 
    pub fn draw_bool_matrix(&mut self, mat: [[bool;9];34]) {
        let buffer = matrix::encode(mat);
        self.sendcommand(DRAW_CMD, Some(buffer.as_slice()));
    }

    ///
    /// Sets the brightness of every LED in the module (0=OFF, 255=FULL)
    /// 
    pub fn set_full_brightness(&mut self, val: u8) {
        self.sendcommand(BRIGHTNESS_CMD, Some(&[val]));
    }


    ///
    /// Write a single column of LEDs - indexed from left to right, 0-8.
    /// This has brightness control, where 0=OFF and 255=FULL brightness.
    /// Columns are not changed until the commit_col function is run (Allows you to
    /// write all the columns THEN display them at once)
    /// 
    pub fn set_col(&mut self, col: u8, arr: [u8;34]) {
        let mut vec = vec![];
        vec.push(col);
        vec.extend_from_slice(arr.as_slice());
        self.sendcommand(SET_COL, Some(vec.as_slice()));
    }

    ///
    /// Tell the module to display all the LEDs written to with set_col
    /// 
    pub fn commit_col(&mut self) {
        self.sendcommand(COMMIT_COL, Some(&[]));
    }

    ///
    /// Display an entire matrix with individual LED brightness values. Slow updating,
    /// but allows for more complex UIs
    /// 
    pub fn draw_matrix(&mut self, mat: [[u8; 9]; 34]) {
        // Transpose array
        let tpose = matrix::transpose(mat);

        for i in  0..9 {
            self.set_col(i, tpose[i as usize]);
        }

        self.commit_col();
    }
}





