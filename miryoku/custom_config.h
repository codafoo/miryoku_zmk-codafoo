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

// Custom combo: Q + Z = TAB (useful for Nightingale layout on Extra layer)
// Note: Combo needs to be added to keyboard keymaps after includes

// Override NIGHTINGALE to use SPACE instead of R on second thumb key
// This makes the Extra layer thumb keys consistent with Base layer
#define MIRYOKU_ALTERNATIVES_BASE_NIGHTINGALE \
&kp B,             &kp F,             &kp L,             &kp D,             &kp K,             &kp P,             &kp W,             &kp O,             &kp U,             &kp DOT,           \
U_MT(LGUI, N),     U_MT(LALT, S),     U_MT(LCTRL, H),    U_MT(LSHFT, T),    &kp M,             &kp Y,             U_MT(LSHFT, C),    U_MT(LCTRL, A),    U_MT(LALT, E),     U_MT(LGUI, I),     \
U_LT(U_BUTTON, X), U_MT(RALT, V),     &kp J,             &kp Q,             &kp Z,             &kp COMMA,         &kp G,             &kp SLASH,         U_MT(RALT, SEMI),  U_LT(U_BUTTON, SQT),\
U_NP,              U_NP,              U_LT(U_MEDIA, ESC),U_LT(U_NAV, SPACE),U_LT(U_MOUSE, TAB),U_LT(U_SYM, RET),  U_LT(U_NUM, BSPC), U_LT(U_FUN, DEL),  U_NP,              U_NP

