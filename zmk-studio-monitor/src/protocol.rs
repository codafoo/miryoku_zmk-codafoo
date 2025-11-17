// ZMK Studio RPC Protocol Implementation
// Based on ZMK Studio protocol specification

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("HID error: {0}")]
    Hid(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Invalid response: {0}")]
    InvalidResponse(String),
    #[error("Communication timeout")]
    Timeout,
}

// ZMK Studio RPC message types
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    // Core messages
    Request = 0x00,
    Response = 0x01,
    Notification = 0x02,

    // Request types
    GetDeviceInfo = 0x10,
    GetKeymap = 0x11,
    GetLayerNames = 0x12,
    GetBehaviors = 0x13,

    // Live state
    GetActiveLayer = 0x20,
    GetBatteryLevel = 0x21,
    GetConnectionState = 0x22,

    // Keymap operations
    SetKeyBinding = 0x30,
    SetLayerName = 0x31,

    // Debug
    GetLogMessages = 0x40,
    SubscribeEvents = 0x41,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub name: String,
    pub version: String,
    pub firmware: String,
    pub layout: String,
    pub key_count: u16,
    pub layer_count: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub level: u8,        // 0-100
    pub voltage: u16,      // millivolts
    pub charging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerState {
    pub active_layer: u8,
    pub layer_names: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPress {
    pub timestamp: u64,
    pub layer: u8,
    pub position: u16,
    pub key_code: String,
    pub pressed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    pub timestamp: u64,
    pub level: LogLevel,
    pub message: String,
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum LogLevel {
    Error = 0,
    Warning = 1,
    Info = 2,
    Debug = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    pub connected: bool,
    pub connection_type: ConnectionType,
    pub signal_strength: Option<i8>, // dBm for BLE
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum ConnectionType {
    Usb = 0,
    Bluetooth = 1,
}

// RPC Message structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcMessage {
    pub msg_type: MessageType,
    pub sequence: u16,
    pub payload: Vec<u8>,
}

impl RpcMessage {
    pub fn new_request(msg_type: MessageType, sequence: u16, payload: Vec<u8>) -> Self {
        Self {
            msg_type,
            sequence,
            payload,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, ProtocolError> {
        postcard::to_stdvec(self)
            .map_err(|e| ProtocolError::Serialization(e.to_string()))
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, ProtocolError> {
        postcard::from_bytes(data)
            .map_err(|e| ProtocolError::Serialization(e.to_string()))
    }
}

// HID Report structure for ZMK Studio
pub const ZMK_STUDIO_USAGE_PAGE: u16 = 0xFF00;
pub const ZMK_STUDIO_USAGE: u16 = 0x0001;
pub const REPORT_SIZE: usize = 64;

#[derive(Debug)]
pub struct HidReport {
    pub data: [u8; REPORT_SIZE],
    pub length: usize,
}

impl HidReport {
    pub fn new() -> Self {
        Self {
            data: [0; REPORT_SIZE],
            length: 0,
        }
    }

    pub fn from_message(msg: &RpcMessage) -> Result<Self, ProtocolError> {
        let bytes = msg.to_bytes()?;

        if bytes.len() > REPORT_SIZE - 1 {
            return Err(ProtocolError::Serialization(
                "Message too large for HID report".to_string()
            ));
        }

        let mut report = Self::new();
        report.data[0] = bytes.len() as u8;
        report.data[1..bytes.len() + 1].copy_from_slice(&bytes);
        report.length = bytes.len() + 1;

        Ok(report)
    }

    pub fn to_message(&self) -> Result<RpcMessage, ProtocolError> {
        if self.length == 0 {
            return Err(ProtocolError::InvalidResponse("Empty report".to_string()));
        }

        let msg_len = self.data[0] as usize;
        if msg_len > self.length - 1 {
            return Err(ProtocolError::InvalidResponse(
                "Invalid message length".to_string()
            ));
        }

        RpcMessage::from_bytes(&self.data[1..msg_len + 1])
    }
}

// Protocol client for communicating with ZMK keyboard
pub struct ZmkStudioClient {
    device: Option<hidapi::HidDevice>,
    sequence: u16,
}

impl ZmkStudioClient {
    pub fn new() -> Result<Self, ProtocolError> {
        Ok(Self {
            device: None,
            sequence: 0,
        })
    }

    pub fn connect(&mut self) -> Result<(), ProtocolError> {
        let api = hidapi::HidApi::new()
            .map_err(|e| ProtocolError::Hid(e.to_string()))?;

        // Try to find ZMK Studio device
        // In reality, you'd scan for devices with ZMK Studio capability
        // For now, we'll use a mock implementation

        tracing::info!("Scanning for ZMK Studio devices...");

        for device_info in api.device_list() {
            tracing::debug!(
                "Found device: {:04x}:{:04x} - {}",
                device_info.vendor_id(),
                device_info.product_id(),
                device_info.product_string().unwrap_or("Unknown")
            );

            // Check if this is a ZMK device
            // You would check for ZMK-specific VID/PID or usage page
            if Self::is_zmk_device(device_info) {
                let device = device_info.open_device(&api)
                    .map_err(|e| ProtocolError::Hid(e.to_string()))?;

                tracing::info!("Connected to ZMK device");
                self.device = Some(device);
                return Ok(());
            }
        }

        Err(ProtocolError::Hid("No ZMK Studio device found".to_string()))
    }

    fn is_zmk_device(device_info: &hidapi::DeviceInfo) -> bool {
        // Check for ZMK Studio usage page
        device_info.usage_page() == ZMK_STUDIO_USAGE_PAGE &&
        device_info.usage() == ZMK_STUDIO_USAGE
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    fn next_sequence(&mut self) -> u16 {
        self.sequence = self.sequence.wrapping_add(1);
        self.sequence
    }

    pub fn send_request(&mut self, msg_type: MessageType, payload: Vec<u8>) -> Result<RpcMessage, ProtocolError> {
        let device = self.device.as_ref()
            .ok_or_else(|| ProtocolError::Hid("Not connected".to_string()))?;

        let sequence = self.next_sequence();
        let request = RpcMessage::new_request(msg_type, sequence, payload);
        let report = HidReport::from_message(&request)?;

        // Send request
        device.write(&report.data[..report.length])
            .map_err(|e| ProtocolError::Hid(e.to_string()))?;

        // Read response
        let mut response_data = [0u8; REPORT_SIZE];
        let len = device.read_timeout(&mut response_data, 1000)
            .map_err(|e| ProtocolError::Hid(e.to_string()))?;

        if len == 0 {
            return Err(ProtocolError::Timeout);
        }

        let mut response_report = HidReport::new();
        response_report.data[..len].copy_from_slice(&response_data[..len]);
        response_report.length = len;

        response_report.to_message()
    }

    pub fn get_device_info(&mut self) -> Result<DeviceInfo, ProtocolError> {
        let response = self.send_request(MessageType::GetDeviceInfo, vec![])?;

        serde_json::from_slice(&response.payload)
            .map_err(|e| ProtocolError::Serialization(e.to_string()))
    }

    pub fn get_battery_info(&mut self) -> Result<BatteryInfo, ProtocolError> {
        let response = self.send_request(MessageType::GetBatteryLevel, vec![])?;

        serde_json::from_slice(&response.payload)
            .map_err(|e| ProtocolError::Serialization(e.to_string()))
    }

    pub fn get_layer_state(&mut self) -> Result<LayerState, ProtocolError> {
        let response = self.send_request(MessageType::GetActiveLayer, vec![])?;

        serde_json::from_slice(&response.payload)
            .map_err(|e| ProtocolError::Serialization(e.to_string()))
    }

    pub fn get_connection_state(&mut self) -> Result<ConnectionState, ProtocolError> {
        let response = self.send_request(MessageType::GetConnectionState, vec![])?;

        serde_json::from_slice(&response.payload)
            .map_err(|e| ProtocolError::Serialization(e.to_string()))
    }

    pub fn get_log_messages(&mut self) -> Result<Vec<LogMessage>, ProtocolError> {
        let response = self.send_request(MessageType::GetLogMessages, vec![])?;

        serde_json::from_slice(&response.payload)
            .map_err(|e| ProtocolError::Serialization(e.to_string()))
    }
}
