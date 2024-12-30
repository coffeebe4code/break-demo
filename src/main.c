#include "pieces/single.h"
#include <stdio.h>

int main() {
  single_t a = single_new(0,0);
  
  return (int)a.powerkind;
}
