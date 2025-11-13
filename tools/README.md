# ZMK Miryoku Keymap Explorer

An interactive web application for exploring and visualizing your ZMK Miryoku keyboard configurations.

## Features

### 🎹 Visual Keyboard Layouts
- Interactive 3D keyboard visualization
- Color-coded keys by type (modifiers, layer keys, special functions)
- Click any key to see detailed information
- Real-time layer switching

### 🔍 Keyboard Management
- Search through all 116+ keyboard configurations
- Quick keyboard switching with dropdown selector
- View keyboard specifications (key count, layouts, mapping)

### 📊 Layer Exploration
- View all 11 Miryoku layers:
  - BASE, EXTRA, TAP, ALT (alpha layers)
  - NAV, MOUSE, MEDIA, NUM, SYM, FUN, BUTTON (utility layers)
- Switch between layers instantly
- See which alpha layout is assigned to each layer (Colemak-DH, QWERTY, Nightingale, Gallium, etc.)

### 🛠️ Powerful Tools

#### Compare Layers
- Side-by-side comparison of BASE, EXTRA, TAP, and ALT layers
- See differences between your configured alpha layouts
- Perfect for understanding your multi-layout setup

#### Export Configuration
- Download complete keyboard configuration as JSON
- Includes all layers, keys, and settings
- Easy backup and sharing

#### View Statistics
- Key usage statistics per layer
- Count of modifiers, layer keys, and special functions
- Total keys vs. used keys

#### Print Layout
- Print-friendly layout for reference
- Great for creating physical cheat sheets

### 🎨 Beautiful Dark Theme
- Easy on the eyes for long exploration sessions
- Color-coded key types for instant recognition
- Responsive design works on desktop and mobile

## Quick Start

### 1. Generate Keymap Data

First, run the parser to extract data from your ZMK configuration:

```bash
cd tools
python3 keymap_parser.py
```

This will generate `keymap_data.json` with data from all your keyboards.

### 2. Open the Explorer

Simply open `keymap_explorer.html` in your web browser:

```bash
# Using default browser
open keymap_explorer.html

# Or use a specific browser
google-chrome keymap_explorer.html
firefox keymap_explorer.html

# Or start a simple web server
python3 -m http.server 8000
# Then visit: http://localhost:8000/keymap_explorer.html
```

### 3. Start Exploring!

1. Use the search box or dropdown to select a keyboard
2. Click layer buttons to switch between layers
3. Click any key to see details
4. Use the tools to compare, export, and analyze

## Files

- `keymap_parser.py` - Python script that extracts keymap data from ZMK configs
- `keymap_explorer.html` - Self-contained web application (HTML + CSS + JavaScript)
- `keymap_quick_ref.py` - Command-line tool for generating text-based layout references
- `launch_explorer.sh` - Convenient launcher script for the web app
- `keymap_data.json` - Generated data file (created by parser)
- `README.md` - This file

## Command-Line Quick Reference

For quick terminal-based layout viewing, use the `keymap_quick_ref.py` tool:

```bash
# List all keyboards
python3 keymap_quick_ref.py list

# View all layers for a keyboard
python3 keymap_quick_ref.py corne

# View specific layer
python3 keymap_quick_ref.py corne BASE

# Compare alpha layouts side-by-side
python3 keymap_quick_ref.py corne compare
```

This generates clean ASCII art visualizations perfect for:
- Quick reference during typing
- Creating documentation
- Terminal-based exploration
- Piping to text files for reference sheets

Example output:
```
══════════════════════════════════════════════════════════════════════
 CORNE - Base Layer (COLEMAKDH)
══════════════════════════════════════════════════════════════════════

     Q        W        F        P        B           J        L        U        Y        '
     A        R        S        T        G           M        N        E        I        O
     Z        X        C        D        V           K        H        ,        .        /

        Esc     Space     TAB        Enter     Bksp     Del
```

## Usage Tips

### Understanding Key Colors

- **Gray (standard)**: Regular letter/number keys
- **Pink border**: Modifiers (Shift, Ctrl, Alt, GUI)
- **Cyan border**: Layer toggle/hold keys
- **Green border**: Special functions (Bluetooth, RGB, Boot, Mouse)

### Keyboard Selection

The dropdown shows:
- Keyboard name
- Total key count
- Example: "corne (42 keys)"

### Layer Information

**Alpha Layers** (typing layouts):
- BASE - Your primary layout
- EXTRA - Secondary layout (toggle-able)
- TAP - Tertiary layout
- ALT - Quaternary layout (often QWERTY)

**Utility Layers** (functions):
- NAV - Navigation (arrows, home/end, etc.)
- MOUSE - Mouse control and buttons
- MEDIA - Media controls and RGB
- NUM - Numbers and symbols
- SYM - Symbols and special characters
- FUN - Function keys (F1-F12)
- BUTTON - Button/macro layer

### Comparison Mode

Click "Compare Layers" to see all four alpha layouts side-by-side. This helps you:
- Remember which letters are where across different layouts
- Plan layer switches during typing
- Understand your multi-layout strategy

## Customization

### Modifying the Parser

Edit `keymap_parser.py` to:
- Add more layout definitions (see `base_layouts` and `utility_layers` dictionaries)
- Parse additional configuration options
- Extract custom combo information
- Add support for non-Miryoku keymaps

### Customizing the UI

Edit `keymap_explorer.html` to:
- Change color scheme (CSS variables in `:root`)
- Adjust key sizes and spacing
- Add new tools and features
- Modify layout visualization

## Troubleshooting

### "Error loading keymap data"

Make sure you've run the parser first:
```bash
python3 keymap_parser.py
```

And that `keymap_data.json` exists in the same directory as the HTML file.

### Keys appear empty

This is normal for positions that aren't used on your specific keyboard. The Miryoku standard layout (36 keys) is mapped to each physical keyboard, and unused positions show as empty.

### Browser compatibility

The explorer works best in modern browsers:
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

## Future Enhancements

Possible features to add:
- [ ] Visual combo indicators
- [ ] Home row mod highlighting
- [ ] Tap-hold behavior annotations
- [ ] Custom keymap editing
- [ ] Layout generator
- [ ] Diff view between keyboards
- [ ] Save favorite keyboards
- [ ] Dark/light theme toggle
- [ ] Keyboard layout JSON import/export
- [ ] QMK compatibility

## Contributing

Found a bug or want to add a feature? The tool is part of your ZMK configuration repository. Feel free to modify and enhance it!

## Credits

Built for the ZMK Miryoku keyboard configuration system.

- ZMK Firmware: https://zmk.dev
- Miryoku: https://github.com/manna-harbour/miryoku

## License

Same as your ZMK configuration (MIT).
