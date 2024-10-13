#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <unordered_map>
#include <utility>
#include <vector>

bool partTwo = false;
std::unordered_map<char, unsigned char> cardValues = {
    {'2', 2}, {'3', 3},  {'4', 4},  {'5', 5},  {'6', 6},  {'7', 7}, {'8', 8},
    {'9', 9}, {'T', 10}, {'J', 11}, {'Q', 12}, {'K', 13}, {'A', 14}};

char getMostFrequentCharacter(
    const std::unordered_map<char, unsigned> &frequency) {
  char mostFrequent, maxCount = 0;
  for (const auto [label, count] : frequency)
    if (label != 'J' && count > maxCount) {
      maxCount = count;
      mostFrequent = label;
    }
  return mostFrequent;
}

unsigned strength(std::string hand) {
  std::unordered_map<char, unsigned> frequency;
  for (const char label : hand)
    ++frequency[label];

  if (partTwo) {
    const char mostFrequent = getMostFrequentCharacter(frequency);
    for (char &label : hand)
      if (label == 'J')
        label = mostFrequent;

    // count the frequencies again
    frequency.clear();
    for (const char label : hand)
      ++frequency[label];
  }

  const unsigned uniqueLabels = frequency.size();
  if (uniqueLabels == 2) {
    for (const auto [label, count] : frequency)
      if (count == 4)
        return 6; // four of a kind: 4 1
      else if (count == 3)
        return 5; // full house: 3 2
  } else if (uniqueLabels == 3) {
    for (const auto [label, count] : frequency)
      if (count == 3)
        return 4; // three of a kind: 3 1 1
      else if (count == 2)
        return 3; // two pair: 2 2 1
  } else if (uniqueLabels == 1)
    return 7; // five of a kind: 5
  else if (uniqueLabels == 4)
    return 2; // one pair: 2 1 1 1
  return 1;   // high card: 1 1 1 1 1
}

bool compareHands(const std::pair<std::string, unsigned> &lhs,
                  const std::pair<std::string, unsigned> &rhs) {

  const unsigned lhsStrenght = strength(lhs.first),
                 rhsStrength = strength(rhs.first);

  if (partTwo)
    cardValues['J'] = 0;

  if (lhsStrenght == rhsStrength)
    for (int i = 0; i < 5; ++i)
      if (lhs.first[i] != rhs.first[i])
        return cardValues.at(lhs.first[i]) < cardValues.at(rhs.first[i]);

  return lhsStrenght < rhsStrength;
}

unsigned countWinnings(std::vector<std::pair<std::string, unsigned>> hands) {
  std::sort(hands.begin(), hands.end(), compareHands);

  unsigned winnings = 0, i = 0;
  for (auto &[hand, bid] : hands)
    winnings += ++i * bid;
  return winnings;
}

int main() {
  std::ifstream input("data/day7.txt");
  std::vector<std::pair<std::string, unsigned>> hands;

  std::string line;
  while (std::getline(input, line)) {
    const unsigned sep = line.find(' ');
    hands.push_back({line.substr(0, sep), std::stoul(line.substr(sep + 1))});
  }

  std::cout << countWinnings(hands) << '\n';
  partTwo = true;
  std::cout << countWinnings(hands) << std::endl;

  return 0;
}
