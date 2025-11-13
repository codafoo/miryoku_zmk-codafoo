#!/usr/bin/env python3
"""
ZMK Miryoku Keymap Parser
Extracts keyboard configurations and layer mappings from ZMK config files
"""

import json
import os
import re
from pathlib import Path
from typing import Dict, List, Any

# Miryoku layer definitions
MIRYOKU_LAYERS = [
    {"id": "BASE", "name": "Base", "index": 0},
    {"id": "EXTRA", "name": "Extra", "index": 1},
    {"id": "TAP", "name": "Tap", "index": 2},
    {"id": "BUTTON", "name": "Button", "index": 3},
    {"id": "NAV", "name": "Nav", "index": 4},
    {"id": "MOUSE", "name": "Mouse", "index": 5},
    {"id": "MEDIA", "name": "Media", "index": 6},
    {"id": "NUM", "name": "Num", "index": 7},
    {"id": "SYM", "name": "Sym", "index": 8},
    {"id": "FUN", "name": "Fun", "index": 9},
    {"id": "ALT", "name": "Alt", "index": 10},
]

# Available alpha layouts
ALPHA_LAYOUTS = [
    "AZERTY", "BEAKL15", "COLEMAK", "COLEMAKDH", "COLEMAKDHK",
    "DVORAK", "HALMAK", "WORKMAN", "QWERTY", "QWERTZ",
    "NIGHT", "NIGHTINGALE", "GALLIUM", "GRAPHITE"
]

