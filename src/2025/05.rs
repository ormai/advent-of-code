aoc::solution!(2025, 5);

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh_ingredients, available_ingredients) = input.split_once("\n\n")?;
    let fresh_ingredients: Vec<(u64, u64)> = fresh_ingredients
        .lines()
        .filter_map(|line| {
            line.split_once('-')
                .and_then(|(start, end)| Some((start.parse().ok()?, end.parse().ok()?)))
        })
        .collect();

    Some(
        available_ingredients
            .lines()
            .skip(1)
            .fold(0, |acc, ingredient| {
                let ingredient: u64 = ingredient.parse().unwrap();
                if fresh_ingredients
                    .iter()
                    .any(|&(start, end)| start <= ingredient && ingredient <= end)
                {
                    acc + 1
                } else {
                    acc
                }
            }),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<(u64, u64)> = input
        .lines()
        .skip_while(|line| line.is_empty())
        .map_while(|line| {
            line.split_once('-')
                .and_then(|(start, end)| Some((start.parse().ok()?, end.parse().ok()?)))
        })
        .collect();
    ranges.sort_unstable_by_key(|&(start, _)| start);

    let mut non_overlapping = Vec::with_capacity(ranges.len());
    non_overlapping.push(ranges[0]);
    for &(start, end) in &ranges[1..] {
        let last = non_overlapping.last_mut()?;
        if start <= last.1 {
            last.1 = last.1.max(end)
        } else {
            non_overlapping.push((start, end))
        }
    }
    Some(
        non_overlapping
            .into_iter()
            .fold(0, |acc, (start, end)| acc + end - start + 1),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(14));
    }
}
