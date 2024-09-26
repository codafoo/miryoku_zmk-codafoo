// Copyright 2023 Manna Harbour
// https://github.com/manna-harbour/miryoku

#if !defined (MIRYOKU_LAYOUTMAPPING_BRAIN)

  #define XXX &none

  #define MIRYOKU_LAYOUTMAPPING_BRAIN( \
       K00, K01, K02, K03, K04,      K05, K06, K07, K08, K09, \
  N30, K10, K11, K12, K13, K14       K15, K16, K17, K18, K19, N38,\
  N32, K20, K21, K22, K23, K24       K25, K26, K27, K28, K29, N39,\
                 K33, K34, K31,      K37, K35, K36 \
  ) \
       K00  K01  K02  K03  K04       K05  K06  K07  K08  K09 \
  XXX  K10  K11  K12  K13  K14       K15  K16  K17  K18  K19 XXX \
  XXX  K20  K21  K22  K23  K24       K25  K26  K27  K28  K29 XXX \
                 K33  K34  K31       K37  K35  K36

  #endif

#define MIRYOKU_MAPPING MIRYOKU_LAYOUTMAPPING_BRAIN
