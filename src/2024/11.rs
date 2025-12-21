use memoize::memoize;

aoc::solution!(2024, 11);

pub fn part_one(input: &str) -> Option<u64> {
    Some(blink(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(blink(input, 75))
}

/// Counts the total stones that there will be after blinking `times` times.
fn blink(input: &str, times: u8) -> u64 {
    parse_stones(input)
        .map(|stone| blink_once(stone, times))
        .sum()
}

/// Counts how many stones there will be after blinking `step` times.
#[memoize]
fn blink_once(stone: u64, step: u8) -> u64 {
    if step == 0 {
        1
    } else if stone == 0 {
        blink_once(1, step - 1)
    } else {
        let digits = count_digits(stone);
        if digits.is_multiple_of(2) {
            let lower_mask = 10u64.pow(u32::from(digits / 2));
            blink_once(stone / lower_mask, step - 1) + blink_once(stone % lower_mask, step - 1)
        } else {
            blink_once(stone * 2024, step - 1)
        }
    }
}

fn count_digits(mut n: u64) -> u8 {
    let mut digits = 0;
    while n > 0 {
        digits += 1;
        n /= 10;
    }
    digits
}

fn parse_stones(input: &str) -> impl Iterator<Item = u64> {
    input.split_whitespace().filter_map(|n| n.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blinking() {
        assert_eq!(blink("125 17", 6), 22);
        assert_eq!(blink("125 17", 25), 55312);
    }
}
