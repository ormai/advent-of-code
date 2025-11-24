use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("../input");

fn main() {
    determine_regions(INPUT.lines().map(|line| line.chars().collect()).collect());
}

/// Build a map of letters to regions of that letter
fn determine_regions(map: Vec<Vec<char>>) -> HashMap<char, Vec<HashSet<(usize, usize)>>> {
    let mut regions: HashMap<char, Vec<HashSet<(usize, usize)>>> = HashMap::new();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            let regions_for_plant = regions.entry(map[r][c]).or_insert(Vec::new());

            if let Some(partial_region) = regions_for_plant.iter_mut().find(|partial_region| {
                partial_region
                    .iter()
                    .any(|&(or, oc)| orthogonally_adjacent(r, c, or, oc))
            }) {
                partial_region.insert((r, c));
            } else {
                regions_for_plant.push(HashSet::from([(r, c)]));
            }
        }
    }

    // Proceeding from the top left corner of the grid when determining regions
    // falls short in this case:
    //
    //   ...A...
    //   ..AA...
    //
    // The A on the first line and the first A on the second line arent adjacent
    // even though they belong to the same region.

    regions
}

/// Whether two points in a grid are either on the same row or on the same column.
fn orthogonally_adjacent(r: usize, c: usize, other_r: usize, other_c: usize) -> bool {
    let dr = r.abs_diff(other_r);
    let dc = c.abs_diff(other_c);
    (dr == 1 || dc == 1) && dr + dc == 1
}