class KeymapParser:
    def __init__(self, repo_root: str):
        self.repo_root = Path(repo_root)
        self.config_dir = self.repo_root / "config"
        self.miryoku_dir = self.repo_root / "miryoku"
        self.mapping_dir = self.miryoku_dir / "mapping"

    def find_all_keyboards(self) -> List[str]:
        """Find all keyboard keymap files"""
        keyboards = []
        for keymap_file in self.config_dir.glob("*.keymap"):
            keyboards.append(keymap_file.stem)
        return sorted(keyboards)

    def parse_keymap_file(self, keyboard: str) -> Dict[str, Any]:
        """Parse a single keyboard's keymap file"""
        keymap_path = self.config_dir / f"{keyboard}.keymap"

        if not keymap_path.exists():
            return None

        with open(keymap_path, 'r') as f:
            content = f.read()

        # Extract mapping include
        mapping_match = re.search(r'#include\s+"[^"]*mapping/(\d+)/([^"]+)"', content)

        keyboard_data = {
            "name": keyboard,
            "keymap_path": str(keymap_path.relative_to(self.repo_root)),
        }

        if mapping_match:
            key_count = mapping_match.group(1)
            mapping_file = mapping_match.group(2)
            keyboard_data["key_count"] = int(key_count)
            keyboard_data["mapping_file"] = mapping_file
            keyboard_data["mapping_name"] = Path(mapping_file).stem

            # Parse the mapping file
            mapping_path = self.mapping_dir / key_count / mapping_file
            if mapping_path.exists():
                keyboard_data["physical_layout"] = self.parse_mapping_file(mapping_path)

        return keyboard_data

    def parse_mapping_file(self, mapping_path: Path) -> Dict[str, Any]:
        """Parse a Miryoku mapping file to extract physical layout"""
        with open(mapping_path, 'r') as f:
            content = f.read()

        # Extract the mapping macro definition
        macro_match = re.search(
            r'#define\s+MIRYOKU_LAYOUTMAPPING_\w+\([^)]+\)\s*\\\n((?:.*\\\n)*.*?)(?:\n\n|#define)',
            content,
            re.MULTILINE | re.DOTALL
        )

        layout_data = {
            "raw_mapping": "",
            "positions": []
        }

        if macro_match:
            mapping_lines = macro_match.group(1)
            layout_data["raw_mapping"] = mapping_lines

            # Parse thumb combo positions
            thumb_combos_left = re.search(r'MIRYOKU_KLUDGE_THUMBCOMBOS_LEFT\s+(\d+)\s+(\d+)', content)
            thumb_combos_right = re.search(r'MIRYOKU_KLUDGE_THUMBCOMBOS_RIGHT\s+(\d+)\s+(\d+)', content)

            if thumb_combos_left:
                layout_data["thumb_combos_left"] = [int(thumb_combos_left.group(1)), int(thumb_combos_left.group(2))]
            if thumb_combos_right:
                layout_data["thumb_combos_right"] = [int(thumb_combos_right.group(1)), int(thumb_combos_right.group(2))]

            # Parse top row combos
            toprow_left = re.search(r'MIRYOKU_KLUDGE_TOPROWCOMBOS_LEFTPINKIE\s+(\d+)\s+(\d+)', content)
            toprow_right = re.search(r'MIRYOKU_KLUDGE_TOPROWCOMBOS_RIGHTPINKIE\s+(\d+)\s+(\d+)', content)

            if toprow_left:
                layout_data["toprow_combos_left"] = [int(toprow_left.group(1)), int(toprow_left.group(2))]
            if toprow_right:
                layout_data["toprow_combos_right"] = [int(toprow_right.group(1)), int(toprow_right.group(2))]

        return layout_data

    def parse_custom_config(self) -> Dict[str, Any]:
        """Parse custom_config.h to get configured layouts"""
        config_path = self.miryoku_dir / "custom_config.h"

        config_data = {
            "base": "COLEMAKDH",
            "extra": None,
            "tap": None,
            "alt": None
        }

        if not config_path.exists():
            return config_data

        with open(config_path, 'r') as f:
            content = f.read()

        # Extract alpha layout definitions
        for layer_type in ["ALPHAS", "EXTRA", "TAP", "ALT"]:
            pattern = rf'#define\s+MIRYOKU_{layer_type}_(\w+)'
            match = re.search(pattern, content)
            if match:
                layout = match.group(1)
                if layer_type == "ALPHAS":
                    config_data["base"] = layout
                else:
                    config_data[layer_type.lower()] = layout

        return config_data

    def get_layer_keys(self, layer_name: str, layout: str = "COLEMAKDH") -> List[str]:
        """Get the keys for a specific layer and layout"""
        # Standard Miryoku layout definitions (simplified)
        # In reality, these would be parsed from the layer_alternatives.h file

        base_layouts = {
            "COLEMAKDH": [
                "Q", "W", "F", "P", "B",                "J", "L", "U", "Y", "'",
                "A", "R", "S", "T", "G",                "M", "N", "E", "I", "O",
                "Z", "X", "C", "D", "V",                "K", "H", ",", ".", "/",
                "",  "",  "ESC", "SPC", "TAB",          "ENT", "BSPC", "DEL", "", ""
            ],
            "QWERTY": [
                "Q", "W", "E", "R", "T",                "Y", "U", "I", "O", "P",
                "A", "S", "D", "F", "G",                "H", "J", "K", "L", ";",
                "Z", "X", "C", "V", "B",                "N", "M", ",", ".", "/",
                "",  "",  "ESC", "SPC", "TAB",          "ENT", "BSPC", "DEL", "", ""
            ],
            "NIGHTINGALE": [
                "K", "L", "D", "W", "X",                "Q", "P", "U", "O", "Y",
                "S", "T", "N", "M", "B",                ".",  "A", "E", "I", "H",
                "F", "R", "C", "G", "V",                "Z", "J", "'", ",", "/",
                "",  "",  "ESC", "SPC", "TAB",          "ENT", "BSPC", "DEL", "", ""
            ],
            "GALLIUM": [
                "B", "L", "D", "C", "V",                "J", "Y", "O", "U", ",",
                "N", "R", "T", "S", "G",                "P", "H", "A", "E", "I",
                "X", "Q", "M", "W", "Z",                "K", "F", "'", ".", "/",
                "",  "",  "ESC", "SPC", "TAB",          "ENT", "BSPC", "DEL", "", ""
            ],
            "GRAPHITE": [
                "B", "L", "D", "W", "Z",                "'", "F", "O", "U", "J",
                "N", "R", "T", "S", "G",                "Y", "H", "A", "E", "I",
                "Q", "X", "M", "C", "V",                "K", "P", ".", ",", "/",
                "",  "",  "ESC", "SPC", "TAB",          "ENT", "BSPC", "DEL", "", ""
            ],
        }

        utility_layers = {
            "NAV": [
                "BOOT", "", "", "", "",                  "REDO", "PASTE", "COPY", "CUT", "UNDO",
                "LGUI", "LALT", "LCTRL", "LSHFT", "",    "CAPS", "LEFT", "DOWN", "UP", "RIGHT",
                "", "", "", "", "",                       "INS", "HOME", "PGDN", "PGUP", "END",
                "",  "",  "", "", "",                     "ENT", "BSPC", "DEL", "", ""
            ],
            "MOUSE": [
                "BOOT", "", "", "", "",                  "REDO", "PASTE", "COPY", "CUT", "UNDO",
                "LGUI", "LALT", "LCTRL", "LSHFT", "",    "", "MLEFT", "MDOWN", "MUP", "MRIGHT",
                "", "", "", "", "",                       "", "MWLEFT", "MWDOWN", "MWUP", "MWRIGHT",
                "",  "",  "", "", "",                     "BTN1", "BTN3", "BTN2", "", ""
            ],
            "MEDIA": [
                "BOOT", "", "", "", "",                  "RGB_TOG", "RGB_INCR", "RGB_HUE+", "RGB_SAT+", "RGB_BRI+",
                "LGUI", "LALT", "LCTRL", "LSHFT", "",    "", "PREV", "VOL-", "VOL+", "NEXT",
                "", "", "", "", "",                       "BT_SEL", "BT_PRV", "BT_NXT", "BT_CLR", "BT_CLR_ALL",
                "",  "",  "", "", "",                     "STOP", "PLAY", "MUTE", "", ""
            ],
            "NUM": [
                "[", "7", "8", "9", "]",                 "", "", "", "", "BOOT",
                ";", "4", "5", "6", "=",                 "", "RSHFT", "RCTRL", "RALT", "RGUI",
                "`", "1", "2", "3", "\\",                "", "", "", "", "",
                "",  "",  ".", "0", "-",                 "", "", "", "", ""
            ],
            "SYM": [
                "{", "&", "*", "(", "}",                 "", "", "", "", "BOOT",
                ":", "$", "%", "^", "+",                 "", "RSHFT", "RCTRL", "RALT", "RGUI",
                "~", "!", "@", "#", "|",                 "", "", "", "", "",
                "",  "",  "(", ")", "_",                 "", "", "", "", ""
            ],
            "FUN": [
                "F12", "F7", "F8", "F9", "PSCR",         "", "", "", "", "BOOT",
                "F11", "F4", "F5", "F6", "SLCK",         "", "RSHFT", "RCTRL", "RALT", "RGUI",
                "F10", "F1", "F2", "F3", "PAUSE",        "", "", "", "", "",
                "",  "",  "APP", "SPC", "TAB",           "", "", "", "", ""
            ],
            "BUTTON": [
                "UNDO", "CUT", "COPY", "PASTE", "REDO",  "REDO", "PASTE", "COPY", "CUT", "UNDO",
                "LGUI", "LALT", "LCTRL", "LSHFT", "",    "", "RSHFT", "RCTRL", "RALT", "RGUI",
                "UNDO", "CUT", "COPY", "PASTE", "REDO",  "REDO", "PASTE", "COPY", "CUT", "UNDO",
                "",  "",  "BTN2", "BTN1", "BTN3",        "BTN3", "BTN1", "BTN2", "", ""
            ],
        }

        if layer_name in ["BASE", "EXTRA", "TAP", "ALT"]:
            return base_layouts.get(layout, base_layouts["COLEMAKDH"])
        else:
            return utility_layers.get(layer_name, [])

    def generate_keyboard_data(self, keyboard: str) -> Dict[str, Any]:
        """Generate complete data for a keyboard"""
        keyboard_data = self.parse_keymap_file(keyboard)
        if not keyboard_data:
            return None

        custom_config = self.parse_custom_config()

        # Add layer information
        layers = []
        for layer in MIRYOKU_LAYERS:
            layer_info = layer.copy()

            # Determine which layout to use for this layer
            if layer["id"] == "BASE":
                layout = custom_config["base"]
            elif layer["id"] == "EXTRA":
                layout = custom_config["extra"] or custom_config["base"]
            elif layer["id"] == "TAP":
                layout = custom_config["tap"] or custom_config["base"]
            elif layer["id"] == "ALT":
                layout = custom_config["alt"] or "QWERTY"
            else:
                layout = None

            layer_info["layout"] = layout
            layer_info["keys"] = self.get_layer_keys(layer["id"], layout)
            layers.append(layer_info)

        keyboard_data["layers"] = layers
        keyboard_data["custom_config"] = custom_config
        keyboard_data["available_layouts"] = ALPHA_LAYOUTS

        return keyboard_data

    def generate_all_data(self) -> Dict[str, Any]:
        """Generate data for all keyboards"""
        keyboards = self.find_all_keyboards()

        all_data = {
            "keyboards": [],
            "miryoku_layers": MIRYOKU_LAYERS,
            "available_layouts": ALPHA_LAYOUTS,
            "custom_config": self.parse_custom_config()
        }

        for keyboard in keyboards:
            kb_data = self.generate_keyboard_data(keyboard)
            if kb_data:
                all_data["keyboards"].append(kb_data)

        return all_data


def main():
    """Main entry point"""
    repo_root = Path(__file__).parent.parent
    parser = KeymapParser(str(repo_root))

    # Generate data for all keyboards
    data = parser.generate_all_data()

    # Write to JSON file
    output_path = repo_root / "tools" / "keymap_data.json"
    with open(output_path, 'w') as f:
        json.dump(data, f, indent=2)

    print(f"Generated keymap data for {len(data['keyboards'])} keyboards")
    print(f"Output: {output_path}")

    return data


if __name__ == "__main__":
    main()
