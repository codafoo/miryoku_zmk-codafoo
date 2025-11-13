#!/usr/bin/env python3
"""
Quick Reference Generator for ZMK Miryoku Keymaps
Generates text-based keyboard layout visualizations for quick reference
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Any


class QuickReferenceGenerator:
    def __init__(self, data_file: str):
        with open(data_file, 'r') as f:
            self.data = json.load(f)

    def format_key(self, key: str, width: int = 8) -> str:
        """Format a key label to fit in a fixed width"""
        if not key or key == '':
            return '·'.center(width)

        # Shorten common labels
        shortcuts = {
            'LSHFT': 'LShft', 'RSHFT': 'RShft',
            'LCTRL': 'LCtrl', 'RCTRL': 'RCtrl',
            'LALT': 'LAlt', 'RALT': 'RAlt',
            'LGUI': 'LGUI', 'RGUI': 'RGUI',
            'BSPC': 'Bksp', 'SPC': 'Space',
            'ENT': 'Enter', 'ESC': 'Esc',
            'DEL': 'Del', 'CAPS': 'Caps',
            'PSCR': 'PrtSc', 'SLCK': 'ScrLk',
            'PAUSE': 'Pause', 'INS': 'Ins',
            'PGUP': 'PgUp', 'PGDN': 'PgDn',
            'LEFT': '←', 'RIGHT': '→',
            'UP': '↑', 'DOWN': '↓',
        }

        display_key = shortcuts.get(key, key)

        # Truncate if too long
        if len(display_key) > width:
            display_key = display_key[:width-1] + '…'

        return display_key.center(width)

    def generate_layer_text(self, keyboard_name: str, layer_name: str) -> str:
        """Generate text visualization for a specific layer"""
        keyboard = next((kb for kb in self.data['keyboards'] if kb['name'] == keyboard_name), None)
        if not keyboard:
            return f"Keyboard '{keyboard_name}' not found"

        layer = next((l for l in keyboard['layers'] if l['id'] == layer_name), None)
        if not layer:
            return f"Layer '{layer_name}' not found"

        keys = layer['keys']
        layout = layer.get('layout', 'Unknown')

        output = []
        output.append("═" * 70)
        output.append(f" {keyboard_name.upper()} - {layer['name']} Layer ({layout})")
        output.append("═" * 70)
        output.append("")

        # Left hand rows
        left_top = [self.format_key(keys[i]) for i in range(5)]
        left_mid = [self.format_key(keys[i]) for i in range(10, 15)]
        left_bot = [self.format_key(keys[i]) for i in range(20, 25)]

        # Right hand rows
        right_top = [self.format_key(keys[i]) for i in range(5, 10)]
        right_mid = [self.format_key(keys[i]) for i in range(15, 20)]
        right_bot = [self.format_key(keys[i]) for i in range(25, 30)]

        # Top row
        output.append("  " + " ".join(left_top) + "    " + " ".join(right_top))
        # Middle row
        output.append("  " + " ".join(left_mid) + "    " + " ".join(right_mid))
        # Bottom row
        output.append("  " + " ".join(left_bot) + "    " + " ".join(right_bot))

        # Thumb cluster
        output.append("")
        thumbs = [self.format_key(keys[i]) for i in range(32, 38) if i < len(keys)]
        if thumbs:
            thumb_left = thumbs[:3] if len(thumbs) >= 3 else thumbs
            thumb_right = thumbs[3:6] if len(thumbs) >= 6 else []
            output.append("      " + " ".join(thumb_left) + "    " + " ".join(thumb_right))

        output.append("")
        return "\n".join(output)

    def generate_keyboard_reference(self, keyboard_name: str, layers: List[str] = None) -> str:
        """Generate complete reference for a keyboard"""
        if layers is None:
            layers = ['BASE', 'NAV', 'NUM', 'SYM', 'FUN', 'MOUSE', 'MEDIA']

        output = []
        output.append("\n" + "█" * 70)
        output.append(f"  ZMK MIRYOKU KEYBOARD REFERENCE: {keyboard_name.upper()}")
        output.append("█" * 70 + "\n")

        for layer_name in layers:
            output.append(self.generate_layer_text(keyboard_name, layer_name))
            output.append("")

        return "\n".join(output)

    def generate_layout_comparison(self, keyboard_name: str) -> str:
        """Generate comparison of all alpha layouts"""
        keyboard = next((kb for kb in self.data['keyboards'] if kb['name'] == keyboard_name), None)
        if not keyboard:
            return f"Keyboard '{keyboard_name}' not found"

        output = []
        output.append("\n" + "█" * 70)
        output.append(f"  ALPHA LAYOUT COMPARISON: {keyboard_name.upper()}")
        output.append("█" * 70 + "\n")

        alpha_layers = ['BASE', 'EXTRA', 'TAP', 'ALT']

        for layer_name in alpha_layers:
            layer = next((l for l in keyboard['layers'] if l['id'] == layer_name), None)
            if not layer:
                continue

            keys = layer['keys']
            layout = layer.get('layout', 'Unknown')

            output.append(f"╔═══ {layer['name']} ({layout}) " + "═" * (58 - len(layer['name']) - len(layout)))
            output.append("║")

            # Show only alpha keys (30 keys)
            left_top = [self.format_key(keys[i], 6) for i in range(5)]
            left_mid = [self.format_key(keys[i], 6) for i in range(10, 15)]
            left_bot = [self.format_key(keys[i], 6) for i in range(20, 25)]

            right_top = [self.format_key(keys[i], 6) for i in range(5, 10)]
            right_mid = [self.format_key(keys[i], 6) for i in range(15, 20)]
            right_bot = [self.format_key(keys[i], 6) for i in range(25, 30)]

            output.append("║  " + " ".join(left_top) + "    " + " ".join(right_top))
            output.append("║  " + " ".join(left_mid) + "    " + " ".join(right_mid))
            output.append("║  " + " ".join(left_bot) + "    " + " ".join(right_bot))
            output.append("║")

        output.append("╚" + "═" * 68)
        output.append("")

        return "\n".join(output)

    def list_keyboards(self) -> str:
        """List all available keyboards"""
        output = []
        output.append("\n" + "═" * 70)
        output.append("  AVAILABLE KEYBOARDS")
        output.append("═" * 70 + "\n")

        for kb in sorted(self.data['keyboards'], key=lambda x: x['name']):
            config = kb.get('custom_config', {})
            base = config.get('base', 'Unknown')
            output.append(f"  • {kb['name']:<25} ({kb['key_count']:>2} keys, {base})")

        output.append("")
        output.append(f"Total: {len(self.data['keyboards'])} keyboards")
        output.append("")

        return "\n".join(output)


def main():
    """Main entry point"""
    repo_root = Path(__file__).parent.parent
    data_file = Path(__file__).parent / "keymap_data.json"

    if not data_file.exists():
        print("Error: keymap_data.json not found!")
        print("Please run keymap_parser.py first:")
        print("  python3 keymap_parser.py")
        sys.exit(1)

    generator = QuickReferenceGenerator(str(data_file))

    if len(sys.argv) == 1:
        print("\n📚 ZMK Miryoku Quick Reference Generator")
        print("=" * 50)
        print("\nUsage:")
        print("  python3 keymap_quick_ref.py list")
        print("  python3 keymap_quick_ref.py <keyboard>")
        print("  python3 keymap_quick_ref.py <keyboard> <layer>")
        print("  python3 keymap_quick_ref.py <keyboard> compare")
        print("\nExamples:")
        print("  python3 keymap_quick_ref.py list")
        print("  python3 keymap_quick_ref.py corne")
        print("  python3 keymap_quick_ref.py corne BASE")
        print("  python3 keymap_quick_ref.py corne compare")
        print("")
        sys.exit(0)

    command = sys.argv[1]

    if command == "list":
        print(generator.list_keyboards())
    elif len(sys.argv) == 2:
        # Full keyboard reference
        print(generator.generate_keyboard_reference(command))
    elif len(sys.argv) == 3:
        if sys.argv[2] == "compare":
            # Layout comparison
            print(generator.generate_layout_comparison(command))
        else:
            # Single layer
            print(generator.generate_layer_text(command, sys.argv[2]))


if __name__ == "__main__":
    main()
