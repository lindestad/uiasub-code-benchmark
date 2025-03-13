use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Instant;

fn main() {
    // Usage: benchmark <challenge> <input_file> <executables_dir>
    if env::args().len() < 4 {
        eprintln!(
            "Usage: {} [reverse|gcd] <input_file> <executables_dir>",
            env::args().next().unwrap()
        );
        std::process::exit(1);
    }
    let args: Vec<String> = env::args().collect();
    let challenge = &args[1];
    let input_file = &args[2];
    let executables_dir = &args[3];

    // Load input data from the file
    let input = fs::read_to_string(input_file).expect("Failed to read the input file");

    // Compute the expected output using our reference implementation
    let expected_output = match challenge.as_str() {
        "reverse" => reference_reverse(&input),
        "gcd" => reference_gcd(&input),
        _ => {
            eprintln!("Unknown challenge: {}. Use 'reverse' or 'gcd'.", challenge);
            std::process::exit(1);
        }
    };

    // Iterate over the executables in the given directory.
    let entries = fs::read_dir(executables_dir).expect("Failed to read executables directory");
    for entry in entries {
        let entry = entry.expect("Error reading a directory entry");
        let path = entry.path();

        // If it's a file (TODO optionally, can also check for execution permissions)
        if path.is_file() {
            println!("Benchmarking executable: {:?}", path);

            // Time the external program
            let start = Instant::now();
            let output = run_executable(&path, &input);
            let duration = start.elapsed();

            // Validate the output (trimming to ignore minor whitespace differences)
            if output.trim() == expected_output.trim() {
                println!("✅ Output correct. Execution time: {:?}", duration);
            } else {
                println!("❌ Output incorrect. Execution time: {:?}", duration);
                println!("Expected Output:\n{}", expected_output);
                println!("Actual Output:\n{}", output);
            }
            println!("----------------------------------");
        }
    }
}

/// Runs an external executable, piping `input` to its stdin and returning the captured stdout.
fn run_executable(path: &Path, input: &str) -> String {
    let mut child = Command::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    {
        // Write the input into the child's stdin.
        let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");
        child_stdin
            .write_all(input.as_bytes())
            .expect("Failed to write input to child process");
    }

    let output = child.wait_with_output().expect("Failed to capture output");
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Reference implementation for the 'reverse' challenge.
/// Splits the input on whitespace, reverses each word, and joins them with a space.
fn reference_reverse(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Reference implementation for the 'gcd' challenge.
/// Parses space-separated numbers and computes their greatest common divisor.
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
