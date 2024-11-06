#include <stdio.h>
#include <string.h>

int main() {
  FILE *input = fopen("input", "r");
  int win[4], count_one = 0, count_two;
  fscanf(input, "%d\n%d\n%d\n", &win[0], &win[1], &win[2]);
  while (fscanf(input, "%d\n", &win[3]) != EOF) {
    if (win[1] + win[2] + win[3] > win[0] + win[1] + win[2]) {
      ++count_two;
    }
    if (win[1] > win[0]) {
      ++count_one;
    }
    memcpy(win, &win[1], sizeof(int[3]));
  }
  fclose(input);
  if (win[2] > win[1]) {
    ++count_one;
  }
  if (win[3] > win[2]) {
    ++count_one;
  }
  printf("Part One: %d\n", count_one);
  printf("Part Two: %d\n", count_two);
}
