aoc::solution!(2021, 2);

pub fn part_one(input: &str) -> Option<u64> {
    let (pos, depth) = command(input).fold((0, 0), |(pos, depth), (action, amount)| match action {
        b'f' => (pos + amount, depth),
        b'd' => (pos, depth + amount),
        b'u' => (pos, depth - amount),
        _ => unreachable!("there are only three commands"),
    });
    Some(pos * depth)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (pos, depth, _) =
        command(input).fold(
            (0, 0, 0),
            |(pos, depth, aim), (action, amount)| match action {
                b'f' => (pos + amount, depth + aim * amount, aim),
                b'd' => (pos, depth, aim + amount),
                b'u' => (pos, depth, aim - amount),
                _ => unreachable!("there are only three commands"),
            },
        );
    Some(pos * depth)
}

fn command(input: &str) -> impl Iterator<Item = (&u8, u64)> {
    input.lines().map(|line| {
        let (action, amount) = line
            .trim_end()
            .split_once(' ')
            .expect("each line contain an action and a number separated by a space");
        (
            action.as_bytes().first().unwrap(),
            amount.parse().expect("small positive integer"),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(150));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(900));
    }
}
