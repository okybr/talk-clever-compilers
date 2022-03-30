#include <stdio.h>
#include <stdlib.h> // für malloc

int main() {
  // Lese Arraygröße zur Laufzeit ein:
  int anzahl;
  scanf("%d", &anzahl);

  // Allokiere Speicher:
  int* array = malloc(sizeof(int));

  // Gib Speicher frei:
  free(array);
}
