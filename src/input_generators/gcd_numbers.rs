use super::util::format_usize;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::error::Error;
use std::fs::File;
use std::io::Write;

const RNG_SEED: u64 = 9001; // Static seed for reproducibility

/// Generates `n` pairs of random integers (each in the range 1..=max_value) formatted as:
/// a b
pub fn generate_gcd_numbers(n: usize, max_value: u64) -> Result<String, Box<dyn Error>> {
    // Initialize a seeded RNG for reproducible results.
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    // Estimate capacity for the output string (each pair will have roughly 8 characters)
    let mut out_str = String::with_capacity(n * 8);

    // Generate n pairs of numbers
    for _ in 0..n {
        let a = rng.random_range(1..=max_value);
        let b = rng.random_range(1..=max_value);
        out_str.push_str(&format!("{} {}\n", a, b));
    }

    // Remove the trailing newline, if any.
    if out_str.ends_with('\n') {
        out_str.pop();
    }
    Ok(out_str)
}

/// Generates `n` pairs of random integers (each in the range 1..=u128::MAX) formatted as:
/// a b
pub fn generate_gcd_numbers_large_capacity(n: usize) -> Result<String, Box<dyn Error>> {
    // Initialize a seeded RNG for reproducible results.
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    // Estimate capacity for the output string (each pair will have roughly 8 characters)
    let mut out_str = String::with_capacity(n * 8);

    // Generate n pairs of numbers
    for _ in 0..n {
        let a: u128 = rng.random();
        let b: u128 = rng.random();
        out_str.push_str(&format!("{} {}\n", a, b));
    }

    // Remove the trailing newline, if any.
    if out_str.ends_with('\n') {
        out_str.pop();
    }
    println!("Generating:");
    println!("{}", &out_str.clone());
    Ok(out_str)
}

/// Generates `n` pairs of random integers (each in the range 1..=max_value) formatted as:
/// a b
/// and writes the result to a file named "input/gcd_numbers_{formatted_n}.txt".
pub fn generate_gcd_numbers_to_file(n: usize, max_value: u32) -> Result<String, Box<dyn Error>> {
    // Initialize a seeded RNG for reproducible results.
    let mut rng = StdRng::seed_from_u64(RNG_SEED);
    // Estimate capacity for the output string (each pair will have roughly 8 characters)
    let mut out_str = String::with_capacity(n * 8);

    // Generate n pairs of numbers
    for _ in 0..n {
        let a = rng.random_range(1..=max_value);
        let b = rng.random_range(1..=max_value);
        out_str.push_str(&format!("{} {}\n", a, b));
    }

    // Remove the trailing newline, if any.
    if out_str.ends_with('\n') {
        out_str.pop();
    }

    // Format the number of pairs for the file name (e.g., 1K for 1,000).
    let num_str = format_usize(n);
    let fpath = format!("input/gcd_numbers_{}.txt", num_str);

    // Write the output string to the file.
    let mut output_file = File::create(fpath)?;
    output_file.write_all(out_str.as_bytes())?;

    println!("Successfully generated gcd numbers.");
    Ok(out_str)
}
