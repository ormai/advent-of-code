use std::{cmp::Ordering, collections::HashMap};

aoc::solution!(2024, 5);

/// Iterate through the updates and sum all the middle pages of each already ordered update.
pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input);
    Some(updates.iter().fold(0, |acc, update| {
        acc + if is_ordered(&rules, update) {
            update[update.len() / 2]
        } else {
            0
        }
    }))
}

/// Order the unordered updates. Then sum the middle pages of the rules that have been ordered.
pub fn part_two(input: &str) -> Option<u64> {
    let (rules, mut updates) = parse_input(input);
    Some(updates.iter_mut().fold(0, |acc, update| {
        if !is_ordered(&rules, update) {
            update.sort_unstable_by(|a, b| match rules.get(a).map(|rule| rule.contains(b)) {
                Some(true) => Ordering::Less,
                _ => Ordering::Equal,
            });
            acc + update[update.len() / 2]
        } else {
            acc
        }
    }))
}

fn is_ordered(rules: &HashMap<u64, Vec<u64>>, update: &[u64]) -> bool {
    for i in 0..update.len() {
        if let Some(rule) = rules.get(&update[i]) {
            for after in rule {
                if update[..i].contains(after) {
                    return false;
                }
            }
        }
    }
    true
}

fn parse_input(input: &str) -> (HashMap<u64, Vec<u64>>, Vec<Vec<u64>>) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            if line.contains("|") {
                let rule: Vec<_> = line
                    .split("|")
                    .map(|d| d.parse().expect("rules contain positive integers"))
                    .collect();
                rules.entry(rule[0]).or_insert(Vec::new()).push(rule[1]);
            } else {
                updates.push(
                    line.split(",")
                        .map(|d| d.parse().expect("updates contain comma separated integers"))
                        .collect(),
                );
            }
        }
    }

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", YEAR, DAY, Part::Both));
        assert_eq!(result, Some(123));
    }
}
