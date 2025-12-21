use aoc::DIRECTIONS;

aoc::solution!(2024, 4);

pub fn part_one(input: &str) -> Option<u64> {
    const XMAS: [u8; 4] = [b'X', b'M', b'A', b'S'];
    let grid = parse_grid(input);
    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == XMAS[0] {
                for (dr, dc) in DIRECTIONS {
                    let mut candidate = Vec::with_capacity(XMAS.len() - 1);
                    let (mut r, mut c) = (r, c);
                    for _ in 0..XMAS.len() - 1 {
                        if let (Some(nr), Some(nc)) =
                            (r.checked_add_signed(dr), c.checked_add_signed(dc))
                        {
                            let Some(&letter) = grid.get(nr).and_then(|r| r.get(nc)) else {
                                break;
                            };
                            (r, c) = (nr, nc);
                            candidate.push(letter);
                        }
                    }
                    if candidate == XMAS[1..] {
                        count += 1;
                    }
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    const MAS: [u8; 3] = [b'M', b'A', b'S'];
    const SAM: [u8; 3] = [b'S', b'A', b'M'];
    let grid = parse_grid(input);
    let mut count = 0;
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'A'
                && let (Some(ul), Some(lr), Some(ur), Some(ll)) = (
                    grid_get(&grid, r, c, -1, -1),
                    grid_get(&grid, r, c, 1, 1),
                    grid_get(&grid, r, c, -1, 1),
                    grid_get(&grid, r, c, 1, -1),
                )
                && ([ul, b'A', lr] == MAS || [ul, b'A', lr] == SAM)
                && ([ur, b'A', ll] == MAS || [ur, b'A', ll] == SAM)
            {
                count += 1;
            }
        }
    }
    Some(count)
}

fn parse_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn grid_get<T: Copy>(grid: &[Vec<T>], r: usize, c: usize, dr: isize, dc: isize) -> Option<T> {
    grid.get(r.checked_add_signed(dr)?)?
        .get(c.checked_add_signed(dc)?)
        .copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(9));
    }
}
