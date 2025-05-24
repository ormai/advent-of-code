use itertools::Itertools;
use std::iter::repeat;

const INPUT: &str = include_str!("input");

fn main() {
    let mut input: Vec<Vec<u64>> = Vec::with_capacity(INPUT.lines().count());
    for line in INPUT.lines() {
        input.push(
            line.replace(":", "")
                .split_whitespace()
                .map(|n| n.parse().expect("input contains valid integers"))
                .collect(),
        );
    }

    let mut ops = vec![|a: u64, b: u64| a + b, |a: u64, b: u64| a * b];
    println!("{}", total_calibration(&input, &ops));

    ops.push(join_nums);
    println!("{}", total_calibration(&input, &ops)); // Slow
}

fn join_nums(a: u64, b: u64) -> u64 {
    let mut digits = 10;
    while b >= digits {
        digits *= 10;
    }
    a * digits + b
}

fn total_calibration(equation: &[Vec<u64>], operators: &[impl Fn(u64, u64) -> u64]) -> u64 {
    equation
        .iter()
        .filter_map(|nums| {
            if repeat(operators.iter())
                .take(nums.len() - 2)
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
