use std::collections::{HashMap, HashSet};

use aoc::ORTHOGONAL_DIRECTIONS;

aoc::solution!(2024, 12);

pub fn part_one(input: &str) -> Option<u64> {
    price_of_fencing(&determine_regions(&parse_grid(input)))
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

/// Build a map of letters to regions of that letter
fn determine_regions(grid: &[Vec<u8>]) -> HashMap<u8, Vec<HashSet<(usize, usize)>>> {
    let (width, height) = (grid.len(), grid[0].len());
    let mut regions: HashMap<u8, Vec<HashSet<(usize, usize)>>> = HashMap::new();
    for r in 0..height {
        for c in 0..width {
            let regions_for_plant = regions.entry(grid[r][c]).or_default();
            if let Some(region) = find_owning_region(grid, regions_for_plant, r, c) {
                region.insert((r, c));
            } else {
                regions_for_plant.push(HashSet::from([(r, c)]));
            }
        }
    }
    regions
}

/// Explore the map in every direction looking for a connection to an existing region
fn find_owning_region<'a>(
    map: &[Vec<u8>],
    regions: &'a mut [HashSet<(usize, usize)>],
    r: usize,
    c: usize,
) -> Option<&'a mut HashSet<(usize, usize)>> {
    let (width, height) = (map.len(), map[0].len());
    for (dr, dc) in ORTHOGONAL_DIRECTIONS {
        let (mut r, mut c) = (r, c);
        while let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc))
            && nr < height
            && nc < width
            && map[nr][nc] == map[r][c]
        {
            (r, c) = (nr, nc);

            if let Some(i) = regions.iter().position(|region| {
                region
                    .iter()
                    .any(|&(or, oc)| nr == or && nc == oc || orthogonally_adjacent(nr, nc, or, oc))
            }) {
                return Some(&mut regions[i]);
            }
        }
    }
    None
}

/// Computes the total price of fencing for all the regions by summing the product of each
/// region's perimiter and area.
fn price_of_fencing(regions: &HashMap<u8, Vec<HashSet<(usize, usize)>>>) -> Option<u64> {
    regions
        .values()
        .flatten()
        .fold(0, |cost, region| {
            cost + region.iter().fold(0, |perimeter, (r, c)| {
                perimeter + ORTHOGONAL_DIRECTIONS.len()
                    - ORTHOGONAL_DIRECTIONS
                        .iter()
                        .filter_map(|&(dr, dc)| {
                            Some((r.checked_add_signed(dr)?, c.checked_add_signed(dc)?))
                        })
                        .filter(|&(nr, nc)| region.contains(&(nr, nc)))
                        .count()
            }) * region.len()
        })
        .try_into()
        .ok()
}

/// Whether two points in a grid are either on the same row or on the same column.
fn orthogonally_adjacent(r: usize, c: usize, other_r: usize, other_c: usize) -> bool {
    let dr = r.abs_diff(other_r);
    let dc = c.abs_diff(other_c);
    (dr == 1 || dc == 1) && dr + dc == 1
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_first_example() {
        assert_eq!(
            part_one(
                "AAAA
BBCD
BBCC
EEEC"
            ),
            Some(140)
        );
    }

    #[test]
    fn test_part_one_multiple_small_regions_with_the_same_letter() {
        assert_eq!(
            part_one(
                "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO"
            ),
            Some(772)
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, None);
    }
}
