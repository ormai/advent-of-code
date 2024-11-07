#include <stdio.h>

int main() {
  FILE *input = fopen("input", "r");
  int pos = 0, depth = 0, aim = 0, pos2 = 0, depth2 = 0;
  char action[8];
  int amount;
  while (fscanf(input, "%s %d\n", action, &amount) != EOF) {
    switch (action[0]) {
    case 'f':
      pos += amount;
      pos2 += amount;
      depth2 += aim * amount;
      break;
    case 'd':
      depth += amount;
      aim += amount;
      break;
    case 'u':
      depth -= amount;
      aim -= amount;
      break;
    }
  }
  printf("Part One: %d\nPart Two: %d\n", pos * depth, pos2 * depth2);
  fclose(input);
}
