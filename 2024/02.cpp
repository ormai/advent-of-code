#include <algorithm>
#include <fstream>
#include <functional>
#include <iostream>
#include <print>
#include <string>
#include <vector>

bool monotony(const std::vector<int> &levels,
              const std::function<bool(int, int)> op) {
  return std::adjacent_find(levels.begin(), levels.end(), op) == levels.end();
}

bool is_safe(const std::vector<int> &levels) {
  return std::adjacent_find(levels.begin(), levels.end(),
                            [](int a, int b) {
                              const int distance = std::abs(a - b);
                              return distance < 1 || distance > 3;
                            }) == levels.end() &&
         (monotony(levels, std::less<>()) ||   // decreasing
          monotony(levels, std::greater<>())); // increasing
}

std::vector<int> split_to_ints(const std::string &s) {
  std::vector<int> tokens;
  std::string::size_type last = 0, next = 0;
  while ((next = s.find(' ', last)) != std::string::npos) {
    tokens.push_back(std::stoi(s.substr(last, next - last)));
    last = next + 1;
  }
  tokens.push_back(std::stoi(s.substr(last)));
  return tokens;
}

int main() {
  int safe_reports = 0, dampened = 0;
  std::ifstream input("input");
  std::string line;
  while (std::getline(input, line)) {
    std::vector<int> levels = split_to_ints(line);
    if (is_safe(levels)) {
      safe_reports++;
    } else {
      for (int i = 0; i < levels.size(); ++i) {
        std::vector<int> maybe(levels.begin(), levels.begin() + i);
        maybe.insert(maybe.end(), levels.begin() + i + 1, levels.end());
        if (is_safe(maybe)) {
          dampened++;
          break;
        }
      }
    }
  }
  std::cout << safe_reports << '\n' << safe_reports + dampened << std::endl;
}
