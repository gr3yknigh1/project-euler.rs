#include "stdio.h"

int main() {
  int sum_of_seq = 0;
  int sum_of_sqrt = 0;

  for (int i=1; i < 100 + 1; i++) {
    sum_of_seq += i;
    sum_of_sqrt += i * i;
  }
  
  int total = sum_of_seq * sum_of_seq - sum_of_sqrt;

  printf("%d", total);
  return 0;
}
