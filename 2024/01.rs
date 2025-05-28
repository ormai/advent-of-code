use std::collections::HashMap;

const INPUT: &str = include_str!("input");

fn main() {
    let (mut left, mut right) = (Vec::new(), Vec::new());
    for line in INPUT.lines() {
        let mut numbers = line
            .split_whitespace()
            .map(|n| n.parse().expect("input should be only numbers"));
        left.push(numbers.next().expect("first split"));
        right.push(numbers.next().expect("second split"));
    }
    left.sort_unstable();
    right.sort_unstable();

    println!(
        "{}\n{}",
        distance(&left, &right),
        similarity_score(&left, &right)
    );
}

fn distance(left: &[i32], right: &[i32]) -> u32 {
    left.iter().zip(right).map(|(l, &r)| l.abs_diff(r)).sum()
}

fn similarity_score(left: &[i32], right: &[i32]) -> i32 {
    let mut right_counts = HashMap::new();
    for &num in right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    left.iter()
        .map(|&n| n * right_counts.get(&n).unwrap_or(&0))
        .sum()
}
