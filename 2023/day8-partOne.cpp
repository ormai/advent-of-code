#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <utility>

typedef std::pair<std::string, std::string> node;

unsigned stepThrough(const std::unordered_map<std::string, node> &nodes,
                     const std::string &instructions) {
  unsigned count = 0, i = 0;
  std::string first = "AAA";
  while (first != "ZZZ") {
    if (i == instructions.size())
      i = 0;
    first = instructions[i++] == 'L' ? nodes.at(first).first
                                     : nodes.at(first).second;
    ++count;
  }
  return count;
}

int main() {
  std::unordered_map<std::string, node> nodes;

  std::ifstream input("data/day8.txt");
  std::string instructions, line;
  std::getline(input, instructions); // first line only
  std::getline(input, line);         // discard one empty line
  while (std::getline(input, line))  // all the others
    nodes[line.substr(0, 3)] = {line.substr(7, 3), line.substr(12, 3)};

  std::cout << stepThrough(nodes, instructions) << std::endl;
  return 0;
}
