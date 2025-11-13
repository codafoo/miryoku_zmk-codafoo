# roBa Keyboard Setup Guide

The roBa is a 43-key split ergonomic keyboard with an integrated PMW3610 trackball on the right side and automouse functionality.

## Prerequisites

**Good news!** The roBa shield files are automatically pulled from the original repository during the build process via the outboard system. No manual setup required!

## How It Works

The build system automatically:
1. Clones the shield definitions from `kumamuk-git/zmk-config-roBa`
2. Extracts the shield files during compilation
3. Links them into the build directory

This is configured in `.github/workflows/outboards/shields/roBa`.

## Configuration Files

The following configuration files are already set up in this repository:

- **config/roBa.keymap** - Uses custom 43-key roBa Miryoku mapping with custom combos
- **config/roBa_R.conf** - Right side configuration with automouse enabled
- **config/roBa_L.conf** - Left side configuration

### 3. Build Firmware

Once the shield files are in place:

```bash
# Build roBa firmware
gh workflow run build-roBa.yml --ref main

# Or build as part of all keyboards
gh workflow run build-all-keyboards.yml --ref main
```

## roBa Specifications

| Feature | Details |
|---------|---------|
| **Keys** | 43 total (variable per side) |
| **Controller** | Seeeduino XIAO BLE (both sides) |
| **Sensor** | PMW3610 optical sensor (right side) |
| **Extras** | Rotary encoder support |
| **Display** | None |
| **Wireless** | Bluetooth LE |

## Automouse Configuration

The roBa is configured with automouse functionality that automatically activates the MOUSE layer when the trackball is moved:

### Current Settings (config/roBa_R.conf)

```conf
# Automouse triggers MOUSE layer on trackball movement
CONFIG_ZMK_INPUT_LISTENER_INPUT_DEVICE_AUTOMOUSE=y
CONFIG_ZMK_INPUT_LISTENER_INPUT_DEVICE_AUTOMOUSE_TIMEOUT_MS=700

# PMW3610 Sensor Settings
CONFIG_PMW3610_CPI=400                    # Cursor sensitivity
CONFIG_PMW3610_ORIENTATION_180=y          # Sensor rotated 180°
CONFIG_PMW3610_SCROLL_TICK=16             # Scroll sensitivity
CONFIG_PMW3610_POLLING_RATE_SW=125        # 125Hz polling
```

### How Automouse Works

1. **Idle State**: Keyboard operates on BASE/EXTRA/TAP/ALT layers normally
2. **Movement Detected**: When you move the trackball, the MOUSE layer automatically activates
3. **Mouse Actions**:
   - Trackball movement controls cursor
   - Physical keys become mouse buttons (as defined in MOUSE layer)
   - Scroll mode available
4. **Auto-Deactivate**: After 700ms of no movement, returns to previous layer

### MOUSE Layer (Miryoku Layer 5)

The Miryoku MOUSE layer provides:
- **Left side**: Undo/cut/copy/paste shortcuts, GUI/Alt/Ctrl/Shift modifiers
- **Right side**: Mouse buttons, scroll wheel simulation
- **Layer access**: Hold SYM thumb key to access MOUSE layer manually

### Customizing Automouse

To adjust automouse behavior, edit `config/roBa_R.conf`:

**Change timeout (milliseconds):**
```conf
CONFIG_ZMK_INPUT_LISTENER_INPUT_DEVICE_AUTOMOUSE_TIMEOUT_MS=1000  # 1 second
```

**Adjust cursor speed (CPI):**
```conf
CONFIG_PMW3610_CPI=800  # Higher = faster, range: 200-3200
```

**Adjust scroll sensitivity:**
```conf
CONFIG_PMW3610_SCROLL_TICK=32  # Higher = less sensitive
```

## Layout Configuration

The roBa uses the same 4-layer alpha system as other keyboards in this repository:

| Layer | Layout | Description |
|-------|--------|-------------|
| BASE (0) | Colemak-DH | Primary typing layer |
| EXTRA (1) | Nightingale | Secondary with D+Z=TAB combo |
| TAP (2) | Gallium | Tertiary layout |
| ALT (10) | QWERTY | Quaternary layout |

### Switching Layouts

