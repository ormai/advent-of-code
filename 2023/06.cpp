#include <cctype>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

unsigned partOne(const std::vector<long unsigned> &time,
                 const std::vector<long unsigned> &distance) {
  unsigned prod = 1;

  for (int i = 0; i < time.size(); ++i) {
    unsigned ways = 0;
    for (int j = 1; j < time[i]; ++j)
      if ((time[i] - j) * j > distance[i])
        ++ways;
    prod *= ways;
  }
  return prod;
}

int main() {
  std::ifstream input("data/day6.txt");

  std::string timeRaw, distanceRaw;
  std::getline(input, timeRaw);
  std::getline(input, distanceRaw);

  std::vector<long unsigned> time, distance;

  long unsigned num = 0;
  for (const char c : timeRaw)
    if (isdigit(c))
      num = num * 10 + (c - 48);
    else if (num != 0) {
      time.push_back(num);
      num = 0;
    }
  time.push_back(num);

  num = 0;
  for (const char c : distanceRaw)
    if (isdigit(c))
      num = num * 10 + (c - 48);
    else if (num != 0) {
      distance.push_back(num);
      num = 0;
    }
  distance.push_back(num);

  std::cout << partOne(time, distance) << '\n';

  long unsigned bigTime = 0, bigDistance = 0;
  for (int i = 0; i < timeRaw.size(); ++i) {
    if (isdigit(timeRaw[i]))
      bigTime = bigTime * 10 + (timeRaw[i] - 48);

    if (isdigit(distanceRaw[i]))
      bigDistance = bigDistance * 10 + (distanceRaw[i] - 48);
  }

  // create objects because the partOne() takes only references to const
  std::vector<unsigned long> bigTimeSingle = {bigTime},
                             bigDistanceSingle = {bigDistance};
  std::cout << partOne(bigTimeSingle, bigDistanceSingle) << std::endl;

  return 0;
}
