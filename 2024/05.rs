use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let (rules, mut updates) = parse_input(include_str!("input"));
    println!(
        "{}\n{}",
        middle_pages_of_already_ordered_updates(&rules, &updates),
        middle_pages_after_ordering(&rules, &mut updates)
    );
}

// Part two.
// Order the unordered updates as per the rules.
// Then sum the middle pages of the rules that have been ordered.
fn middle_pages_after_ordering(rules: &HashMap<u32, Vec<u32>>, updates: &mut [Vec<u32>]) -> u32 {
    updates.iter_mut().fold(0, |acc, update| {
        if !is_ordered(rules, update) {
            update.sort_unstable_by(|a, b| {
                if let Some(rule) = rules.get(a) {
                    if rule.contains(b) {
                        return Ordering::Less;
                    }
                }
                Ordering::Equal
            });
            return acc + update[update.len() / 2];
        }
        acc
    })
}

/// Part one.
/// Iterate through the updates and sum all the
/// middle pages of each already ordered update.
fn middle_pages_of_already_ordered_updates(
    rules: &HashMap<u32, Vec<u32>>,
    updates: &[Vec<u32>],
) -> u32 {
    updates.iter().fold(0, |acc, update| {
        acc + if is_ordered(rules, update) {
            update[update.len() / 2]
        } else {
            0
        }
    })
}

fn is_ordered(rules: &HashMap<u32, Vec<u32>>, update: &[u32]) -> bool {
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

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();

    for line in input.lines() {
        if !line.is_empty() {
            if line.contains("|") {
                let rule: Vec<u32> = line
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
mod test {
    use crate::{
        middle_pages_after_ordering, middle_pages_of_already_ordered_updates, parse_input,
    };

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part_one() {
        let (rules, updates) = parse_input(EXAMPLE);
        assert_eq!(
            middle_pages_of_already_ordered_updates(&rules, &updates),
            143
        );
    }

    #[test]
    fn part_two() {
        let (rules, mut updates) = parse_input(EXAMPLE);
        assert_eq!(middle_pages_after_ordering(&rules, &mut updates), 123);
    }
}
