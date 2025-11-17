# ZMK Studio Monitor

A real-time terminal-based monitor for ZMK Studio keyboards written in Rust. Displays live keyboard state, layer information, battery status, key events, and debug logs.

## Features

### 🎹 Real-Time Monitoring
- Live keyboard state updates (100ms polling)
- Active layer tracking with visual indicator
- Battery level and voltage display
- Connection status (USB/Bluetooth)
- Signal strength for wireless connections

### 📊 Key Event Tracking
- Real-time key press logging
- Position and layer information for each key
- Scrollable event history (last 100 events)
- Timestamp for each event

### 🔋 Battery Information
- Battery percentage (0-100%)
- Voltage reading (in volts)
- Charging status indicator
- Color-coded battery level (green/yellow/red)

### 📝 Debug Logs
- Live debug log streaming from keyboard
- Log level filtering (Error, Warning, Info, Debug)
- Scrollable log history (last 1000 messages)
- Color-coded log levels

### 📈 Statistics
- Session duration tracking
- Total key press counter
- Average WPM calculation
- Keys per layer breakdown

### 🎨 Beautiful Terminal UI
- Tab-based navigation
- Color-coded information
- Keyboard shortcuts for quick navigation
- Responsive layout

## Prerequisites

- Rust 1.70 or later
- A ZMK keyboard with Studio support (or run in mock mode)
- USB or Bluetooth connection to keyboard

## Installation

### Build from Source

```bash
cd zmk-studio-monitor
cargo build --release
```

The binary will be at `target/release/zmk-studio-monitor`

### Run Directly

```bash
cargo run --release
```

## Usage

### Starting the Monitor

```bash
# Run in current directory
cargo run --release

# Or run the compiled binary
./target/release/zmk-studio-monitor
```

### Keyboard Controls

- **1-4**: Switch between tabs
  - **1**: Overview - Device info, status, statistics
  - **2**: Keyboard - Layer information
  - **3**: Key Events - Real-time key press log
  - **4**: Logs - Debug messages from keyboard

- **↑/↓**: Scroll through logs and events
- **PgUp/PgDn**: Fast scroll (10 lines at a time)
- **q**: Quit application

## Interface Tabs

### 1. Overview Tab

Shows comprehensive device information:

```
┌─ Device Info ─────────────────┐
│ Device: roBa                  │
│ Firmware: ZMK 3.5.0           │
│ Layout: 43-key split (43 keys)│
│ Layers: 11                    │
└───────────────────────────────┘

┌─ Status ──────────────────────┐
│ Battery: 87% (4.15V)          │
│ Connection: Bluetooth (-65 dBm)│
│ Active Layer: BASE (0)        │
└───────────────────────────────┘

┌─ Statistics ──────────────────┐
│ Session Duration: 5m 23s      │
│ Total Key Presses: 342        │
│ Average WPM: 45.2             │
└───────────────────────────────┘
```

### 2. Keyboard Tab

Layer information:

```
┌─ Active Layer ────────────────┐
│ Current Layer: BASE (Layer 0) │
│                               │
│ Layer represents your current │
│ keymap state.                 │
└───────────────────────────────┘

┌─ All Layers ──────────────────┐
│ ➤ 0: BASE                     │
│   1: EXTRA                    │
│   2: TAP                      │
│   3: BUTTON                   │
│   4: NAV                      │
│   5: MOUSE                    │
│   6: MEDIA                    │
│   7: NUM                      │
│   8: SYM                      │
│   9: FUN                      │
│  10: ALT                      │
└───────────────────────────────┘
```

### 3. Key Events Tab

Real-time key press monitoring:

```
┌─ Key Events (Use ↑/↓ to scroll) - 45 events ─┐
│ [14:32:45.234] Layer 0 Pos 12 -> S           │
│ [14:32:45.156] Layer 0 Pos 34 -> SPC         │
│ [14:32:45.089] Layer 0 Pos 27 -> H           │
│ [14:32:44.923] Layer 0 Pos 10 -> A           │
│ [14:32:44.856] Layer 4 Pos 35 -> ENT         │
│ [14:32:44.789] Layer 0 Pos 18 -> K           │
│ ...                                           │
└───────────────────────────────────────────────┘
```

### 4. Logs Tab

Debug log messages:

```
┌─ Debug Logs (Use ↑/↓ to scroll) - 87 messages ─┐
│ [14:32:46.123] INFO  ZMK Monitor started        │
│ [14:32:46.125] INFO  Running in mock/demo mode  │
│ [14:32:46.127] DEBUG Keyboard initialized       │
│ [14:32:47.234] DEBUG Layer changed to NAV       │
│ [14:32:48.456] DEBUG USB device enumerated      │
│ [14:32:49.789] DEBUG Matrix scan complete       │
│ ...                                              │
└──────────────────────────────────────────────────┘
```

