// ZMK Monitor - Real-time keyboard monitoring

use crate::protocol::*;
use anyhow::Result;
use chrono::{DateTime, Local};
use std::collections::VecDeque;
use tokio::time::{interval, Duration};

const MAX_LOG_MESSAGES: usize = 1000;
const MAX_KEY_EVENTS: usize = 100;
const POLL_INTERVAL_MS: u64 = 100;

#[derive(Debug, Clone)]
pub struct KeyboardState {
    pub device_info: Option<DeviceInfo>,
    pub battery: Option<BatteryInfo>,
    pub layer_state: Option<LayerState>,
    pub connection: Option<ConnectionState>,
    pub last_update: DateTime<Local>,
    pub connected: bool,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            device_info: None,
            battery: None,
            layer_state: None,
            connection: None,
            last_update: Local::now(),
            connected: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonitorStats {
    pub total_key_presses: u64,
    pub session_duration: Duration,
    pub average_wpm: f32,
    pub keys_per_layer: Vec<(u8, u64)>,
}

impl MonitorStats {
    pub fn new() -> Self {
        Self {
            total_key_presses: 0,
            session_duration: Duration::from_secs(0),
            average_wpm: 0.0,
            keys_per_layer: Vec::new(),
        }
    }
}

pub struct ZmkMonitor {
    client: ZmkStudioClient,
    state: KeyboardState,
    log_messages: VecDeque<LogMessage>,
    key_events: VecDeque<KeyPress>,
    stats: MonitorStats,
    session_start: DateTime<Local>,
    mock_mode: bool,
}

impl ZmkMonitor {
    pub fn new() -> Result<Self> {
        let client = ZmkStudioClient::new()?;

        Ok(Self {
            client,
            state: KeyboardState::new(),
            log_messages: VecDeque::with_capacity(MAX_LOG_MESSAGES),
            key_events: VecDeque::with_capacity(MAX_KEY_EVENTS),
            stats: MonitorStats::new(),
            session_start: Local::now(),
            mock_mode: true, // Enable mock mode for demo
        })
    }

    pub async fn start_monitoring(&mut self) -> Result<()> {
        // Try to connect
        if !self.mock_mode {
            match self.client.connect() {
                Ok(_) => {
                    self.state.connected = true;
                    self.initialize_state()?;
                }
                Err(e) => {
                    tracing::warn!("Failed to connect to device: {}. Running in mock mode.", e);
                    self.mock_mode = true;
                    self.initialize_mock_state();
                }
            }
        } else {
            self.initialize_mock_state();
        }

        // Start polling loop
        let mut ticker = interval(Duration::from_millis(POLL_INTERVAL_MS));

        loop {
            ticker.tick().await;
            self.update_state().await?;
        }
    }

    fn initialize_state(&mut self) -> Result<()> {
        self.state.device_info = Some(self.client.get_device_info()?);
        self.state.battery = self.client.get_battery_info().ok();
        self.state.layer_state = self.client.get_layer_state().ok();
        self.state.connection = self.client.get_connection_state().ok();
        self.state.last_update = Local::now();

        Ok(())
    }

    fn initialize_mock_state(&mut self) {
        self.state.device_info = Some(DeviceInfo {
            name: "roBa (Mock)".to_string(),
            version: "1.0.0".to_string(),
            firmware: "ZMK 3.5.0".to_string(),
            layout: "43-key split".to_string(),
            key_count: 43,
            layer_count: 11,
        });

        self.state.battery = Some(BatteryInfo {
            level: 87,
            voltage: 4150,
            charging: false,
        });

        self.state.layer_state = Some(LayerState {
            active_layer: 0,
            layer_names: vec![
                "BASE".to_string(),
                "EXTRA".to_string(),
                "TAP".to_string(),
                "BUTTON".to_string(),
                "NAV".to_string(),
                "MOUSE".to_string(),
                "MEDIA".to_string(),
                "NUM".to_string(),
                "SYM".to_string(),
                "FUN".to_string(),
                "ALT".to_string(),
            ],
        });

        self.state.connection = Some(ConnectionState {
            connected: true,
            connection_type: ConnectionType::Bluetooth,
            signal_strength: Some(-65),
        });

        self.state.connected = true;
        self.state.last_update = Local::now();

        // Add some mock log messages
        self.add_log(LogLevel::Info, "ZMK Monitor started");
        self.add_log(LogLevel::Info, "Running in mock/demo mode");
        self.add_log(LogLevel::Debug, "Keyboard initialized successfully");
    }

