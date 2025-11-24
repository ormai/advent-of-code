use aoc::template::{Day, all_days, readme_benchmarks, run_multi::run_multi, timings::Timings};
use aoc_client::{AocClient, AocResult};

// TODO: when the user does not provide year and day default to day and print error if not in december before the 25th
// TODO: replace the use of aoc-cli with https://crates.io/crates/aoc-client

use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process;
use std::{
    collections::HashSet,
    process::{Command, Stdio},
};

use crate::args::{Subcommand, parse};

mod args {
    use aoc::template::Day;
    use std::process;

    pub enum Subcommand {
        Scaffold {
            day: Day,
        },
        Solve {
            day: Day,
            release: bool,
            dhat: bool,
            submit: Option<u8>,
        },
        All {
            release: bool,
        },
        Time {
            all: bool,
            day: Option<Day>,
            store: bool,
        },
    }

    pub fn parse() -> Result<Subcommand, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        let app_args = match args.subcommand()?.as_deref() {
            Some("all") => Subcommand::All {
                release: args.contains("--release"),
            },
            Some("time") => {
                let all = args.contains("--all");
                let store = args.contains("--store");
                Subcommand::Time {
                    all,
                    day: args.opt_free_from_str()?,
                    store,
                }
            }
            // Some("read") => Subcommand::Read {
            //     day: args.free_from_str()?,
            // },
            Some("scaffold") => Subcommand::Scaffold {
                day: args.free_from_str()?,
            },
            Some("solve") => Subcommand::Solve {
                day: args.free_from_str()?,
                release: args.contains("--release"),
                submit: args.opt_value_from_str("--submit")?,
                dhat: args.contains("--dhat"),
            },
            Some(x) => {
                eprintln!("Unknown command: {x}");
                process::exit(1);
            }
            _ => {
                eprintln!("No command specified.");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {remaining:?}.");
        }

        Ok(app_args)
    }
}

fn aoc_client(year: i32, day: u32) -> AocResult<AocClient> {
    AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(year)?
        .day(day)?
        .build()
}

fn main() {
    match parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
        Ok(args) => match args {
            Subcommand::All { release } => all(release),
            Subcommand::Time { day, all, store } => time(day, all, store),
            // Subcommand::Read { day } => read(day),
            Subcommand::Scaffold { day } => {
                if let Err(e) = scaffold(day) {
                    eprintln!("Scaffold failed: {e}");
                    process::exit(1);
                }
            }
            Subcommand::Solve {
                day,
                release,
                dhat,
                submit,
            } => solve(day, release, dhat, submit),
        },
    };
}

fn all(is_release: bool) {
    run_multi(&all_days().collect(), is_release, false);
}

fn solve(day: Day, release: bool, dhat: bool, submit_part: Option<u8>) {
    let mut cmd_args = vec!["run".to_string(), "--bin".to_string(), day.to_string()];

    if dhat {
        cmd_args.extend([
            "--profile".to_string(),
            "dhat".to_string(),
            "--features".to_string(),
            "dhat-heap".to_string(),
        ]);
    } else if release {
        cmd_args.push("--release".to_string());
    }

    cmd_args.push("--".to_string());

    if let Some(submit_part) = submit_part {
        cmd_args.push("--submit".to_string());
        cmd_args.push(submit_part.to_string());
    }

    let mut cmd = Command::new("cargo")
        .args(&cmd_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    cmd.wait().unwrap();
}

fn time(day: Option<Day>, run_all: bool, store: bool) {
    let stored_timings = Timings::read_from_file();

    let days_to_run = day.map_or_else(
        || {
            if run_all {
                all_days().collect()
            } else {
                // when the `--all` flag is not set, filter out days that are fully benched.
                all_days()
                    .filter(|day| !stored_timings.is_day_complete(*day))
                    .collect()
            }
        },
        |day| HashSet::from([day]),
    );

    let timings = run_multi(&days_to_run, true, true).unwrap();

    if store {
        let merged_timings = stored_timings.merge(&timings);
        merged_timings.store_file().unwrap();

        println!();
        match readme_benchmarks::update(merged_timings) {
            Ok(()) => {
                println!("Stored updated benchmarks.");
            }
            Err(_) => {
                eprintln!("Failed to store updated benchmarks.");
            }
        }
    }
}

fn create_file(path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .create_new(true) // does not overwrite existing
        .truncate(true)
        .open(path)
}

/// Creates the solution file from a template. Downloads the input, the example, and the puzzle description.
fn scaffold(day: Day) -> Result<(), Box<dyn Error>> {
    let client = aoc_client(2024, day.into_inner().into()).unwrap();

    let solution_path = format!("src/bin/{day}.rs");
    create_file(&solution_path)?.write_all(
        "aoc::solution!(%DAY_NUMBER%);

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
        .replace("%DAY_NUMBER%", &day.into_inner().to_string())
        .as_bytes(),
    )?;
    println!("Created solution file '{solution_path}'");

    let input_path = format!("data/inputs/{day}.txt");
    create_file(&input_path)?.write_all(client.get_input()?.as_bytes())?;
    println!("Created input file '{input_path}'");

    // FIXME: this pulls the whole description
    let example_path = format!("data/examples/{day}.txt");
    create_file(&example_path)?.write_all(client.get_puzzle_html()?.as_bytes())?;
    println!("Created example file \"{}\"", &example_path);

    Ok(())
}
