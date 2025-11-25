use aoc::solution::days;
use aoc::solution::{aoc_client, readme_benchmarks, run_multi::run_multi, timings::Timings};
use aoc_client::AocClient;
use chrono::{Datelike, FixedOffset, Utc};
use clap::{ArgAction, Command, arg, command, value_parser};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::process;
use std::{collections::HashSet, process::Stdio};

fn main() {
    let matches = cli().get_matches();

    let puzzle = Puzzle::new(
        matches.get_one::<i32>("year").copied(),
        matches.get_one::<u32>("day").copied(),
    );

    match matches.subcommand() {
        Some(("solve", sub_matches)) => {
            puzzle.solve(
                sub_matches.get_flag("release"),
                sub_matches.get_flag("dhat"),
                sub_matches.get_one("submit").copied(),
            );
        }
        Some(("scaffold", _)) => puzzle.scaffold(),
        Some(("all", sub_matches)) => {
            run_multi(&days().collect(), sub_matches.get_flag("release"), false);
        }
        Some(("time", sub_matches)) => {
            puzzle.time(sub_matches.get_flag("all"), sub_matches.get_flag("store"))
        }
        _ => unreachable!(),
    }
}

/// Command Line Interface
fn cli() -> Command {
    command!()
        .about("Advent of Code helper")
        .subcommand_required(true)
        .arg(
            arg!(year: -y --year <YEAR> "Puzzle year, if missing current year is used")
                .value_parser(value_parser!(i32).range(2015..=2025)),
        )
        .arg(
            arg!(day: -d --day <DAY> "Puzzle day, if missing current day is used")
                .value_parser(value_parser!(u32).range(1..=25)),
        )
        .subcommand(
            Command::new("solve")
                .about("Test a solution for correctness")
                .arg(arg!(release: -r --release "Enable release mode").action(ArgAction::SetTrue))
                .arg(
                    arg!(submit: -s --submit <PART> "Submit one part of the solution")
                        .value_parser(value_parser!(u8).range(1..=2)),
                )
                .arg(arg!(dhat: -t --dhat "Analyze heap allocations").action(ArgAction::SetTrue)),
        )
        .subcommand(
            Command::new("scaffold")
                .about("Create new solution file from template and download input"),
        )
        .subcommand(
            Command::new("all")
                .about("Run all solutions")
                .arg(arg!(-r --release "Enable release mode")),
        )
        .subcommand(
            Command::new("time")
                .about("Benchmark solutions")
                .arg(arg!(-a --all "Benchmark all solutions"))
                .arg(arg!(-s --store "Add benchmark results to the README.md")),
        )
}

/// A puzzle in Advent of Code
struct Puzzle {
    year: i32,
    day: u32,
    client: AocClient,
}

impl Puzzle {
    /// Creates a new puzzle given optional year and optional date
    fn new(year: Option<i32>, day: Option<u32>) -> Self {
        // Advent of Code uses the UTC-5 timezone.
        let now = Utc::now().with_timezone(&FixedOffset::east_opt(5 * 3600).unwrap());
        let year = year.unwrap_or_else(|| now.year());
        let day = day.unwrap_or_else(|| {
            if now.month() != 12 {
                eprintln!("It's not December, you must provide a day with -d or --day");
                process::exit(1);
            }
            now.day()
        });

        if year == 2025 && day > 12 {
            eprintln!("2025 has only 12 puzzles");
            process::exit(1);
        }

        let client = match aoc_client(year, day) {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to create AocClient: {e}");
                process::exit(1);
            }
        };

