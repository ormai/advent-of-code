#include <fstream>
#include <iostream>
#include <regex>
#include <string>

int main() {
  std::ifstream input_file("input");
  std::string line;
  int total = 0, conditional = 0;
  static const std::regex mul(R"(mul\((\d{1,3}),(\d{1,3})\))"),
      command(R"(don't\(\)|do\(\))");

  bool enabled = true;
  while (std::getline(input_file, line)) {
    for (std::smatch m; std::regex_search(line, m, mul); line = m.suffix()) {
      const int prod = std::stoi(m[1]) * std::stoi(m[2]);
      total += prod;

      const std::string prefix = m.prefix().str();
      if (std::smatch prefix_m; std::regex_search(prefix, prefix_m, command)) {
        enabled = prefix_m[0] == "do()";
      }
      if (enabled) {
        conditional += prod;
      }
    }
  }
  std::cout << total << '\n' << conditional << std::endl;
}
