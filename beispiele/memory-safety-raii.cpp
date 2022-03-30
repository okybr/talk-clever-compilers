#include <iostream>

class Objekt {

private:
  int* array;

public:
  Objekt(const int anzahl) {
    array = new int[anzahl]; // Setze Objektfeld data
                             // als Referenz auf neu allokierten Speicher
  }
  ~Objekt() {
    delete[] array;          // Gebe dynamischen Speicherbereich frei
  }
};

int main() {
  int anzahl;
  std::cin >> anzahl;
  Objekt objekt(anzahl);
}                            // Gültigkeitsbereich der Variable object endet.\
                             // Destruktor wird implizit aufgerufen.
