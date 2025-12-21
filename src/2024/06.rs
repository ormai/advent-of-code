use std::collections::HashSet;

aoc::solution!(2024, 6);

const ORTHOGONAL_DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start) = parse_input(input);
    let (mut guard, mut dir, mut positions) = (start, 0, HashSet::from([start]));
    while let Some((nr, nc)) = next_position(&grid, &guard, &ORTHOGONAL_DIRECTIONS[dir]) {
        if grid[nr][nc] == b'#' {
            dir = (dir + 1) % ORTHOGONAL_DIRECTIONS.len();
        } else {
            positions.insert((nr, nc));
            guard = (nr, nc);
        }
    }
    positions.len().try_into().ok()
}

pub fn part_two(input: &str) -> Option<u64> {
    // FIXME: too slow
    let (mut grid, start) = parse_input(input);
    let mut count = 0;
    let mut visited = HashSet::with_capacity(grid.len() * grid[0].len());
    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'.' {
                grid[r][c] = b'#';
                let (mut pos, mut dir) = (start, 0);
                while let Some((nr, nc)) = next_position(&grid, &pos, &ORTHOGONAL_DIRECTIONS[dir]) {
                    if !visited.insert((pos, dir)) {
                        count += 1;
                        break;
                    }
                    if grid[nr][nc] == b'#' {
                        dir = (dir + 1) % ORTHOGONAL_DIRECTIONS.len();
                    } else {
                        pos = (nr, nc);
                    }
                }
                grid[r][c] = b'.';
                visited.clear();
            }
        }
    }
    count.try_into().ok()
}

#[inline]
fn next_position<T: Copy>(
    map: &[Vec<T>],
    guard: &(usize, usize),
    dir: &(isize, isize),
) -> Option<(usize, usize)> {
    let nr = guard.0.checked_add_signed(dir.0)?;
    let nc = guard.1.checked_add_signed(dir.1)?;
    if nr < map.len() && nc < map[nr].len() {
        Some((nr, nc))
    } else {
        None
    }
}

fn parse_input(input: &str) -> (Vec<Vec<u8>>, (usize, usize)) {
    let map: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == b'^' {
                return (map, (r, c));
            }
        }
    }
    unreachable!("Start position must be found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(6));
    }
}
