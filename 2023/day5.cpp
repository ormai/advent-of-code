#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

struct Row {
  unsigned long destination = 0;
  unsigned long source = 0;
  unsigned long length = 0;
};

struct Map {
  std::vector<Row> rows;

  unsigned operator[](const unsigned target) const {
    for (Row r : rows)
      if (target >= r.source && target < r.source + r.length) {
        return r.destination + target - r.source;
      }
    return target;
  }

  unsigned getMaxDestination() const {
    unsigned max = 0;
    for (Row r : rows)
      if (r.destination + r.length - 1 > max)
        max = r.destination + r.length - 1;
    return max;
  }
};

unsigned partTwo(const std::vector<unsigned> &seeds, const Map &seedSoil,
                 const Map &soilFertilizer, const Map &fertilizerWater,
                 const Map &waterLight, const Map &lightTemperature,
                 const Map &temperatureHumidity, const Map &humidityLocation) {

  /* It's very inefficient. On my machine it took 50:42 minutes to complete */
  unsigned lowestLocation = humidityLocation.getMaxDestination();

  for (int i = 0; i < seeds.size(); i += 2)
    for (int j = 0; j < seeds[i + 1]; ++j) {
      unsigned location = humidityLocation
          [temperatureHumidity
               [lightTemperature
                    [waterLight
                         [fertilizerWater
                              [soilFertilizer[seedSoil[seeds[i] + j]]]]]]];

      if (location < lowestLocation)
        lowestLocation = location;
    }
  return lowestLocation;
}

unsigned partOne(const std::vector<unsigned> &seeds, const Map &seedSoil,
                 const Map &soilFertilizer, const Map &fertilizerWater,
                 const Map &waterLight, const Map &lightTemperature,
                 const Map &temperatureHumidity, const Map &humidityLocation) {

  std::vector<unsigned> locations;

  for (unsigned seed : seeds) {
    locations.push_back(
        humidityLocation
            [temperatureHumidity
                 [lightTemperature
                      [waterLight[fertilizerWater
                                      [soilFertilizer[seedSoil[seed]]]]]]]);
  }

  return *std::min_element(locations.begin(), locations.end());
}

Row toRow(std::string &raw) {
  unsigned firstSpace = raw.find(' '),
           secondSpace = raw.find(' ', firstSpace + 1);
  return {std::stoul(raw.substr(0, firstSpace)),
          std::stoul(raw.substr(firstSpace + 1, secondSpace - firstSpace - 1)),
          std::stoul(raw.substr(secondSpace + 1))};
}

int main() {
  std::vector<unsigned> seeds;
  Map seedToSoil, soilToFertilizer, fertilizerToWater, waterToLight,
      lightToTemperature, temperatureToHumidity, humidityToLocation;

  bool seed_soil, soil_fertilizer, fertilizer_water, water_light,
      light_temperature, temperature_humidity, humidity_location;

  std::ifstream input("data/day5.txt");
  std::string line;
  while (std::getline(input, line)) {
    if (line == "")
      seed_soil = soil_fertilizer = fertilizer_water = water_light =
          light_temperature = temperature_humidity = humidity_location = false;
    if (line.substr(0, 7) == "seeds: ") {
      long unsigned space = line.find(' ', 7), start = 7;
      seeds.push_back(std::stoull(line.substr(start, space - 1)));
      do {
        start = space + 1;
        space = line.find(' ', start);
        seeds.push_back(std::stoul(line.substr(start, space - 1)));
      } while (space != std::string::npos);
    } else if (line == "seed-to-soil map:")
      seed_soil = true;
    else if (seed_soil)
      seedToSoil.rows.push_back(toRow(line));
    else if (line == "soil-to-fertilizer map:")
      soil_fertilizer = true;
    else if (soil_fertilizer)
      soilToFertilizer.rows.push_back(toRow(line));
    else if (line == "fertilizer-to-water map:")
      fertilizer_water = true;
    else if (fertilizer_water)
      fertilizerToWater.rows.push_back(toRow(line));
    else if (line == "water-to-light map:")
      water_light = true;
    else if (water_light)
      waterToLight.rows.push_back(toRow(line));
    else if (line == "light-to-temperature map:")
      light_temperature = true;
    else if (light_temperature)
      lightToTemperature.rows.push_back(toRow(line));
    else if (line == "temperature-to-humidity map:")
      temperature_humidity = true;
    else if (temperature_humidity)
      temperatureToHumidity.rows.push_back(toRow(line));
    else if (line == "humidity-to-location map:")
      humidity_location = true;
    else if (humidity_location)
      humidityToLocation.rows.push_back(toRow(line));
  }

  std::cout << partOne(seeds, seedToSoil, soilToFertilizer, fertilizerToWater,
                       waterToLight, lightToTemperature, temperatureToHumidity,
                       humidityToLocation)
            << '\n'
            << partTwo(seeds, seedToSoil, soilToFertilizer, fertilizerToWater,
                       waterToLight, lightToTemperature, temperatureToHumidity,
                       humidityToLocation)
            << std::endl;

  return 0;
}
