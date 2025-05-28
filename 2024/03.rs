use regex::Regex;
use std::error::Error;

const INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;
    let command = Regex::new(r"(don't\(\)|do\(\))")?;

    let (mut total, mut conditional) = (0, 0);
    let mut enabled = true;
    for line in INPUT.lines() {
        let mut prev_match_end = 0;
        for (range, factors) in mul
            .captures_iter(line)
            .map(|c| (c.get(0).unwrap().range(), c.extract::<2>().1))
        {
            let product: i32 = factors.iter().map(|f| f.parse::<i32>().unwrap()).product();
            total += product;

            if let Some(command_match) = command.captures(&line[prev_match_end..range.start]) {
                enabled = command_match.get(0).ok_or("")?.as_str() == "do()";
            }
            if enabled {
                conditional += product;
            }
            prev_match_end = range.end;
        }
    }

    println!("{total}\n{conditional}");
    Ok(())
}
