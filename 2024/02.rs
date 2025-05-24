use std::cmp::Ordering;

const INPUT: &str = include_str!("input");

fn main() {
    let mut safe_reports = 0;
    let mut dampened = 0;
    for line in INPUT.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().expect("input contains only integers"))
            .collect();
        if is_safe(&levels) {
            safe_reports += 1;
        } else if can_dampen(&levels) {
            dampened += 1;
        }
    }
    println!("{safe_reports}");
    println!("{}", safe_reports + dampened);
}

fn can_dampen(levels: &[i32]) -> bool {
    (0..levels.len()).any(|idx| {
        let n_removed: Vec<i32> = levels
            .iter()
            .enumerate()
            .filter_map(|(i, &level)| if i != idx { Some(level) } else { None })
            .collect();
        is_safe(&n_removed)
    })
}

fn is_safe(levels: &[i32]) -> bool {
    distance_is_in_range(levels)
        && (monotony(levels, Ordering::Less) || monotony(levels, Ordering::Greater))
}

fn monotony(levels: &[i32], tonality: Ordering) -> bool {
    levels.windows(2).all(|w| w[0].cmp(&w[1]) == tonality)
}

fn distance_is_in_range(levels: &[i32]) -> bool {
    levels
        .windows(2)
        .all(|w| (1..=3).contains(&w[0].abs_diff(w[1])))
}

#[test]
fn is_safe_works() {
    assert!(is_safe(&[7, 6, 4, 2, 1]));
    assert!(!is_safe(&[1, 2, 7, 8, 9]));
    assert!(!is_safe(&[9, 7, 6, 2, 1]));
    assert!(!is_safe(&[1, 3, 2, 4, 5]));
    assert!(!is_safe(&[8, 6, 4, 4, 1]));
    assert!(is_safe(&[1, 3, 6, 7, 9]));
}
