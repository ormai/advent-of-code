#include <algorithm>
#include <cstdlib>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>

int main() {
  std::ifstream input("input");
  std::vector<int> left, right;
  int first, second;
  while (input >> first >> second) {
    left.push_back(first);
    right.push_back(second);
  }
  input.close();

  // Part Two
  const int similarity_score =
      std::accumulate(left.begin(), left.end(), 0, [right](int acc, int n) {
        return acc + std::count(right.begin(), right.end(), n) * n;
      });

  // Part One
  std::sort(left.begin(), left.end());
  std::sort(right.begin(), right.end());
  int distance = 0;
  for (int i = 0; i < left.size(); ++i) {
    distance += std::abs(left[i] - right[i]);
  }

  std::cout << distance << '\n' << similarity_score << std::endl;
}
