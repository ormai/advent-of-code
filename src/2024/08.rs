use std::collections::{HashMap, HashSet};

aoc::solution!(2024, 8);

pub fn part_one(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let mut antinodes = HashSet::new();
    for (_, frequency) in categorize_antennas_by_frequency(&map) {
        for (r, &(r1, c1)) in frequency.iter().enumerate() {
            for &(r2, c2) in &frequency[r + 1..] {
                if let (Some(r), Some(c)) = ((2 * r1).checked_sub(r2), (2 * c1).checked_sub(c2))
                    && r < map.len()
                    && c < map[r].len()
                {
                    antinodes.insert((r, c));
                }
                if let (Some(r), Some(c)) = ((2 * r2).checked_sub(r1), (2 * c2).checked_sub(c1))
                    && r < map.len()
                    && c < map[r].len()
                {
                    antinodes.insert((r, c));
                }
            }
        }
    }
    antinodes.len().try_into().ok()
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = parse_input(input);
    let frequencies = categorize_antennas_by_frequency(&map);
    let (w, h) = (map.len() as isize, map[0].len() as isize);
    let mut antinodes = HashSet::new();
    for frequency in frequencies.values() {
        for &(r1, c1) in frequency {
            for &(r2, c2) in frequency {
                if r1 != r2 || c1 != c2 {
                    let (dr, dc) = (r2 as isize - r1 as isize, c2 as isize - c1 as isize);
                    let (mut r, mut c) = (r1 as isize, c1 as isize);
                    while r >= 0 && c >= 0 && r < h && c < w {
                        antinodes.insert((r, c));
                        r += dr;
                        c += dc;
                    }
                }
            }
        }
    }
    antinodes.len().try_into().ok()
}

fn categorize_antennas_by_frequency(map: &[Vec<u8>]) -> HashMap<u8, Vec<(usize, usize)>> {
    let mut frequencies = HashMap::new();
    for (r, row) in map.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != b'.' {
                frequencies.entry(cell).or_insert(Vec::new()).push((r, c));
            }
        }
    }
    frequencies
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(34));
    }
}