    async fn update_state(&mut self) -> Result<()> {
        if self.mock_mode {
            self.update_mock_state().await;
        } else if self.client.is_connected() {
            // Update battery (less frequently)
            if rand::random::<u8>() % 100 == 0 {
                self.state.battery = self.client.get_battery_info().ok();
            }

            // Update layer state
            self.state.layer_state = self.client.get_layer_state().ok();

            // Get new log messages
            if let Ok(logs) = self.client.get_log_messages() {
                for log in logs {
                    self.add_log_message(log);
                }
            }

            self.state.last_update = Local::now();
        }

        // Update stats
        self.update_stats();

        Ok(())
    }

    async fn update_mock_state(&mut self) {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // Randomly simulate layer changes
        if rng.gen_ratio(1, 100) {
            if let Some(ref mut layer_state) = self.state.layer_state {
                layer_state.active_layer = rng.gen_range(0..11);
                self.add_log(
                    LogLevel::Debug,
                    &format!("Layer changed to {}", layer_state.layer_names[layer_state.active_layer as usize])
                );
            }
        }

        // Simulate battery drain
        if rng.gen_ratio(1, 1000) {
            if let Some(ref mut battery) = self.state.battery {
                if battery.level > 0 {
                    battery.level -= 1;
                    battery.voltage = 3500 + (battery.level as u16 * 7);
                }
            }
        }

        // Simulate key presses
        if rng.gen_ratio(1, 10) {
            let layer = self.state.layer_state.as_ref()
                .map(|ls| ls.active_layer)
                .unwrap_or(0);

            let key_event = KeyPress {
                timestamp: Local::now().timestamp_millis() as u64,
                layer,
                position: rng.gen_range(0..43),
                key_code: Self::random_key_code(&mut rng),
                pressed: true,
            };

            self.add_key_event(key_event);
            self.stats.total_key_presses += 1;
        }

        // Simulate occasional log messages
        if rng.gen_ratio(1, 200) {
            let messages = [
                "USB device enumerated",
                "Bluetooth connection stable",
                "Layer cache updated",
                "Power management: sleep mode",
                "Matrix scan complete",
            ];
            let msg = messages[rng.gen_range(0..messages.len())];
            self.add_log(LogLevel::Debug, msg);
        }

        self.state.last_update = Local::now();
    }

    fn random_key_code<R: rand::Rng>(rng: &mut R) -> String {
        let keys = ["A", "S", "D", "F", "G", "H", "J", "K", "L", "SPC", "ENT", "BSPC"];
        keys[rng.gen_range(0..keys.len())].to_string()
    }

    fn update_stats(&mut self) {
        self.stats.session_duration = (Local::now() - self.session_start)
            .to_std()
            .unwrap_or(Duration::from_secs(0));

        // Calculate WPM (assuming average word is 5 characters)
        let minutes = self.stats.session_duration.as_secs_f32() / 60.0;
        if minutes > 0.0 {
            self.stats.average_wpm = (self.stats.total_key_presses as f32 / 5.0) / minutes;
        }
    }

    fn add_log(&mut self, level: LogLevel, message: &str) {
        let log = LogMessage {
            timestamp: Local::now().timestamp_millis() as u64,
            level,
            message: message.to_string(),
        };
        self.add_log_message(log);
    }

    fn add_log_message(&mut self, log: LogMessage) {
        if self.log_messages.len() >= MAX_LOG_MESSAGES {
            self.log_messages.pop_front();
        }
        self.log_messages.push_back(log);
    }

    fn add_key_event(&mut self, event: KeyPress) {
        if self.key_events.len() >= MAX_KEY_EVENTS {
            self.key_events.pop_front();
        }
        self.key_events.push_back(event);
    }

    pub fn get_state(&self) -> &KeyboardState {
        &self.state
    }

    pub fn get_log_messages(&self) -> &VecDeque<LogMessage> {
        &self.log_messages
    }

    pub fn get_key_events(&self) -> &VecDeque<KeyPress> {
        &self.key_events
    }

    pub fn get_stats(&self) -> &MonitorStats {
        &self.stats
    }
}

// Add rand dependency for mock mode
use rand;
