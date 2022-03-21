#include <stdio.h>
#include <string.h>
#include <unistd.h>

int main(void) {
  char *a;
  char *b;
  char *c;
  char *d;
  unsigned long int i;

  a = strdup("hello");
  b = strdup("hello");
  c = strdup("hello");
  d = strdup("hello");

  i = 0;

  printf("%lu",sizeof(i));

  while (1) {
    printf("[%lu] \n %s (%p) \n %s (%p) \n %s (%p) \n %s (%p)\n", i, a, (void *)a,
           b, (void *)b, c, (void *)c, d, (void *)d);
    sleep(1);
    i++;
  }
  return 0;
}
