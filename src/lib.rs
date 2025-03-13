pub mod input_generators;

use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

/// Runs an external executable by piping `input` to its stdin and capturing stdout.
pub fn run_executable(path: &Path, input: &str) -> String {
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

/// Helper to format time nicely. If less than one second, print in milliseconds.
pub fn format_time(seconds: f64) -> String {
    if seconds < 1.0 {
        format!("{:.4}ms", seconds * 1000.0)
    } else {
        format!("{:.4}s", seconds)
    }
}

/// Reference implementation for the 'reverse' challenge.
pub fn reference_reverse(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

/// Reference implementation for the 'gcd' challenge.
pub fn reference_gcd(input: &str) -> String {
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
pub fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
