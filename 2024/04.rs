const INPUT: &str = include_str!("input");
const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (1, 1),
    (0, 1),
    (1, -1),
    (1, 0),
    (-1, -1),
    (0, -1),
];

fn main() {
    let grid: Vec<Vec<char>> = parse_grid(INPUT);
    println!(
        "{}\n{}",
        word_search(&grid, "XMAS"),
        x_mas_word_search(&grid)
    );
}

fn x_mas_word_search(haystack: &[Vec<char>]) -> u32 {
    const WORD: [char; 3] = ['M', 'A', 'S'];
    const WORD_REV: [char; 3] = ['S', 'A', 'M'];
    let mut count = 0;
    for r in 0..haystack.len() {
        for c in 0..haystack[r].len() {
            if haystack[r][c] == 'A' {
                let get_diagonally = |dr, dc| {
                    in_bounds(haystack, r as isize + dr, c as isize + dc)
                        .map(|(nr, nc)| haystack[nr][nc])
                };
                if let (Some(ul), Some(lr), Some(ur), Some(ll)) = (
                    get_diagonally(-1, -1),
                    get_diagonally(1, 1),
                    get_diagonally(-1, 1),
                    get_diagonally(1, -1),
                ) {
                    let diagonal_matches = |diagonal| diagonal == WORD || diagonal == WORD_REV;
                    if diagonal_matches([ul, 'A', lr]) && diagonal_matches([ur, 'A', ll]) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn word_search(haystack: &[Vec<char>], needle: &str) -> u32 {
    let word: Vec<char> = needle.chars().collect();
    let mut count = 0;
    for r in 0..haystack.len() {
        for c in 0..haystack[r].len() {
            if haystack[r][c] == word[0] {
                for (dr, dc) in DIRECTIONS {
                    let mut candidate = Vec::with_capacity(needle.len() - 1);
                    let (mut r, mut c) = (r, c);
                    for _ in 0..word.len() - 1 {
                        if let Some((nr, nc)) =
                            in_bounds(haystack, r as isize + dr, c as isize + dc)
                        {
                            (r, c) = (nr, nc);
                            candidate.push(haystack[r][c]);
                        } else {
                            break;
                        }
                    }
                    if candidate == word[1..] {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn in_bounds(grid: &[Vec<char>], r: isize, c: isize) -> Option<(usize, usize)> {
    if r >= 0 && c >= 0 {
        let (r, c) = (r as usize, c as usize);
        if r < grid.len() && c < grid[r].len() {
            return Some((r, c));
        }
    }
    None
}

fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod test {
    use crate::{parse_grid, word_search, x_mas_word_search};

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn word_search_finds_all_xmases_in_example() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(word_search(&grid, "XMAS"), 18);
    }

    #[test]
    fn x_word_search_fins_all_x_mases_in_example() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(x_mas_word_search(&grid), 9);
    }
}
