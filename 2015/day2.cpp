#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>

int main() {
  std::ifstream input("data/day2.txt");

  unsigned wrappingPaperSurface = 0, ribbonLength = 0;
  std::string line;
  while (std::getline(input, line)) {
    unsigned firstX = line.find('x'), secondX = line.find('x', firstX + 1);
    unsigned l = std::stoul(line.substr(0, firstX)),
             w = std::stoul(line.substr(firstX + 1, secondX - 1 - firstX)),
             h = std::stoul(line.substr(secondX + 1));

    wrappingPaperSurface +=
        2 * l * w + 2 * w * h + 2 * h * l + std::min({l * w, w * h, l * h});

    ribbonLength +=
        std::min({l + l + w + w, w + w + h + h, l + l + h + h}) + l * w * h;
  }

  std::cout << "Square feet of wrapping paper: " << wrappingPaperSurface << '\n'
            << "Feet of ribbon: " << ribbonLength << std::endl;

  return 0;
}
