use itertools::Itertools;
use std::{
    iter,
    ops::{Add, Mul},
};

aoc::solution!(2024, 7);

pub fn part_one(input: &str) -> Option<u64> {
    Some(total_calibration(
        parse_equation(input),
        vec![Add::add, Mul::mul],
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(total_calibration(
        parse_equation(input),
        vec![Add::add, Mul::mul, merge],
    ))
}

fn total_calibration(equation: Vec<Vec<u64>>, operators: Vec<fn(u64, u64) -> u64>) -> u64 {
    equation
        .into_iter()
        .filter_map(|nums| {
            if iter::repeat_n(operators.iter(), nums.len() - 2)
                .multi_cartesian_product()
                .any(|ops| {
                    nums[2..]
                        .iter()
                        .zip(ops.iter())
                        .fold(nums[1], |acc, (&a, op)| op(acc, a))
                        == nums[0]
                })
            {
                Some(nums[0])
            } else {
                None
            }
        })
        .sum()
}

fn merge(a: u64, b: u64) -> u64 {
    let mut digits = 10;
    while b >= digits {
        digits *= 10;
    }
    a * digits + b
}

fn parse_equation(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.replace(":", "")
                .split_whitespace()
                .map(|n| n.parse().expect("input contains valid integers"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(11387));
    }
}
