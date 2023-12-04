#include <cctype>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <tuple>
#include <list>
#include <vector>

// A card is a set of winning numbers and a set of the numbers we have
// Also store the points
typedef std::tuple<std::set<unsigned>, std::set<unsigned>, unsigned, unsigned>
    scratchcard;

std::set<unsigned> getNumbers(const std::string &raw);
void countPoints(std::list<scratchcard> &card);
unsigned partOne(const std::list<scratchcard> &cards);
unsigned partTwo(std::list<scratchcard> &cards);

int main() {
  std::ifstream input("data/day4.txt");

  std::list<scratchcard> cards;

  std::string line;
  while (std::getline(input, line)) {
    const unsigned start = line.find(':') + 2, sep = line.find('|');
    std::string key = line.substr(0, start - 2);

    std::string winning = line.substr(start, sep - start),
                ours = line.substr(sep + 2, line.size() - sep + 36);

    ours.push_back(' ');
    cards.push_back({getNumbers(winning), getNumbers(ours), 0, 1});
  }

  countPoints(cards);
  // std::cout << partOne(cards) << '\n';
  std::cout << partTwo(cards) << std::endl;
}

unsigned partTwo(std::list<scratchcard> &cards) {
  std::vector<scratchcard> cardsV;

  for (scratchcard card : cards)
    cardsV.push_back(card);

  for (int i = 0; i < cards.size(); ++i) {
    auto &[a, b, points, count] = cardsV[i];
    for (int j = 1; j <= points; ++j) {
      auto &[a, b, c, count2] = cardsV[i + j];
      count2 += count;
    }
  }

  unsigned duplicates = 0;
  for (auto &[a, b, c, count] : cardsV) {
    duplicates += count;
  }
  return duplicates;
}

unsigned partOne(const std::list<scratchcard> &cards) {
  unsigned points = 0;
  for (scratchcard card : cards)
    points += std::get<2>(card);
  return points;
}

void countPoints(std::list<scratchcard> &cards) {
  for (auto &card : cards) {
    auto &[winning, ours, points, count] = card;
    for (unsigned n : ours)
      if (winning.count(n)) {
        ++points; // only count how many it matches for partTwo
        // if (points == 0)
        //   points = 1;
        // else
        //   points *= 2;
      }
  }
}

std::set<unsigned> getNumbers(const std::string &raw) {
  std::set<unsigned> nums;

  std::string currentNumber;
  for (char c : raw)
    if (isdigit(c))
      currentNumber += c;
    else if (!currentNumber.empty()) {
      nums.insert(std::stoul(currentNumber));
      currentNumber = "";
    }

  return nums;
}