1. Hold any utility layer key (NAV, NUM, SYM, etc.)
2. Double-tap toggle keys:
   - Position 5: ALT (QWERTY)
   - Position 6: BASE (Colemak-DH)
   - Position 7: EXTRA (Nightingale)
   - Position 8: TAP (Gallium)

### Changing Default Layouts

Edit `miryoku/custom_config.h`:

```c
#define MIRYOKU_ALPHAS_COLEMAKDH   // BASE layer
#define MIRYOKU_EXTRA_NIGHTINGALE  // EXTRA layer
#define MIRYOKU_TAP_GALLIUM        // TAP layer
#define MIRYOKU_ALT_QWERTY         // ALT layer
```

Available layouts: `COLEMAKDH`, `QWERTY`, `DVORAK`, `WORKMAN`, `NIGHT`, `NIGHTINGALE`, `GALLIUM`, `GRAPHITE`, etc.

## Trackball Usage Tips

### Basic Usage
- **Cursor Movement**: Roll the trackball in any direction
- **Left Click**: Press mouse button 1 (typically mapped to thumb key)
- **Right Click**: Press mouse button 2
- **Middle Click**: Press mouse button 3
- **Scrolling**: May require holding a scroll modifier key

### Calibration

If cursor movement feels off:

1. **Adjust CPI** (sensitivity) in `config/roBa_R.conf`
2. **Check orientation** - Sensor rotation is set to 180°
3. **Test axis inversion** if cursor moves backwards:
   ```conf
   CONFIG_PMW3610_INVERT_X=y
   CONFIG_PMW3610_INVERT_Y=y
   ```

### Performance Tuning

For better performance:
```conf
CONFIG_PMW3610_POLLING_RATE_SW=125    # Try 125, 250, or 500
CONFIG_PMW3610_SMART_ALGORITHM=y       # Enables smart movement
CONFIG_PMW3610_MOVEMENT_THRESHOLD=0    # Minimum movement to register
```

## Troubleshooting

### Trackball Not Working

1. **Check shield files are installed** in `boards/shields/roBa/`
2. **Verify SPI is enabled** (should be in shield overlay files)
3. **Flash settings_reset** to clear any corrupt configuration
4. **Check sensor connection** - hardware issue if still not working

### Automouse Not Activating

1. **Verify build includes automouse configs** from `config/roBa_R.conf`
2. **Check MOUSE layer is defined** (Miryoku includes this by default)
3. **Try manual MOUSE layer access** - Hold SYM thumb key
4. **Increase timeout** if it's deactivating too quickly

### Cursor Jumping or Erratic

1. **Clean trackball and sensor** - dust can cause issues
2. **Lower CPI** for more control
3. **Adjust movement threshold**:
   ```conf
   CONFIG_PMW3610_MOVEMENT_THRESHOLD=5
   ```

### Bluetooth Connection Issues

**Reset Bluetooth:**
1. Flash `settings_reset` firmware to both halves
2. Re-flash regular firmware
3. Re-pair with your device
4. Delete old pairings from host device

**Switch between paired devices:**
- Access BUTTON layer (usually double-tap button layer key)
- Select BT profile 0-4 for different devices

## Additional Resources

- **roBa Original Repository**: https://github.com/kumamuk-git/zmk-config-roBa
- **Miryoku Documentation**: https://github.com/manna-harbour/miryoku
- **ZMK Mouse/Pointing**: https://zmk.dev/docs/features/pointing
- **ZMK Automouse**: https://zmk.dev/docs/features/automouse
- **PMW3610 Sensor**: https://zmk.dev/docs/hardware/pointing/pmw3610

## Manual Shield Installation (Optional)

If you want to work with the shield files locally, you can manually add them:

```bash
# Create shields directory structure
mkdir -p boards/shields/roBa

# Clone the original roBa repository (temporary)
git clone https://github.com/kumamuk-git/zmk-config-roBa.git /tmp/roBa-temp

# Copy shield files
cp -r /tmp/roBa-temp/boards/shields/roBa/* boards/shields/roBa/

# Clean up
rm -rf /tmp/roBa-temp
```

**Note:** This is optional. GitHub Actions builds automatically fetch the shields.
