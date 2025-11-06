# Codafoo Miryoku ZMK Build Guide

This repository contains a customized Miryoku ZMK configuration with support for multiple keyboard layouts that can be toggled on the fly.

## Table of Contents
- [Quick Start](#quick-start)
- [Building Firmware](#building-firmware)
- [Current Configuration](#current-configuration)
- [Configuring Layouts](#configuring-layouts)
- [Available Layouts](#available-layouts)
- [Custom Combos](#custom-combos)
- [Supported Keyboards](#supported-keyboards)
- [Local Development](#local-development)

## Quick Start

**Build all keyboards:**
```bash
gh workflow run build-all-keyboards.yml --ref main
```

**Build individual keyboards:**
```bash
gh workflow run build-corne.yml --ref main
gh workflow run build-urchin.yml --ref main
gh workflow run build-ergonaut_one_s.yml --ref main
gh workflow run build-brain.yml --ref brain
```

**Check build status:**
```bash
gh run list --limit 5
gh run watch  # Watch latest build
```

**Download artifacts:**
```bash
gh run download <run-id>
```

## Building Firmware

### Via GitHub Actions (Recommended)

1. **Trigger a build:**
   ```bash
   gh workflow run build-all-keyboards.yml --ref main
   ```

2. **Monitor the build:**
   ```bash
   gh run watch
   ```

3. **Download firmware:**
   - Go to: https://github.com/codafoo/miryoku_zmk-codafoo/actions
   - Click on your workflow run
   - Download artifacts from the bottom of the page
   - Each keyboard will have separate artifacts for left/right sides and settings_reset

4. **Flash firmware:**
   - Double-tap reset button on keyboard (enters bootloader mode)
   - Keyboard appears as USB drive
   - Drag and drop the `.uf2` file onto the drive
   - Keyboard automatically reboots with new firmware

### Build Outputs

Each build produces:
- **Left/Right firmware**: `zmk.uf2` - Main keyboard firmware
- **Settings reset**: `settings_reset` artifact - Clears Bluetooth pairings and settings

## Current Configuration

The current setup uses a **4-layer alpha system**:

| Layer | Layout      | Toggle Position | Description |
|-------|-------------|----------------|-------------|
| 0     | BASE        | Position 6     | Colemak-DH (primary) |
| 1     | EXTRA       | Position 7     | Nightingale (D+Z=TAB combo) |
| 2     | TAP         | Position 8     | Gallium |
| 10    | ALT         | Position 5     | QWERTY |

### Switching Between Layouts

1. Hold any layer key (NAV, NUM, SYM, FUN, MOUSE, MEDIA)
2. Double-tap one of the toggle keys in the top row:
   - `[5:ALT]` → Switch to QWERTY
   - `[6:BASE]` → Switch to Colemak-DH
   - `[7:EXTRA]` → Switch to Nightingale
   - `[8:TAP]` → Switch to Gallium
   - `[9:BOOT]` → Enter bootloader

All layers have:
- ✅ Home row mods (GUI/Alt/Ctrl/Shift)
- ✅ Layer-tap thumb keys
- ✅ Full Miryoku functionality

## Configuring Layouts

### Quick Configuration

Edit `miryoku/custom_config.h`:

```c
#define MIRYOKU_ALPHAS_COLEMAKDH   // BASE layer (0)
#define MIRYOKU_EXTRA_NIGHTINGALE  // EXTRA layer (1)
#define MIRYOKU_TAP_GALLIUM        // TAP layer (2)
#define MIRYOKU_ALT_QWERTY         // ALT layer (10)
```

### Example Configurations

**Night + Nightingale + Gallium + QWERTY:**
```c
#define MIRYOKU_ALPHAS_NIGHT
#define MIRYOKU_EXTRA_NIGHTINGALE
#define MIRYOKU_TAP_GALLIUM
#define MIRYOKU_ALT_QWERTY
```

**Graphite + Night + Gallium + Colemak-DH:**
```c
#define MIRYOKU_ALPHAS_GRAPHITE
#define MIRYOKU_EXTRA_NIGHT
#define MIRYOKU_TAP_GALLIUM
#define MIRYOKU_ALT_COLEMAKDH
```

**QWERTY + Colemak-DH + Dvorak + Workman:**
```c
#define MIRYOKU_ALPHAS_QWERTY
#define MIRYOKU_EXTRA_COLEMAKDH
#define MIRYOKU_TAP_DVORAK
#define MIRYOKU_ALT_WORKMAN
```

## Available Layouts

### Standard Layouts
- `COLEMAKDH` - Colemak-DH (default Miryoku)
- `QWERTY` - Standard QWERTY
- `QWERTZ` - German QWERTY variant
- `DVORAK` - Dvorak
- `WORKMAN` - Workman
- `AZERTY` - French AZERTY
- `HALMAK` - Halmak

### Alternative Layouts (Added)
- `NIGHT` - Night layout (optimized for 2-thumb keyboards)
  - Thumbs: ESC(Media), R(Nav), SPACE(Mouse)
- `NIGHTINGALE` - Night variant with TAB combo (D+Z)
- `GALLIUM` - Gallium layout
- `GRAPHITE` - Graphite layout

### Layout Resources
- **Night/Nightingale**: https://www.valorance.org/night/
- **Gallium**: Based on frequency-optimized layout
- **Graphite**: https://github.com/rdavison/graphite-layout
- **Standard Miryoku layouts**: https://github.com/manna-harbour/miryoku

## Custom Combos

Combos are defined in `miryoku/custom_combos.dtsi`.

### Current Combos

**TAB Combo (D+Z):**
- Active on: Layer 1 (Nightingale/EXTRA only)
- Timeout: 50ms
- Keys: Positions 23 + 24 (D and Z keys in Night/Nightingale)

### Adding Custom Combos

Edit `miryoku/custom_combos.dtsi`:

```c
/ {
    combos {
        compatible = "zmk,combos";

        // TAB on Nightingale
        combo_tab {
            timeout-ms = <50>;
            key-positions = <23 24>;  // D + Z
            bindings = <&kp TAB>;
            layers = <1>;             // EXTRA layer only
        };

        // Add your own combo
        combo_esc {
            timeout-ms = <50>;
            key-positions = <12 13>;  // Example: J + K
            bindings = <&kp ESC>;
            layers = <0 1 2 10>;      // All alpha layers
        };
    };
};
```

### Finding Key Positions

Key positions are numbered from 0-33 for a 34-key layout (like Urchin):
```
Left Hand:          Right Hand:
 0   1   2   3   4   5   6   7   8   9
10  11  12  13  14  15  16  17  18  19
20  21  22  23  24  25  26  27  28  29
        30  31  32  33
```

For 36-key layouts add 2 keys, for 42-key add 8 keys, etc.

## Supported Keyboards

| Keyboard | Controller Options | Display | Shield Names |
|----------|-------------------|---------|--------------|
| **Brain** | nice_nano_v2 | nice_view | `brain_left`, `brain_right` |
| **Corne** | nice_nano_v2 | nice_view | `corne_left`, `corne_right` |
| **Urchin** | nice_nano_v2 | nice_view | `urchin_left`, `urchin_right` |
| **Flake** | nice_nano_v2, seeed_xiao_ble | none | `flake_left`, `flake_right` |
| **Ergonaut One** | nice_nano_v2 | none | `ergonaut_one` |
| **Ergonaut One S** | nice_nano_v2 | none | `ergonaut_one_s` |
| **roBa** | seeeduino_xiao_ble | none | `roBa_L`, `roBa_R` (with PMW3610 trackball & automouse) |

### Adding a New Keyboard

1. **Create keymap file:** `config/mykeyboard.keymap`
   ```c
   #include "../miryoku/custom_config.h"
   #include "../miryoku/mapping/XX/keyboardname.h"
   #include "../miryoku/miryoku.dtsi"
   #include "../miryoku/custom_combos.dtsi"
   ```

2. **Create workflow:** `.github/workflows/build-mykeyboard.yml`
   ```yaml
   name: 'Build MyKeyboard'
   on: workflow_dispatch
   jobs:
     build:
       uses: ./.github/workflows/main.yml
       secrets: inherit
       with:
         board: '["nice_nano_v2"]'
         shield: '["mykeyboard_left","mykeyboard_right"]'
   ```

3. **Add to build-all-keyboards.yml** (optional)

## Local Development

### Prerequisites
- Docker (for containerized builds)
- Or: Native ZMK build environment

### Build with Docker

```bash
# Clone repository
git clone https://github.com/codafoo/miryoku_zmk-codafoo.git
cd miryoku_zmk-codafoo

# Build specific keyboard
docker run --rm -it -v $(pwd):/workspace \
  zmkfirmware/zmk-build-arm:stable \
  /bin/bash -c "
    cd /workspace
    west init -l miryoku/
    west update
    west build -s zmk/app -b nice_nano_v2 -- \
      -DSHIELD='corne_left nice_view_adapter nice_view' \
      -DZMK_CONFIG=/workspace/config
  "
```

### Test Changes Locally

1. Make changes to `miryoku/custom_config.h`
2. Build locally with Docker (see above)
3. Check `build/zephyr/zmk.uf2`
4. Test on hardware
5. Commit and push when satisfied

## Troubleshooting

### Keyboard Not Connecting

**Flash settings_reset:**
1. Download `settings_reset` firmware for your board
2. Flash to left side
3. Flash to right side
4. Flash your regular firmware back
5. Re-pair Bluetooth

### Wrong Layout Active

**Reset to BASE layer:**
1. Hold NAV or NUM layer key
2. Double-tap the BASE toggle (position 6)
3. Release all keys

### Build Fails

**Check common issues:**
- Syntax errors in `custom_config.h`
- Invalid layout names (check spelling)
- Missing includes in keymap files
- Review GitHub Actions logs for specific errors

### Combo Not Working

**Verify:**
- Correct key positions for your keyboard
- Active on correct layer (`layers = <1>` for EXTRA only)
- Timeout not too short (`timeout-ms = <50>` is recommended)
- Keys pressed simultaneously (within timeout)

## File Structure

```
miryoku_zmk-codafoo/
├── .github/workflows/
│   ├── build-all-keyboards.yml    # Build all keyboards
│   ├── build-brain.yml            # Individual builds
│   ├── build-corne.yml
│   └── ...
├── config/
│   ├── brain.keymap               # Per-keyboard configs
│   ├── corne.keymap
│   ├── flake.keymap
│   └── ...
├── miryoku/
│   ├── custom_config.h            # Layout configuration
│   ├── custom_combos.dtsi         # Combo definitions
│   ├── miryoku.dtsi               # Main Miryoku config
│   └── miryoku_babel/
│       ├── miryoku_layer_alternatives.h    # Layout definitions
│       ├── miryoku_layer_selection.h       # Layer mapping
│       └── miryoku_layer_list.h            # Layer enumeration
└── README.md
```

## ZMK Studio

All keyboards are configured with ZMK Studio enabled for real-time keymap editing:

- **Access Studio**: Connect keyboard via USB or Bluetooth
- **Studio URL**: https://zmk.studio/
- **Features**:
  - Live keymap editing without reflashing
  - Layer configuration
  - Behavior customization
  - No locking by default (CONFIG_ZMK_STUDIO_LOCKING=n)

**Note**: ZMK Studio is experimental. Stable firmware editing via GitHub Actions is still recommended.

## Resources

- **Miryoku Documentation**: https://github.com/manna-harbour/miryoku
- **ZMK Documentation**: https://zmk.dev/
- **ZMK Studio**: https://zmk.studio/
- **ZMK Discord**: https://zmk.dev/community/discord/invite
- **Miryoku Reference**: https://github.com/manna-harbour/miryoku/tree/master/docs/reference
- **roBa Setup Guide**: See `roBa_setup.md` for trackball keyboard with automouse

## Contributing

When making changes:
1. Test locally if possible
2. Create descriptive commit messages
3. Document layout changes in this file
4. Update `custom_config.h` comments

## License

Copyright 2021-2023 Manna Harbour
https://github.com/manna-harbour/miryoku

This configuration follows the Miryoku project's licensing.