## Architecture

### Module Structure

```
zmk-studio-monitor/
├── src/
│   ├── main.rs          # Entry point and initialization
│   ├── protocol.rs      # ZMK Studio RPC protocol implementation
│   ├── monitor.rs       # Keyboard state monitoring logic
│   └── ui.rs            # Terminal UI with ratatui
├── Cargo.toml           # Dependencies and build config
└── README.md            # This file
```

### Protocol Implementation

The monitor uses the ZMK Studio RPC protocol over USB HID or Bluetooth:

- **Message Types**: Request, Response, Notification
- **RPC Commands**:
  - `GetDeviceInfo` - Device metadata
  - `GetBatteryLevel` - Battery status
  - `GetActiveLayer` - Current layer state
  - `GetLogMessages` - Debug logs
  - `GetConnectionState` - Connection info

- **Data Serialization**: Postcard (compact binary format)
- **Transport**: USB HID reports (64 bytes)

### Mock Mode

When no ZMK keyboard is detected, the monitor runs in **mock/demo mode** with:
- Simulated device (roBa keyboard)
- Random layer changes
- Gradual battery drain
- Random key events
- Periodic log messages

This allows you to:
- Test the UI without hardware
- Develop and debug features
- Demonstrate the tool
- Learn the interface

## ZMK Studio Protocol

The monitor implements the ZMK Studio protocol which provides:

### HID Communication
- Usage Page: `0xFF00`
- Usage: `0x0001`
- Report Size: 64 bytes
- Bi-directional communication

### Message Format
```rust
struct RpcMessage {
    msg_type: MessageType,
    sequence: u16,
    payload: Vec<u8>,
}
```

### Supported Operations
- Device information queries
- Real-time state monitoring
- Battery and connection status
- Layer state tracking
- Debug log streaming
- Event subscriptions

## Development

### Adding New Features

1. **Protocol Extension** (`protocol.rs`):
   - Add new message types
   - Define request/response structures
   - Implement client methods

2. **State Management** (`monitor.rs`):
   - Update `KeyboardState` struct
   - Add polling logic
   - Maintain history buffers

3. **UI Updates** (`ui.rs`):
   - Add new tabs or panels
   - Implement drawing functions
   - Handle user input

### Testing

```bash
# Run with cargo
cargo run

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Build release binary
cargo build --release
```

### Dependencies

Core dependencies:
- `hidapi` - USB HID communication
- `ratatui` - Terminal UI framework
- `crossterm` - Terminal control
- `tokio` - Async runtime
- `serde` / `serde_json` - Serialization
- `postcard` - Compact binary serialization
- `chrono` - Time handling

## Troubleshooting

### No Device Found

If you see "No ZMK Studio device found":

1. **Check USB permissions** (Linux):
   ```bash
   # Add udev rule for your keyboard
   sudo nano /etc/udev/rules.d/50-zmk.rules
   # Add: SUBSYSTEM=="usb", ATTRS{idVendor}=="XXXX", MODE="0666"
   sudo udevadm control --reload-rules
   ```

2. **Check Bluetooth pairing**:
   - Ensure keyboard is paired
   - Verify ZMK Studio is enabled in firmware

3. **Run in mock mode**:
   - The tool automatically falls back to mock mode
   - Test UI functionality without hardware

### Build Issues

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release

# Check for missing system dependencies
# Ubuntu/Debian:
sudo apt-get install libusb-1.0-0-dev pkg-config libudev-dev

# Fedora:
sudo dnf install libusb1-devel systemd-devel
```

### Performance

If the UI is laggy:
- Reduce polling rate in `monitor.rs` (increase `POLL_INTERVAL_MS`)
- Limit log buffer size (`MAX_LOG_MESSAGES`)
- Use release build for better performance

## Future Enhancements

Planned features:
- [ ] Real-time keymap visualization
- [ ] Heat map of key usage
- [ ] Layer transition history
- [ ] Configurable polling intervals
- [ ] Export statistics to CSV
- [ ] Multiple keyboard support
- [ ] Custom key binding display
- [ ] Macro recording
- [ ] Integration with ZMK config files
- [ ] Web dashboard option

## Contributing

This tool is part of the ZMK Miryoku configuration repository. Feel free to:
- Report bugs
- Suggest features
- Submit pull requests
- Improve documentation

## License

Same as the parent ZMK configuration (MIT).

## Credits

- **ZMK Firmware**: https://zmk.dev
- **ZMK Studio**: https://github.com/zmkfirmware/zmk-studio
- **ratatui**: https://github.com/ratatui-org/ratatui
- **Miryoku**: https://github.com/manna-harbour/miryoku

## Related Tools

- `keymap_explorer.html` - Web-based keymap explorer
- `keymap_quick_ref.py` - Terminal-based layout reference
- `keymap_parser.py` - Keymap data extractor
