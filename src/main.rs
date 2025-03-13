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

    // Default number of runs is 1. Use -n <number> to override.
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
            String::from("./executable_goes_here"),
        ),
        "gcd" => (
            reference_gcd(&input),
            String::from("./executable_goes_here"),
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
        let path = entry.path();

        // Only process if the entry is a file.
        if path.is_file() {
            println!("Benchmarking executable: {:?}", path);
            let mut times = Vec::new();

            // Run the executable num_runs times.
            for run in 1..=num_runs {
                let start = Instant::now();
                let output = run_executable(&path, &input);
                let duration = start.elapsed();
                let duration_secs = duration.as_secs_f64();
                times.push(duration_secs);
                println!("Run {}: {:.6} seconds", run, duration_secs);

                // Validate output on each run (optional)
                if output.trim() != expected_output.trim() {
                    println!("❌ Output incorrect on run {}.", run);
                    println!("Expected Output:\n{}", expected_output);
                    println!("Actual Output:\n{}", output);
                }
            }

            // Compute statistics: average, min, max, and standard deviation.
            let sum: f64 = times.iter().sum();
            let count = times.len() as f64;
            let average = sum / count;
            let min = times.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let variance = times
                .iter()
                .map(|time| {
                    let diff = time - average;
                    diff * diff
                })
                .sum::<f64>()
                / count;
            let std_dev = variance.sqrt();

            println!("Summary for {:?}:", path);
            println!("Average: {:.6} seconds", average);
            println!("Min: {:.6} seconds", min);
            println!("Max: {:.6} seconds", max);
            println!("Std Dev: {:.6} seconds", std_dev);
            println!("----------------------------------");
        }
    }
}

/// Runs an external executable, piping `input` to its stdin and capturing stdout.
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
