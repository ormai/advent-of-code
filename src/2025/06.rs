aoc::solution!(2025, 6);

pub fn part_one(input: &str) -> Option<u64> {
    let problems: Vec<u64> = input
        .lines()
        .flat_map(|line| line.split_whitespace().map_while(|line| line.parse().ok()))
        .collect();

    let ops = parse_operators(input)?;
    let columns = ops.len();

    Some(
        (0..columns)
            .zip(ops.iter())
            .map(|(c, op)| {
                let column = problems.iter().skip(c).step_by(columns);
                match op {
                    b'+' => column.sum(),
                    b'*' => column.product(),
                    _ => 0,
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let ops = parse_operators(input)?;

    let lines: Vec<_> = input.lines().collect();
    let mat = transpose(
        &lines[..lines.len() - 1]
            .iter()
            .map(|line| line.bytes().collect())
            .collect::<Vec<_>>(),
    );

    Some(
        mat.split(|row| row.iter().all(|&c| c == b' '))
            .zip(ops.iter())
            .map(|(col, op)| {
                let col = col.iter().filter_map(|dirty_num| {
                    dirty_num
                        .iter()
                        .filter_map(|byte| {
                            if byte.is_ascii_digit() {
                                Some(u64::from(byte - b'0'))
                            } else {
                                None
                            }
                        })
                        .reduce(|acc, d| acc * 10 + d)
                });
                match op {
                    b'+' => col.sum(),
                    b'*' => col.product(),
                    _ => 0,
                }
            })
            .sum(),
    )
}

/// Transposes a rectangular matrix naively. Returns a new matrix.
fn transpose<T: Copy>(mat: &[Vec<T>]) -> Vec<Vec<T>> {
    if mat.is_empty() {
        vec![]
    } else {
        let (width, height) = (mat[0].len(), mat.len()); // Of the input matrix
        let mut transposed = Vec::with_capacity(width);
        for c in 0..width {
            let mut new_row = Vec::with_capacity(height);
            for old_row in mat {
                new_row.push(old_row[c]);
            }
            transposed.push(new_row);
        }
        transposed
    }
}

/// Parses the last line of the input, returning a Vec conaining only b'+' or b'*'
fn parse_operators(input: &str) -> Option<Vec<u8>> {
    input
        .lines()
        .last()
        .map(|last_line| last_line.split_whitespace().flat_map(str::bytes).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(3263827));
    }
}
