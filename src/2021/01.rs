aoc::solution!(2021, 1);

pub fn part_one(input: &str) -> Option<u64> {
    count_increases_between_subsequent_windows(input, 2)
}

pub fn part_two(input: &str) -> Option<u64> {
    count_increases_between_subsequent_windows(input, 4)
}

fn count_increases_between_subsequent_windows(input: &str, win_size: usize) -> Option<u64> {
    input
        .lines()
        .map(|line| line.trim_end().parse().unwrap())
        .collect::<Vec<_>>()
        .windows(win_size)
        .filter(|win| win[..win_size - 1].iter().sum::<u32>() < win[1..].iter().sum())
        .count()
        .try_into()
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(5));
    }
}
