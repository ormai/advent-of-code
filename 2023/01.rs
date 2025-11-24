const INPUT: &str = include_str!("../input");

fn main() {
    let calibration_value: u32 = INPUT
        .lines()
        .map(|line| {
            line.chars()
                .find(|c| c.is_ascii_digit())
                .unwrap()
                .to_digit(10)
                .unwrap()
                * 10
                + line
                    .chars()
                    .rev()
                    .find(|c| c.is_ascii_digit())
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
        })
        .sum();
    println!("{calibration_value}");
}

fn find_digit() {
    
}
