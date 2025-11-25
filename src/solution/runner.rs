/// Encapsulates code that interacts with solution functions.
use std::fmt::Display;
use std::hint::black_box;
use std::io::{Write, stdout};
use std::time::{Duration, Instant};
use std::{cmp, env, process};

use crate::solution::{ANSI_BOLD, Part, aoc_client};
use crate::solution::{ANSI_ITALIC, ANSI_RESET};

pub fn run_part<I: Copy, T: Display>(
    func: impl Fn(I) -> Option<T>,
    input: I,
    year: i32,
    day: u32,
    part: Part,
) {
    let part_str = format!("Part {part}");

    let (result, duration, samples) =
        run_timed(func, input, |result| print_result(result, &part_str, ""));

    print_result(&result, &part_str, &format_duration(&duration, samples));

    if let Some(result) = result {
        submit_result(result, year, day, part);
    }
}

/// Run a solution part. The behavior differs depending on whether we are running a release or debug build:
///  1. in debug, the function is executed once.
///  2. in release, the function is benched (approx. 1 second of execution time or 10 samples, whatever take longer.)
fn run_timed<I: Copy, T>(
    func: impl Fn(I) -> T,
    input: I,
    hook: impl Fn(&T),
) -> (T, Duration, u128) {
    let timer = Instant::now();
    let result = {
        #[cfg(feature = "dhat-heap")]
        let _profiler = dhat::Profiler::new_heap();

        func(input)
    };
    let base_time = timer.elapsed();

    hook(&result);

    let run = if std::env::args().any(|x| x == "--time") {
        bench(func, input, &base_time)
    } else {
        (base_time, 1)
    };

    (result, run.0, run.1)
}

fn bench<I: Copy, T>(func: impl Fn(I) -> T, input: I, base_time: &Duration) -> (Duration, u128) {
    let mut stdout = stdout();

    print!(" > {ANSI_ITALIC}benching{ANSI_RESET}");
    let _ = stdout.flush();

    let bench_iterations =
        (Duration::from_secs(1).as_nanos() / cmp::max(base_time.as_nanos(), 10)).clamp(10, 10000);

    let mut timers: Vec<Duration> = vec![];

    for _ in 0..bench_iterations {
        let timer = Instant::now();
        black_box(func(black_box(input)));
        timers.push(timer.elapsed());
    }

    (
        #[allow(clippy::cast_possible_truncation)]
        Duration::from_nanos(average_duration(&timers) as u64),
        bench_iterations,
    )
}

fn average_duration(numbers: &[Duration]) -> u128 {
    numbers
        .iter()
        .map(std::time::Duration::as_nanos)
        .sum::<u128>()
        / numbers.len() as u128
}

fn format_duration(duration: &Duration, samples: u128) -> String {
    if samples == 1 {
        format!(" ({duration:.1?})")
    } else {
        format!(" ({duration:.1?} @ {samples} samples)")
    }
}

fn print_result<T: Display>(result: &Option<T>, part: &str, duration_str: &str) {
    let is_intermediate_result = duration_str.is_empty();

    match result {
        Some(result) => {
            if result.to_string().contains('\n') {
                let str = format!("{part}: ▼ {duration_str}");
                if is_intermediate_result {
                    print!("{str}");
                } else {
                    print!("\r");
                    println!("{str}");
                    println!("{result}");
                }
            } else {
                let str = format!("{part}: {ANSI_BOLD}{result}{ANSI_RESET}{duration_str}");
                if is_intermediate_result {
                    print!("{str}");
                } else {
                    print!("\r");
                    println!("{str}");
                }
            }
        }
        None => {
            if is_intermediate_result {
                print!("{part}: ✖");
            } else {
                print!("\r");
                println!("{part}: ✖             ");
            }
        }
    }
}

/// Parse the arguments passed to `solve` and try to submit one part of the solution if we are in `--release` mode.
fn submit_result<T: Display>(result: T, year: i32, day: u32, part: Part) {
    let args: Vec<String> = env::args().collect();

    if !args.contains(&"--submit".into()) {
        return;
    }

    if args.len() < 3 {
        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
        process::exit(1);
    }

    let part_index = args.iter().position(|x| x == "--submit").unwrap() + 1;

    let Ok(part_submit) = args[part_index].parse::<Part>() else {
        eprintln!("Unexpected command-line input. Format: cargo solve 1 --submit 1");
        process::exit(1);
    };

    if part_submit != part {
        return;
    }

    println!("Submitting result...");

    let part = match part {
        Part::Both => panic!("Cannot send both parts"),
        Part::One => 1,
        Part::Two => 2,
    };
    aoc_client(year, day)
        .expect("Failed to create AocClient")
        .submit_answer_and_show_outcome(part, result)
        .expect("failed to submit result")
}
