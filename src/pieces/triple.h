#pragma once
#include "./single.h"

typedef struct triple_t {
  single_t top;
  single_t mid;
  single_t bot;
} triple_t; 

triple_t triple_new(single_t top, single_t mid, single_t bot);
void triple_rotate_up(triple_t* triple);
