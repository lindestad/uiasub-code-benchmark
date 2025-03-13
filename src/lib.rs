pub mod input_generators;

use rayon::prelude::*;
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
pub fn reference_reverse_old(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn reference_reverse(input: &str) -> String {
    // Split the input on whitespace and collect words.
    let words: Vec<&str> = input.split_whitespace().collect();

    // Define the desired chunk size. Each chunk will be processed by one thread.
    const CHUNK_SIZE: usize = 10_000;

    // Process chunks in parallel: each chunk reverses all its words and joins them.
    let reversed_chunks: Vec<String> = words
        .par_chunks(CHUNK_SIZE)
        .map(|chunk| {
            // Reverse each word in the chunk.
            let reversed_words: Vec<String> = chunk
                .iter()
                .map(|word| word.chars().rev().collect())
                .collect();
            // Join the reversed words with a single space.
            reversed_words.join(" ")
        })
        .collect();

    // Finally, join all the chunk results with a space.
    reversed_chunks.join(" ")
}

/// Reference implementation for the 'gcd' challenge.
pub fn reference_gcd_old(input: &str) -> String {
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

/// Computes the GCD of a large space-separated input using parallel reduction with Stein's algorithm.
pub fn reference_gcd(input: &str) -> String {
    let numbers: Vec<i64> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();

    if numbers.is_empty() {
        return "No valid numbers found".to_string();
    }

    // Use 0 as the identity since gcd(0, x) = |x|
    let result = numbers.par_iter().cloned().reduce(|| 0, stein_gcd);

    result.to_string()
}

/// Computes the greatest common divisor using Stein's (binary GCD) algorithm.
fn stein_gcd(mut a: i64, mut b: i64) -> i64 {
    // Handle simple cases.
    if a == 0 {
        return b.abs();
    }
    if b == 0 {
        return a.abs();
    }

    // Make both numbers non-negative.
    a = a.abs();
    b = b.abs();

    // Count the number of common factors of 2.
    let shift = (a | b).trailing_zeros();

    // Remove factors of 2 from a.
    a >>= a.trailing_zeros();

    while b != 0 {
        // Remove factors of 2 from b.
        b >>= b.trailing_zeros();

        // Ensure a <= b.
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        // Subtract the smaller from the larger.
        b -= a;
    }
    // Restore common factors of 2.
    a << shift
}
