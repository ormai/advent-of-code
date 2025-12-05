use std::cmp::Ordering;

aoc::solution!(2024, 2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(reports(input).filter(|r| is_safe(r)).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(reports(input).filter(|r| can_dampen(r)).count() as u64)
}

fn is_safe(levels: &[u64]) -> bool {
    distance_in_range(levels)
        && (is_monotone(levels, Ordering::Less) || is_monotone(levels, Ordering::Greater))
}

fn can_dampen(levels: &[u64]) -> bool {
    (0..levels.len()).any(|idx| {
        let n_removed: Vec<_> = levels
            .iter()
            .enumerate()
            .filter_map(|(i, &level)| if i != idx { Some(level) } else { None })
            .collect();
        is_safe(&n_removed)
    })
}

fn reports(input: &str) -> impl Iterator<Item = Vec<u64>> {
    input.lines().map(|line| {
        line.split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect()
    })
}

fn is_monotone(levels: &[u64], tonality: Ordering) -> bool {
    levels.windows(2).all(|w| w[0].cmp(&w[1]) == tonality)
}

fn distance_in_range(levels: &[u64]) -> bool {
    levels
        .windows(2)
        .all(|w| matches!(w[0].abs_diff(w[1]), 1..=3))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(4));
    }
}
