use std::collections::{HashMap, HashSet};

fn main() {}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}
/// Build a map of letters to regions of that letter
fn get_regions(map: &[Vec<char>]) -> HashMap<char, Vec<HashSet<(usize, usize)>>> {
    let mut regions: HashMap<char, Vec<HashSet<(usize, usize)>>> = HashMap::new();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if let Some(letter) = regions.get_mut(&map[r][c]) {
                if !letter.iter_mut().any(|region| {
                    if region.iter().any(|&(pr, pc)| {
                        let r_diff = r.abs_diff(pr);
                        let c_diff = c.abs_diff(pc);
                        r_diff + c_diff == 1 || (r_diff == 1 && c_diff == 1)
                    }) {
                        region.insert((r, c))
                    } else {
                        false
                    }
                }) {
                    letter.push(HashSet::from([(r, c)]));
                }
            } else {
                regions.insert(map[r][c], vec![HashSet::from([(r, c)])]);
            }

            // regions.entry(map[r][c]).or_insert(Vec::new()).push((r, c));
        }
    }

    for letter in regions.values_mut() {
        for (i, region) in letter.iter_mut().enumerate() {
            for other_region in &letter[i+1..] {
                if !region.is_disjoint(other_region) {
                    region.extend(other_region);
                }
            }
        }
    }


    regions
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn region_forming() {
        let map = parse_map(EXAMPLE_3);
        let regions = get_regions(&map);
        for (char, reg) in regions {
            print!("{char}: ");
            for r in reg {
                print!("[");
                for (i, j) in r {
                    print!("({i},{j})");
                }
                print!("] ");
            }
            println!();
        }
    }
}
