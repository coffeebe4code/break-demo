#pragma once

typedef enum single_k {
  B,
  R,
  E,
  A,
  K,
  S1,
  S2,
  S3,
  S4
} single_k;

typedef enum power_k {
  None,
  Def,
  Att
} power_k;

typedef struct single_t {
  single_k kind;
  power_k powerkind;
} single_t;

single_t single_new(single_k kind, power_k powerkind);
