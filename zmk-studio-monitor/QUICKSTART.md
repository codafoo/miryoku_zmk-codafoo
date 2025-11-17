# Quick Start Guide

## System Dependencies

Before building, you need to install system dependencies:

### Ubuntu / Debian
```bash
sudo apt-get update
sudo apt-get install -y libudev-dev pkg-config libusb-1.0-0-dev
```

### Fedora / RHEL
```bash
sudo dnf install systemd-devel libusb1-devel
```

### Arch Linux
```bash
sudo pacman -S systemd libusb
```

### macOS
```bash
brew install libusb
```

## Building

```bash
cd zmk-studio-monitor

# Check compilation (faster)
cargo check

# Build debug binary
cargo build

# Build optimized release binary
cargo build --release
```

## Running

```bash
# Run directly with cargo (debug mode)
cargo run

# Run release build
cargo run --release

# Or run the compiled binary
./target/release/zmk-studio-monitor
```

## Controls

- **1-4**: Switch tabs (Overview / Keyboard / Key Events / Logs)
- **↑/↓**: Scroll logs and events
- **PgUp/PgDn**: Fast scroll
- **q**: Quit

## Mock Mode

The tool runs in **mock/demo mode** by default if no ZMK Studio device is found.

This allows you to:
- Test the UI without hardware
- See all features in action
- Learn the interface
- Develop and debug

Mock mode simulates:
- roBa keyboard (43 keys, 11 layers)
- Battery at 87%, slowly draining
- Random layer changes
- Random key presses
- Periodic debug log messages
- Bluetooth connection at -65 dBm

## Connecting to Real Hardware

### USB Connection

1. Connect your ZMK keyboard via USB
2. Ensure ZMK Studio is enabled in your firmware
3. Run the monitor - it should auto-detect

### Linux Permissions

If you get permission errors:

```bash
# Create udev rule for your keyboard
sudo nano /etc/udev/rules.d/50-zmk.rules

# Add this line (replace XXXX with your keyboard's vendor ID):
SUBSYSTEM=="usb", ATTRS{idVendor}=="XXXX", MODE="0666"

# Reload rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Or add yourself to the dialout/plugdev group
sudo usermod -a -G dialout,plugdev $USER
# Log out and back in for group changes to take effect
```

### Bluetooth Connection

Bluetooth support requires additional setup:
- Keyboard must be paired with the system
- ZMK Studio must support BLE communication
- May require special permissions

## Troubleshooting

### Build Errors

**Error: "Unable to find libudev"**
```bash
# Install missing dependencies (see System Dependencies above)
sudo apt-get install libudev-dev pkg-config
```

**Error: "package not found"**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Runtime Issues

**No device found**
- Check USB connection
- Verify ZMK Studio is enabled in firmware
- Check permissions (see Linux Permissions above)
- Tool will run in mock mode automatically

**UI rendering issues**
- Ensure terminal supports colors (modern terminals)
- Try a different terminal emulator
- Check terminal size (minimum 80x24 recommended)

**Laggy performance**
- Use release build: `cargo run --release`
- Reduce polling rate (edit `POLL_INTERVAL_MS` in monitor.rs)
- Close other applications

## Next Steps

1. Read the full [README.md](README.md) for detailed documentation
2. Explore the codebase to understand the architecture
3. Customize the UI in `src/ui.rs`
4. Extend the protocol in `src/protocol.rs`
5. Add new features and submit PRs!

## Support

For issues, questions, or contributions:
- Check the README.md
- Review the source code comments
- Open an issue in the parent repository
