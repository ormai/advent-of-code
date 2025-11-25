use aoc_client::{AocClient, AocResult};
use std::{env, fmt::Display, fs, ops::RangeInclusive, str::FromStr};

pub mod readme_benchmarks;
pub mod run_multi;
pub mod runner;
pub mod timings;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

pub fn aoc_client(year: i32, day: u32) -> AocResult<AocClient> {
    AocClient::builder()
        .session_cookie_from_default_locations()?
        .year(year)?
        .day(day)?
        .input_filename(format!("data/inputs/{year}/{day:02}.txt"))
        .puzzle_filename(format!("data/puzzles/{year}/{day:02}.md"))
        .overwrite_files(true)
        .build()
}

#[derive(PartialEq)]
pub enum Part {
    Both,
    One,
    Two,
}

impl Part {
    fn path_suffix(&self) -> String {
        match self {
            Part::Both => "".to_string(),
            _ => format!("-{self}"),
        }
    }
}

/// Writes! the path component relative to a Puzzle Part
impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Part::Both => panic!("Not representable"),
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

pub fn days() -> RangeInclusive<u32> {
    // FIXME: knowledge of the year is necessary because 2025 has 12 instead of 25 puzzels
    0..=25
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "1" => Ok(Part::One),
            "2" => Ok(Part::Two),
            _ => Err(format!("Cannot build part from string: {s}")),
        }
    }
}

/// Reads a text file to a string. The part suffix is appended. E.g. like `01-2.txt`.
#[must_use]
pub fn read_file(folder: &str, year: i32, day: u32, part: Part) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(year.to_string())
        .join(format!("{day:02}{}.txt", part.path_suffix()));
    fs::read_to_string(filepath).expect("Could not open input file")
}

/// Creates the constants `YEAR`, and `DAY` and sets up the input and runner for each part.
///
/// The optional, third parameter (`Part`) allows you to only run a single part of the solution.
#[macro_export]
macro_rules! solution {
    ($year:expr, $day:expr) => {
        $crate::solution!(@impl $year, @impl $day, [part_one, Part::One] [part_two, Part::Two]);
    };
    ($year:expr, $day:expr, 1) => {
        $crate::solution!(@impl $year, @impl $day, [part_one, Part::One]);
    };
    ($year:expr, $day:expr, 2) => {
        $crate::solution!(@impl $year, @impl $day, [part_two, Part::Two]);
    };

    (@impl $year:expr, @impl $day:expr, $( [$func:expr, $part:expr] )*) => {
        use $crate::solution::{read_file, Part};

        /// The current year.
        const YEAR: i32 = $year;
        /// The current day.
        const DAY: u32 = $day;

        #[cfg(feature = "dhat-heap")]
        #[global_allocator]
        static ALLOC: dhat::Alloc = dhat::Alloc;

        fn main() {
            use $crate::solution::runner::*;
            let input = read_file("inputs", YEAR, DAY, Part::Both);
            $( run_part($func, &input, YEAR, DAY, $part); )*
        }
    };
}
