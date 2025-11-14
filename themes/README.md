# Keymap Visualization Themes

This directory contains theme configurations for keymap-drawer visualizations.

## Available Themes

### Light (Default)
The standard light mode theme. No configuration file needed.

**Usage:**
```bash
task keymap KEYBOARD=brain
```

### Dark
A dark mode theme with clean, readable colors.

**Usage:**
```bash
task keymap KEYBOARD=brain THEME=dark
```

### Auto
Automatically adapts to your system's light/dark mode preference.

**Usage:**
```bash
task keymap KEYBOARD=brain THEME=auto
```

### Nord
A beautiful dark theme using the [Nord color palette](https://www.nordtheme.com/).

**Colors:**
- Background: Polar Night (#2e3440)
- Text: Snow Storm (#eceff4)
- Highlights: Frost & Aurora colors

**Usage:**
```bash
task keymap KEYBOARD=brain THEME=nord
```

### Catppuccin Mocha
A soothing dark theme using the [Catppuccin Mocha palette](https://github.com/catppuccin/catppuccin).

**Colors:**
- Background: Base (#1e1e2e)
- Text: Text (#cdd6f4)
- Highlights: Sky, Blue, Mauve, Green

**Usage:**
```bash
task keymap KEYBOARD=brain THEME=catppuccin-mocha
```

## Creating Custom Themes

To create your own theme:

1. Copy an existing theme file as a starting point:
   ```bash
   cp themes/dark.yaml themes/my-theme.yaml
   ```

2. Edit the CSS in `svg_extra_style` to customize colors:
   ```yaml
   draw_config:
     dark_mode: true  # or false, or "auto"
     svg_extra_style: |
       svg.keymap {
         background-color: #your-bg-color;
       }
       text {
         fill: #your-text-color;
       }
       /* Add more custom styles */
   ```

3. Use your theme:
   ```bash
   task keymap KEYBOARD=brain THEME=my-theme
   ```

## CSS Classes for Styling

Common CSS classes you can target:

- `.keymap` - The main SVG container
- `rect.key` - Key rectangles
- `rect.combo` - Combo key rectangles
- `text` - All text elements
- `.hold text` - Hold-tap hold legends
- `.tap text` - Hold-tap tap legends
- `.shifted text` - Shifted key legends
- `.layer-activator` - Layer activation keys

## Example: Custom Purple Theme

```yaml
draw_config:
  dark_mode: true
  svg_extra_style: |
    svg.keymap {
      background-color: #1a1625;
    }
    text {
      fill: #e4d4ff;
    }
    rect.key {
      fill: #2d2640;
      stroke: #483d63;
    }
    .hold text {
      fill: #b794f6;
      font-weight: 600;
    }
    .layer-activator {
      fill: #e879f9;
      font-weight: 700;
    }
```

## Resources

- [keymap-drawer Configuration Docs](https://github.com/caksoylar/keymap-drawer/blob/main/CONFIGURATION.md)
- [Nord Color Palette](https://www.nordtheme.com/)
- [Catppuccin Color Palette](https://github.com/catppuccin/catppuccin)
