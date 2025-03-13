use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

fn main() {
    // Usage: benchmark <challenge> [-n <num_runs>]
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [reverse|gcd] [-n <num_runs>]", args[0]);
        std::process::exit(1);
    }
    let challenge = &args[1];

    // Default number of runs is 1; override using -n <number>
    let mut num_runs: u32 = 1;
    let mut i = 2;
    while i < args.len() {
        if args[i] == "-n" {
            if i + 1 < args.len() {
                num_runs = args[i + 1]
                    .parse()
                    .expect("Invalid number provided after -n");
                i += 2;
            } else {
                eprintln!("Expected a number after -n");
                std::process::exit(1);
            }
        } else {
            i += 1;
        }
    }

    // Load input data from the file.
    let input =
        fs::read_to_string("./input/custom_wordlist.txt").expect("Failed to read the input file");

    // Compute expected output and set the executables directory.
    let (expected_output, executables_dir) = match challenge.as_str() {
        "reverse" => (
            reference_reverse(&input),
            String::from("./EXE_FILES_HERE/REVERSE_STRING"),
        ),
        "gcd" => (
            reference_gcd(&input),
            String::from("./EXE_FILES_HERE/GREATEST_COMMON_DIVISOR"),
        ),
        _ => {
            eprintln!("Unknown challenge: {}. Use 'reverse' or 'gcd'.", challenge);
            std::process::exit(1);
        }
    };

    // Iterate over executables in the given directory.
    let entries = fs::read_dir(executables_dir).expect("Failed to read executables directory");
    for entry in entries {
        let entry = entry.expect("Error reading a directory entry");
        let name = entry.file_name();
        if name == "PUT YOUR .EXE FILE IN THIS FOLDER.md" {
            continue;
        }
        let path = entry.path();

        if path.is_file() {
            println!("Benchmarking executable: {:?}", path);
            let mut times = Vec::new();
            let mut all_passed = true;

            // Run the executable num_runs times.
            for run in 1..=num_runs {
                let start = Instant::now();
                let output = run_executable(&path, &input);
                let duration = start.elapsed();
                let duration_secs = duration.as_secs_f64();
                times.push(duration_secs);

                let formatted_time = format_time(duration_secs);
                let pass = output.trim() == expected_output.trim();
                if !pass {
                    all_passed = false;
                }
                // Print run result: green if pass, red if fail.
                if pass {
                    // Green: \x1b[32m, Reset: \x1b[0m
                    println!("\x1b[32mRun {}: {}\x1b[0m", run, formatted_time);
                } else {
                    // Red: \x1b[31m, Reset: \x1b[0m
                    println!("\x1b[31mRun {}: {}\x1b[0m", run, formatted_time);
                    println!("❌ Output incorrect on run {}.", run);
                    println!("Expected Output:\n{}", expected_output);
                    println!("Actual Output:\n{}", output);
                }
            }

            // Compute summary statistics.
            let sum: f64 = times.iter().sum();
            let count = times.len() as f64;
            let average = sum / count;
            let min = times.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let variance = times.iter().map(|&t| (t - average).powi(2)).sum::<f64>() / count;
            let std_dev = variance.sqrt();

            // Print summary on one line with colors.
            println!(
                "Summary for {:?}: \n\x1b[36mAvg: {}\x1b[0m | \x1b[32mMin: {}\x1b[0m | \x1b[31mMax: {}\x1b[0m | \x1b[33mStd Dev: {}\x1b[0m",
                path,
                format_time(average),
                format_time(min),
                format_time(max),
                format_time(std_dev)
            );

            // Final pass/fail message.
            if all_passed {
                println!("\x1b[32m✅ Output correct on all runs.\x1b[0m");
            } else {
                println!("\x1b[31m❌ Some runs produced incorrect output.\x1b[0m");
            }
            println!("----------------------------------");
        }
    }
}

/// Helper to format time nicely. If less than one second, print in milliseconds.
fn format_time(seconds: f64) -> String {
    if seconds < 1.0 {
        format!("{:.4}ms", seconds * 1000.0)
    } else {
        format!("{:.4}s", seconds)
    }
}

/// Runs an external executable by piping `input` to its stdin and capturing stdout.
fn run_executable(path: &Path, input: &str) -> String {
    let mut child = Command::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    {
        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        child_stdin
            .write_all(input.as_bytes())
            .expect("Failed to write input to child process");
    }

    let output = child.wait_with_output().expect("Failed to capture output");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Reference implementation for the 'reverse' challenge.
fn reference_reverse(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Reference implementation for the 'gcd' challenge.
fn reference_gcd(input: &str) -> String {
    let numbers: Vec<i64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    if numbers.is_empty() {
        return "No valid numbers found".to_string();
    }
    let result = numbers.into_iter().reduce(gcd).unwrap();
    result.to_string()
}

/// Euclid's algorithm for computing the greatest common divisor.
fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
