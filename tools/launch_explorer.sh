#!/bin/bash
# Launcher script for ZMK Miryoku Keymap Explorer

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

echo "🎹 ZMK Miryoku Keymap Explorer Launcher"
echo "========================================"
echo

# Check if keymap_data.json exists
if [ ! -f "keymap_data.json" ]; then
    echo "⚠️  keymap_data.json not found!"
    echo "📊 Generating keymap data..."
    echo

    if [ -f "keymap_parser.py" ]; then
        python3 keymap_parser.py
        if [ $? -ne 0 ]; then
            echo "❌ Error running parser!"
            exit 1
        fi
        echo
    else
        echo "❌ keymap_parser.py not found!"
        exit 1
    fi
fi

echo "✅ Keymap data ready!"
echo

# Check if Python is available
if command -v python3 &> /dev/null; then
    echo "🚀 Starting web server on http://localhost:8000"
    echo "📱 Open your browser and navigate to:"
    echo "   http://localhost:8000/keymap_explorer.html"
    echo
    echo "Press Ctrl+C to stop the server"
    echo
    python3 -m http.server 8000
else
    echo "⚠️  Python3 not found. Opening HTML file directly..."
    echo

    # Try to open in default browser
    if command -v xdg-open &> /dev/null; then
        xdg-open keymap_explorer.html
    elif command -v open &> /dev/null; then
        open keymap_explorer.html
    elif command -v start &> /dev/null; then
        start keymap_explorer.html
    else
        echo "Please open keymap_explorer.html manually in your browser"
    fi
fi
