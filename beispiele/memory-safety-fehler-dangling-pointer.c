#include <stdio.h>
#include <stdlib.h>

int main() {
  int anzahl;
  scanf("%d", &anzahl);
  int* array = malloc(sizeof(int));
  free(array);
  for (int i = 0; i < anzahl; i++) {
    array[i] = i;
  }
}
