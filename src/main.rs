use std::env;
use std::fs;
use std::time::Instant;
use uiasub_code_benchmark::input_generators::gcd_numbers::generate_gcd_numbers;
use uiasub_code_benchmark::input_generators::gcd_numbers::generate_gcd_numbers_large_capacity;
use uiasub_code_benchmark::input_generators::obelisk::gen_obelisks;
use uiasub_code_benchmark::input_generators::wordlist::wordlist;
use uiasub_code_benchmark::reference_gcd_large_capacity;
use uiasub_code_benchmark::reference_obelisk_count;
use uiasub_code_benchmark::{format_time, reference_gcd, reference_reverse, run_executable};

fn main() {
    // Usage: benchmark <challenge> [-n <num_runs>]
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} [reverse|reverse_large|gcd|gcd_hard|obelisk|obelisk_hard] [-n <num_runs>]",
            args[0]
        );
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
    let input;

    // Compute expected output and set the executables directory.
    let (expected_output, executables_dir) = match challenge.as_str() {
        "reverse" => {
            input = match fs::read_to_string("./input/wordlist_20K.txt") {
                Ok(s) => s,
                Err(_) => wordlist(20_000).expect("Failed to write custom wordlist."),
            };
            (
                reference_reverse(&input),
                String::from("./EXE_FILES_HERE/REVERSE_STRING"),
            )
        }

        "reverse_large" => {
            input = match fs::read_to_string("./input/wordlist_20M.txt") {
                Ok(s) => s,
                Err(_) => wordlist(20_000_000).expect("Failed to write custom wordlist."),
            };
            (
                reference_reverse(&input),
                String::from("./EXE_FILES_HERE/REVERSE_STRING"),
            )
        }
        "gcd" => {
            input = generate_gcd_numbers(20_000, 1_000_000).expect("Failed to generate numbers.");
            (
                reference_gcd(&input),
                String::from("./EXE_FILES_HERE/GREATEST_COMMON_DIVISOR"),
            )
        }
        "gcd_hard" => {
            input = generate_gcd_numbers_large_capacity(200_000) // u128::MAX
                .expect("Failed to generate numbers.");
            (
                reference_gcd_large_capacity(&input),
                String::from("./EXE_FILES_HERE/GREATEST_COMMON_DIVISOR"),
            )
        }
        "obelisk" => {
            input = gen_obelisks();
            let obelisks: Vec<u128> = input
                .split(" ")
                .map(|obelisk| obelisk.parse::<u128>().unwrap())
                .collect();
            (
                reference_obelisk_count(&obelisks, 25).to_string(),
                String::from("./EXE_FILES_HERE/OBELISK_EASY"),
            )
        }
        "obelisk_hard" => {
            input = gen_obelisks();
            let obelisks: Vec<u128> = input
                .split(" ")
                .map(|obelisk| obelisk.parse::<u128>().unwrap())
                .collect();
            (
                reference_obelisk_count(&obelisks, 100).to_string(),
                String::from("./EXE_FILES_HERE/OBELISK_HARD"),
            )
        }
        _ => {
            eprintln!(
                "Unknown challenge: {}. Use [reverse|reverse_large|gcd|gcd_hard|obelisk|obelisk_hard].",
                challenge
            );
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
                    if run == 1 && (matches!(challenge.as_str(), "obelisk" | "obelisk_hard")) {
                        if output.trim() > expected_output.trim() {
                            println!("INFO: The provided answer was too high.");
                        } else {
                            println!("INFO: The provided answer was too low.")
                        }
                    }
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
                "\n\x1b[33mSummary for {:?}: \n\x1b[36mAvg: {}\x1b[0m | \x1b[32mMin: {}\x1b[0m | \x1b[31mMax: {}\x1b[0m | \x1b[33mStd Dev: {}\x1b[0m",
                entry.file_name(),
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
