pub mod input_generators;

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
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

/// Euclid's algorithm for computing the greatest common divisor.
#[allow(dead_code)]
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

    let results: Vec<String> = numbers
        .par_chunks(2)
        .map(|chunk| {
            if chunk.len() < 2 {
                // Return a message for incomplete pairs
                panic!("Got chunk of length < 2, garbled input?");
            } else {
                let result = stein_gcd(chunk[0], chunk[1]);
                result.to_string()
            }
        })
        .collect();

    results.join(" ")
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

/// Computes the GCD of a large space-separated input using parallel reduction with Stein's algorithm.
pub fn reference_gcd_large_capacity(input: &str) -> String {
    let numbers: Vec<u128> = input
        .split_whitespace()
        .filter_map(|s| s.parse::<u128>().ok())
        .collect();

    if numbers.is_empty() {
        return "No valid numbers found".to_string();
    }

    let results: Vec<String> = numbers
        .par_chunks(2)
        .map(|chunk| {
            if chunk.len() < 2 {
                // Return a message for incomplete pairs
                panic!("Got chunk of length < 2, garbled input?");
            } else {
                let result = stein_gcd_large_capacity(chunk[0], chunk[1]);
                result.to_string()
            }
        })
        .collect();

    results.join(" ")
}

/// Computes the greatest common divisor using Stein's (binary GCD) algorithm.
fn stein_gcd_large_capacity(mut a: u128, mut b: u128) -> u128 {
    // Handle simple cases.
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

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

/// Calculate how many obelisks there are after `pull_count`
/// starting from an initial collection of obelisks.
pub fn reference_obelisk_count(obelisks: &[u128], pull_count: usize) -> u128 {
    fn digit_count(mut x: u128) -> usize {
        if x == 0 {
            return 1;
        }
        let mut count = 0;
        while x > 0 {
            x /= 10;
            count += 1;
        }
        count
    }

    fn split_obelisk_two(x: u128) -> (u128, u128) {
        let s = x.to_string();
        let half = s.len() / 2;
        let left_str = &s[..half];
        let right_str = &s[half..];
        let left_num = left_str.parse::<u128>().unwrap();
        let right_num = right_str.parse::<u128>().unwrap();
        (left_num, right_num)
    }
    fn next_obelisks(n: u128) -> Vec<u128> {
        if n == 0 {
            // rule #1
            vec![1]
        } else if n == 7 {
            // rule #2
            vec![3, 2, 3, 2]
        } else {
            let dcount = digit_count(n);
            if dcount % 2 == 0 {
                // rule #2 (even digit count): split
                let (l, r) = split_obelisk_two(n);
                vec![l, r]
            } else {
                // rule #3 (odd digit count, nonzero): multiply
                vec![n * 2404]
            }
        }
    }

    fn expand(
        n: u128,
        t: usize,
        memo: &mut HashMap<(u128, usize), u128>,
        in_progress: &mut HashSet<(u128, usize)>,
    ) -> u128 {
        // Base case
        if t == 0 {
            return 1;
        }

        // If it's already in memo, return cached value.
        if let Some(&cached) = memo.get(&(n, t)) {
            return cached;
        }

        // Cycle detection: if we revisit (n, t) before finishing,
        // we've detected a cycle. Does not seem to occur so left as a panic.
        if !in_progress.insert((n, t)) {
            panic!(
                "Cycle detected at obelisk = {n}, t = {t} -- implement cycle skipping here if this ever panics."
            );
        }

        // Expand to child obelisks, then sum their expansions.
        let children = next_obelisks(n);
        let mut sum = 0;
        for &child in &children {
            sum += expand(child, t - 1, memo, in_progress);
        }

        // Mark (n,t) done; store in memo and remove from "in progress"
        memo.insert((n, t), sum);
        in_progress.remove(&(n, t));

        sum
    }
    let mut memo = HashMap::<(u128, usize), u128>::new();
    let mut in_progress = HashSet::<(u128, usize)>::new();

    let mut total = 0;
    for &initial_obelisk in obelisks {
        total += expand(initial_obelisk, pull_count, &mut memo, &mut in_progress);
    }
    total
}
