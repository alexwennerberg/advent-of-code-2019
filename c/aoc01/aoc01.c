#include <stdio.h>

int calculate_fuel(int mass);

int main() {
  FILE* nfile = fopen("input.txt", "r");
  int mass;
  int total;
  while(fscanf(nfile, "%d", &mass) == 1) {
    total = total + calculate_fuel(mass);
  }
  printf("%d", total);
  return 0;
}

int calculate_fuel(int mass) {
  int total_fuel = 0;
  int fuel;
  fuel = mass / 3 - 2;
  while (fuel > 0) {
    total_fuel = total_fuel + fuel;
    fuel = fuel / 3 - 2;
  }
  return total_fuel;
}
