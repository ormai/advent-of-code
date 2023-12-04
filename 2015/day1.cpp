#include <fstream>
#include <iostream>

int main() {
  std::ifstream input("data/day1.txt");

  int floor = 0;
  char c;
  unsigned position = 1, basement = 0;
  while (input >> c) {
    if (c == '(')
      ++floor;
    else
      --floor;

    if (basement == 0 && floor == -1)
      basement = position;

    ++position;
  }

  std::cout << "Final floor: " << floor << '\n'
            << "Entered basement at: " << basement << std::endl;

  return 0;
}
