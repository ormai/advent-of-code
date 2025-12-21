use aoc::ORTHOGONAL_DIRECTIONS;
use std::collections::{HashSet, VecDeque};

aoc::solution!(2024, 10);

pub fn part_one(input: &str) -> Option<u64> {
    let (score, _) = walk_the_trail(parse_grid(input));
    Some(score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, rating) = walk_the_trail(parse_grid(input));
    Some(rating)
}

fn walk_the_trail(grid: Vec<Vec<u8>>) -> (u64, u64) {
    let len = grid.len();
    assert_eq!(len, grid[0].len(), "The grid is a square");

    let (mut score, mut rating) = (0, 0);
    let mut queue = VecDeque::with_capacity(len);
    let mut visited = HashSet::with_capacity(len);
    for r in 0..len {
        for c in 0..len {
            if grid[r][c] == 0 {
                visited.clear();
                queue.clear();
                queue.push_back((r, c));
                while let Some((pr, pc)) = queue.pop_front() {
                    if grid[pr][pc] == 9 {
                        if visited.insert((pr, pc)) {
                            score += 1
                        }
                        rating += 1
                    } else {
                        for (dr, dc) in ORTHOGONAL_DIRECTIONS {
                            if let Some(nr) = pr.checked_add_signed(dr)
                                && let Some(nc) = pc.checked_add_signed(dc)
                                && let Some(&cell) = grid.get(nr).and_then(|row| row.get(nc))
                                && cell == grid[pr][pc] + 1
                            {
                                queue.push_back((nr, nc));
                            }
                        }
                    }
                }
            }
        }
    }

    (score, rating)
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(81));
    }
}
