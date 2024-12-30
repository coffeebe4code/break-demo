#include "./single.h"

single_t single_new(single_k kind, power_k powerkind) {
  single_t a = { .kind = kind, .powerkind = powerkind };
  return a;
}
