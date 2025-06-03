use memoize::memoize;
use num_integer::Integer;

fn main() {
    let stones: Vec<u64> = include_str!("input")
        .trim_end()
        .split(" ")
        .map(|n| n.parse().expect("Input contains only positive integers"))
        .collect();
    println!("{}\n{}", blink(&stones, 25), blink(&stones, 75));
}

/// Counts the total stones that there will be after blinking `times` times.
fn blink(stones: &[u64], times: u8) -> u64 {
    stones.iter().map(|&stone| blink_once(stone, times)).sum()
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
            let (first_half, second_half) = stone.div_rem(&10u64.pow(u32::from(digits / 2)));
            blink_once(first_half, step - 1) + blink_once(second_half, step - 1)
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

#[cfg(test)]
mod tests {
    use super::blink;

    #[test]
    fn blink_works() {
        assert_eq!(blink(&[125, 17], 6), 22);
        assert_eq!(blink(&[125, 17], 25), 55312);
    }
}
