#include "./single.h"
#include "./triple.h"

triple_t triple_new(single_t top, single_t mid, single_t bot) {
  triple_t a = {
    .top = top,
    .mid = mid,
    .bot = bot,
  };
  return a;
}

void triple_rotate_up(triple_t* triple) {
  single_t temp = triple->top;
  triple->top = triple->mid;
  triple->mid = triple->bot;
  triple->bot = temp;
}
