#include <fstream>
#include <iostream>
#include <string>
#include <vector>

unsigned partOne(std::vector<std::string> &file) {
  unsigned count = 0;
  for (int i = 0; i < file.size(); ++i)
    for (int j = 0; j < file[0].size(); ++j) {
      bool visibleFromNorth = true;
      for (int k = i - 1; visibleFromNorth && k >= 0; --k)
        if (file[k][j] >= file[i][j])
          visibleFromNorth = false;

      if (visibleFromNorth)
        ++count;
      else {
        bool visibleFromSouth = true;
        for (int k = i + 1; visibleFromSouth && k < file.size(); ++k)
          if (file[k][j] >= file[i][j])
            visibleFromSouth = false;

        if (visibleFromSouth)
          ++count;
        else {
          bool visibleFromWest = true;
          for (int k = j - 1; visibleFromWest && k >= 0; --k)
            if (file[i][k] >= file[i][k])
              visibleFromWest = false;

          if (visibleFromWest)
            ++count;
          else {
            bool visibleFromEast = true;
            for (int k = j + 1; visibleFromEast && k < file[0].size(); ++k)
              if (file[i][k] >= file[i][j])
               visibleFromEast = false;

            if (visibleFromEast)
              ++count;
          }
        }
      }
    }
  return count;
}

int main() {
  std::ifstream input("data/day8.txt");
  std::vector<std::string> file;
  std::string line;

  while (std::getline(input, line))
    file.push_back(line);

  std::cout << partOne(file) << std::endl;

  return 0;
}
