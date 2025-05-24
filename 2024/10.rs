use std::collections::{HashSet, VecDeque};

const INPUT: &str = include_str!("input");

fn main() {
    let grid = parse_grid();
    let (score, rating) = walk_the_trail(&grid);
    println!("{score}");
    println!("{rating}");
}

fn walk_the_trail(grid: &Vec<Vec<u32>>) -> (u32, u32) {
    let size = grid.len();
    let mut score = 0;
    let mut rating = 0;

    for r in 0..size {
        for c in 0..size {
            if grid[r][c] == 0 {
                let mut q = VecDeque::from([(r, c)]);
                let mut visited = HashSet::new();

                while !q.is_empty() {
                    let (i, j) = q.pop_front().expect("queue is not empty");
                    if grid[i][j] == 9 {
                        if visited.insert((i, j)) {
                            score += 1
                        }
                        rating += 1
                    } else {
                        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                            let (nr, nc) = (dr + i as i32, dc + j as i32);
                            if 0 <= nr && 0 <= nc {
                                let (nr, nj) = (nr as usize, nc as usize);
                                if nr < size && nj < size && grid[nr][nj] == grid[i][j] + 1 {
                                    q.push_back((nr, nj));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (score, rating)
}

fn parse_grid() -> Vec<Vec<u32>> {
    INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("input contains only digits"))
                .collect()
        })
        .collect()
}
