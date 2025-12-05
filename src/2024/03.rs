use regex::{Captures, Regex};

aoc::solution!(2024, 3);

pub fn part_one(input: &str) -> Option<u64> {
    let mul = mul()?;
    Some(
        input
            .lines()
            .map(|line| mul.captures_iter(line).map(product).sum::<u64>())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mul = mul()?;
    let command = command()?;

    let mut acc = 0;
    let mut enabled = true;
    for line in input.lines() {
        let mut prev_match_end = 0;
        for captures in mul.captures_iter(line) {
            let range = captures.get(0)?.range();

            if let Some(command_match) = command.captures(&line[prev_match_end..range.start]) {
                enabled = command_match.get(0)?.as_str() == "do()";
            }
            if enabled {
                acc += product(captures)
            }
            prev_match_end = range.end
        }
    }
    Some(acc)
}

fn mul() -> Option<Regex> {
    Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").ok()
}

fn command() -> Option<Regex> {
    Regex::new(r"(don't\(\)|do\(\))").ok()
}

fn product(captures: Captures) -> u64 {
    captures
        .extract::<2>()
        .1
        .iter()
        .map(|factor| factor.parse::<u64>().unwrap())
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::One));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Two));
        assert_eq!(result, Some(48));
    }
}
