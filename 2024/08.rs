use std::collections::{HashMap, HashSet};

fn main() {
    let map = parse_map(include_str!("input"));
    println!(
        "{}\n{}",
        count_antinodes(&map),
        count_antinodes_repeat(&map)
    );
}

fn categorize_antennas_by_frequency(map: &[Vec<char>]) -> HashMap<char, Vec<(usize, usize)>> {
    let mut frequencies = HashMap::new();
    for (r, row) in map.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell != '.' {
                frequencies.entry(cell).or_insert(Vec::new()).push((r, c));
            }
        }
    }
    frequencies
}

/// Part one.
fn count_antinodes(map: &[Vec<char>]) -> usize {
    let frequencies = categorize_antennas_by_frequency(map);

    let mut antinodes = HashSet::new();
    for (_, frequency) in frequencies {
        for (r, &(r1, c1)) in frequency.iter().enumerate() {
            for &(r2, c2) in &frequency[r + 1..] {
                if let (Some(r), Some(c)) = ((2 * r1).checked_sub(r2), (2 * c1).checked_sub(c2)) {
                    if r < map.len() && c < map[r].len() {
                        antinodes.insert((r, c));
                    }
                }
                if let (Some(r), Some(c)) = ((2 * r2).checked_sub(r1), (2 * c2).checked_sub(c1)) {
                    if r < map.len() && c < map[r].len() {
                        antinodes.insert((r, c));
                    }
                }
            }
        }
    }
    antinodes.len()
}

/// Part two.
fn count_antinodes_repeat(map: &[Vec<char>]) -> usize {
    let frequencies = categorize_antennas_by_frequency(map);
    let (w, h) = (map.len() as isize, map[0].len() as isize);
    let mut antinodes = HashSet::new();
    for (_, frequency) in &frequencies {
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
    antinodes.len()
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::{count_antinodes, count_antinodes_repeat, parse_map};

    const EXAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn count_antinodes_works_well() {
        assert_eq!(count_antinodes(&parse_map(EXAMPLE)), 14);
    }

    #[test]
    fn count_antinodes_repeat_works_well() {
        assert_eq!(count_antinodes_repeat(&parse_map(EXAMPLE)), 34);
    }
}
