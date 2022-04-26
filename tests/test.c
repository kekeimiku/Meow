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



  while (1) {
    printf("[%lu] %p \n %s (%p) \n %s (%p) \n %s (%p) \n %s (%p)\n",i, &i, a, (void *)a,
           b, (void *)b, c, (void *)c, d, (void *)d);
    sleep(3);
    i++;
  }
  return 0;
}
