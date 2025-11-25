use aoc::template::{all_days, readme_benchmarks, run_multi::run_multi, timings::Timings, Day};
use aoc_client::{AocClient, AocResult};
use chrono::{Datelike, FixedOffset, Utc};
use clap::{ArgAction, Command, arg, command, value_parser};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process;
use std::{collections::HashSet, process::Stdio};

// TODO: when the user does not provide year and day default to day and print error if not in december before the 25th
// TODO: replace the use of aoc-cli with https://crates.io/crates/aoc-client

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
            run_multi(
                &all_days().collect(),
                sub_matches.get_flag("release"),
                false,
            );
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

fn aoc_client(year: i32, day: u32) -> AocResult<AocClient> {
    AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(year)?
        .day(day)?
        .build()
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
    fn scaffold(&self) {
        let solution_path = format!("src/{}/{:02}.rs", self.year, self.day);

        write_file(
            &solution_path,
            &"aoc::solution!(%DAY_NUMBER%);

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
        let result = part_one(&aoc::template::read_file(\"examples\", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc::template::read_file(\"examples\", DAY));
        assert_eq!(result, None);
    }
}
"
            .replace("%DAY_NUMBER%", &self.day.to_string()),
            false,
        );
        println!("Created solution file '{solution_path}'");

        let input_path = format!("data/inputs/{}/{:02}.txt", self.year, self.day);
        write_file(
            &input_path,
            &self
                .client
                .get_input()
                .expect("AocClient should return input"),
            true,
        );
        println!("Created input file '{input_path}'");

        // FIXME: this pulls the whole description
        let example_path = format!("data/examples/{}/{:02}.txt", self.year, self.day);
        write_file(
            &example_path,
            &self
                .client
                .get_puzzle_html()
                .expect("AocClient should return puzzle html"),
            true,
        );
        println!("Created example file \"{}\"", &example_path);
    }

    fn time(&self, run_all: bool, store: bool) {
        let stored_timings = Timings::read_from_file();

        let days_to_run = if run_all {
            all_days().collect()
        } else {
            HashSet::from([Day::new(self.day as u8).unwrap()])
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

fn write_file(path: &str, contents: &str, replace: bool) {
    let path = Path::new(path);

    if path.exists() && !replace {
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
}
