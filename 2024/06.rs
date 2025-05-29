use std::collections::HashSet;

type Point = (usize, usize);
type Direction = (isize, isize);

const DIRECTIONS: [Direction; 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn main() {
    let (mut map, start) = parse_map(include_str!("input"));
    println!(
        "{}\n{}",
        predict_guard_path(&map, start),
        obstruction_loops(&mut map, start)
    );
}

/// Part two. It's a bit slow.
fn obstruction_loops(map: &mut [Vec<char>], start: (usize, usize)) -> u32 {
    let mut count = 0;
    let mut visited = HashSet::new();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == '.' {
                map[r][c] = '#';
                let (mut pos, mut dir) = (start, 0);
                while let Some((nr, nc)) = next_position(map, &pos, &DIRECTIONS[dir]) {
                    if !visited.insert((pos, dir)) {
                        count += 1;
                        break;
                    }
                    if map[nr][nc] == '#' {
                        dir = (dir + 1) % DIRECTIONS.len();
                    } else {
                        pos = (nr, nc);
                    }
                }
                map[r][c] = '.';
                visited.clear();
            }
        }
    }
    count
}

/// Part one.
fn predict_guard_path(map: &[Vec<char>], start: (usize, usize)) -> usize {
    let (mut guard, mut dir, mut positions) = (start, 0, HashSet::from([start]));
    while let Some((nr, nc)) = next_position(map, &guard, &DIRECTIONS[dir]) {
        if map[nr][nc] == '#' {
            dir = (dir + 1) % DIRECTIONS.len();
        } else {
            positions.insert((nr, nc));
            guard = (nr, nc);
        }
    }
    positions.len()
}

fn next_position(map: &[Vec<char>], guard: &Point, dir: &Direction) -> Option<Point> {
    let (r, c) = (guard.0 as isize + dir.0, guard.1 as isize + dir.1);
    if r >= 0 && c >= 0 {
        let (r, c) = (r as usize, c as usize);
        if r < map.len() && c < map[r].len() {
            return Some((r, c));
        }
    }
    None
}

fn parse_map(input: &str) -> (Vec<Vec<char>>, (usize, usize)) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == '^' {
                return (map, (r, c));
            }
        }
    }
    unreachable!("Start position must be found");
}

#[cfg(test)]
mod test {
    use crate::{obstruction_loops, parse_map, predict_guard_path};

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part_one() {
        let (map, start) = parse_map(EXAMPLE);
        assert_eq!(predict_guard_path(&map, start), 41);
    }

    #[test]
    fn part_two() {
        let (mut map, start) = parse_map(EXAMPLE);
        assert_eq!(obstruction_loops(&mut map, start), 6);
    }
}
