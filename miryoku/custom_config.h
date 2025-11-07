// Copyright 2021 Manna Harbour
// https://github.com/manna-harbour/miryoku

// Custom configuration for brain keyboard
// This enables multiple layout support with easy toggling

// Configure the three alpha layouts that can be toggled between:
// - BASE: Your primary layout (double-tap any base layer key to get here)
// - EXTRA: Your secondary layout (double-tap "extra" toggle to access)
// - TAP: Your tertiary layout (double-tap "tap" toggle to access)

// To switch layouts while typing:
// 1. Hold a layer key (NAV, NUM, SYM, etc.) to access utility layers
// 2. Double-tap one of the toggle keys (usually in top right area)
// 3. Options: U_BASE, U_EXTRA, U_TAP

// Example configurations:
// For Night + Nightingale + Gallium:
//   #define MIRYOKU_ALPHAS_NIGHT
//   #define MIRYOKU_EXTRA_NIGHTINGALE
//   #define MIRYOKU_TAP_GALLIUM

// For Colemak-DH + QWERTY + Graphite:
//   #define MIRYOKU_ALPHAS_COLEMAKDH
//   #define MIRYOKU_EXTRA_QWERTY
//   #define MIRYOKU_TAP_GRAPHITE

// For Graphite + Night + Gallium:
//   #define MIRYOKU_ALPHAS_GRAPHITE
//   #define MIRYOKU_EXTRA_NIGHT
//   #define MIRYOKU_TAP_GALLIUM

// Configure all 4 alpha layers:
// ALL layers now have home row mods and layer-tap keys (full functionality)
// - BASE: Your primary layout
// - EXTRA: Secondary layout (toggle: position 7, top-right)
// - TAP: Tertiary layout (toggle: position 8, top-right) - NOW with home row mods!
// - ALT: Quaternary layout (toggle: position 5, left of BASE)

#define MIRYOKU_ALPHAS_COLEMAKDH   // BASE layer
#define MIRYOKU_EXTRA_NIGHTINGALE  // EXTRA layer
#define MIRYOKU_TAP_GALLIUM        // TAP layer (full functionality like BASE)
#define MIRYOKU_ALT_QWERTY         // ALT layer

// Explicitly override layers to ensure full functionality (home row mods + layer taps):
#define MIRYOKU_LAYER_TAP MIRYOKU_ALTERNATIVES_BASE_GALLIUM      // TAP uses BASE Gallium
#define MIRYOKU_LAYER_ALT MIRYOKU_ALTERNATIVES_BASE_QWERTY       // ALT uses BASE QWERTY

// Toggle key positions on utility layers (NAV/NUM/SYM/FUN/MOUSE/MEDIA):
// [5:ALT] [6:BASE] [7:EXTRA] [8:TAP] [9:BOOT]
// All 4 layers are now equal - any can be used as your main layout!

// Custom combo: D + Z = TAB (useful for Night layout)
// Note: Combo needs to be added to brain.keymap after includes

