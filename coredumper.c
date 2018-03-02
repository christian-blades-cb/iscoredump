// I'm here to generate a core dump for testing. Don't forget to `ulimit -c unlimited`.
#include <stdio.h>

int main() {
  int *foo;
  printf("%d\n", foo[0]);

  return 0;
}
