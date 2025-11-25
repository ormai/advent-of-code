aoc::solution!(2024, 1);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);
    Some(left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input);
    let mut right_counts = HashMap::new();
    for num in right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    Some(
        left.iter()
            .map(|&n| n * right_counts.get(&n).unwrap_or(&0))
            .sum(),
    )
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<u64>) {
    let (mut left, mut right) = (Vec::new(), Vec::new());
    for line in input.lines() {
        let mut numbers = line
            .split_whitespace()
            .map(|n| n.parse().expect("input should be only numbers"));
        left.push(numbers.next().expect("first split"));
        right.push(numbers.next().expect("second split"));
    }
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(31));
    }
}