        Self { year, day, client }
    }

    /// Runs a solution against the expamples
    fn solve(&self, release: bool, dhat: bool, submit_part: Option<u8>) {
        let mut args = vec![
            "run".to_string(),
            "--bin".to_string(),
            format!("{}-{:02}", self.year, self.day),
        ];

        if dhat {
            args.extend([
                "--profile".to_string(),
                "dhat".to_string(),
                "--features".to_string(),
                "dhat-heap".to_string(),
            ]);
        } else if release {
            args.push("--release".to_string());
        }

        args.push("--".to_string());

        if let Some(submit_part) = submit_part {
            args.push("--submit".to_string());
            args.push(submit_part.to_string());
        }

        let mut cmd = process::Command::new("cargo")
            .args(&args)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        cmd.wait().unwrap();
    }

    /// Creates the solution file from a template. Downloads the input, the example, and the puzzle description.
    ///
    /// Adds a new `[[bin]]` table to Cargo.toml.
    /// Solution: src/{year}/{day}.rs
    /// Input: data/inputs/{year}/{day}.txt
    /// Puzzle description: data/puzzles/{year}/{day}.md
    ///
    /// If run more than once on a puzzle it will overwrite puzzle description and input. Examples, solution, and
    /// Cargo.toml are not touched.
    fn scaffold(&self) {
        let solution_path = format!("src/{}/{:02}.rs", self.year, self.day);

        write_file(
            &solution_path,
            &"aoc::solution!({{YEAR}}, {{DAY}});

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&read_file(\"examples\", YEAR, DAY, Part::One));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file(\"examples\", YEAR, DAY, Part::Two));
        assert_eq!(result, None);
    }
}
"
            .replace("{{DAY}}", &self.day.to_string())
            .replace("{{YEAR}}", &self.year.to_string()),
            false,
        );

        fs::create_dir_all(format!("data/inputs/{}", self.year))
            .expect("Ancestors should be created");
        self.client.save_input().expect("failed to download input");
        fs::create_dir_all(format!("data/puzzles/{}", self.year))
            .expect("Ancestors should be created");
        self.client
            .save_puzzle_markdown()
            .expect("failed to download puzzle");

        let mut contents = String::new();
        File::open("Cargo.toml")
            .expect("failed to open Cargo.toml")
            .read_to_string(&mut contents)
            .expect("failed to read Cargo.toml");
        if !contents.contains(&format!("{}-{:02}", self.year, self.day)) {
            OpenOptions::new()
                .append(true)
                .open("Cargo.toml")
                .expect("failed to open Cargo.toml")
                .write_all(
                    "
[[bin]]
name = \"{{YEAR}}-{{DAY}}\"
path = \"src/{{YEAR}}/{{DAY}}.rs\"
"
                    .replace("{{DAY}}", &format!("{:02}", self.day))
                    .replace("{{YEAR}}", &self.year.to_string())
                    .as_bytes(),
                )
                .expect("failed to append to Cargo.toml");
        }
    }

    fn time(&self, run_all: bool, store: bool) {
        let stored_timings = Timings::read_from_file();

        let days_to_run = if run_all {
            days().collect()
        } else {
            HashSet::from([self.day])
            // when the `--all` flag is not set, filter out days that are fully benched.
            // all_days()
            //     .filter(|day| !stored_timings.is_day_complete(*day))
            //     .collect()
        };

        let timings = run_multi(&days_to_run, true, true).unwrap();

        if store {
            let merged_timings = stored_timings.merge(&timings);
            merged_timings.store_file().unwrap();
            match readme_benchmarks::update(merged_timings) {
                Ok(()) => println!("Stored updated benchmarks."),
                Err(_) => eprintln!("Failed to store updated benchmarks."),
            }
        }
    }
}

fn write_file(path_str: &str, contents: &str, replace: bool) {
    let path = Path::new(path_str);

    if path.exists() && !replace {
        println!("Skipping file creation: '{path_str}' exists");
        return;
    }

    fs::create_dir_all(path.parent().expect("Path should have a parent"))
        .expect("Ancestors should be created");

    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .expect("new file should be created")
        .write_all(contents.as_bytes())
        .expect("content should be written");
    println!("Created file '{path_str}'");
}
