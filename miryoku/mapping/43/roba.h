// Copyright 2025 Manna Harbour
// https://github.com/manna-harbour/miryoku

// roBa keyboard mapping for Miryoku
// 43-key split keyboard with 3 thumb keys on left, 2 on right
// Extra pinky keys (RC(1,5), RC(2,5), RC(3,6), RC(3,7)) unmapped for ZMK Studio configuration

#if !defined (MIRYOKU_LAYOUTMAPPING_ROBA)

#define XXX &none

#define MIRYOKU_LAYOUTMAPPING_ROBA(\
     K00, K01, K02, K03, K04,                          K05, K06, K07, K08, K09, \
     K10, K11, K12, K13, K14,                          K15, K16, K17, K18, K19, \
     K20, K21, K22, K23, K24,                          K25, K26, K27, K28, K29, \
     N30, N31, K32, K33, K34,                          K35, K36, N37, N38, N39 \
)\
     K00  K01  K02  K03  K04                           K05  K06  K07  K08  K09       \
     K10  K11  K12  K13  K14  XXX               XXX    K15  K16  K17  K18  K19  XXX  \
     K20  K21  K22  K23  K24  XXX               XXX    K25  K26  K27  K28  K29  XXX  \
XXX  XXX  XXX  K32  K33  K34                           K35  K36                 XXX

// roBa has 3 physical thumb keys on left, 2 on right - need combo for 3rd right thumb
#define MIRYOKU_KLUDGE_THUMBCOMBOS_LEFT
#define MIRYOKU_KLUDGE_THUMBCOMBOS_RIGHT 36 37

#endif

#define MIRYOKU_MAPPING MIRYOKU_LAYOUTMAPPING_ROBA
