#!/bin/bash
# Build Ergonaut One S with ZMK Studio support using the snippet

BOARD="seeeduino_xiao_ble"  
SIDE="${1:-left}"

if [ "$SIDE" = "left" ]; then
    SHIELD="ergonaut_one_s_left"
else
    SHIELD="ergonaut_one_s_right"
fi

echo "Building $SHIELD with ZMK Studio support..."
echo "This includes the studio-rpc-usb-uart snippet for USB RPC"
echo ""

docker run --rm -it \
  -v "$(pwd)":/workspace \
  -w /workspace \
  zmkfirmware/zmk-build-arm:stable \
  bash -c "
    cd /workspace
    west init -l config 2>/dev/null || true
    west update
    west zephyr-export
    
    # Build with Studio snippet
    west build -s zmk/app -b $BOARD -d build/$SIDE --pristine -S studio-rpc-usb-uart -- \
      -DSHIELD=$SHIELD \
      -DZMK_CONFIG=/workspace/config \
      -DCONFIG_ZMK_STUDIO=y
    
    # Copy output
    cp build/$SIDE/zephyr/zmk.uf2 firmware-studio-$SHIELD-$BOARD.uf2
  "

echo ""
echo "✓ Build complete with ZMK Studio: firmware-studio-$SHIELD-$BOARD.uf2"
echo "This firmware will show up in studio.zmk.dev when connected via USB"
