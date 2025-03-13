use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

const RNG_SEED: u64 = 9001; // Used for generating repeatable wordlists without syncing the files over git

pub fn wordlist(n: usize) -> Result<String, Box<dyn Error>> {
    // Read words from "words_alpha.txt" into a vector
    let file = File::open("./input/words_alpha.txt")?;
    let reader = BufReader::new(file);
    let mut word_list = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // trim the line and push into the vector
        word_list.push(line.trim().to_string());
    }

    // Initialize a seeded RNG for reproducible results.
    let seed: u64 = RNG_SEED;
    let mut rng = StdRng::seed_from_u64(seed);
    // A rough heuristic capacity estimation for the output string.
    let mut out_str = String::with_capacity(n * 6);
    let list_len = word_list.len();

    // Append a random word followed by a space for NUM_WORDS iterations.
    for _ in 0..n {
        let idx = rng.random_range(0..list_len);
        out_str.push_str(&word_list[idx]);
        out_str.push(' ');
    }

    // Remove the trailing space.
    out_str.pop();
    let num_words_str = format_usize(n);
    let fpath = format!("input/wordlist_{num_words_str}.txt");

    // Write the output string to "custom_wordlist.txt"
    let mut output_file = File::create(fpath)?;
    output_file.write_all(out_str.as_bytes())?;

    println!("Successfully generated wordlist.");
    Ok(out_str)
}

fn format_usize(n: usize) -> String {
    if n >= 1_000_000 {
        // Format as millions (Mega)
        format!("{:.0}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        // Format as thousands (Kilo)
        format!("{:.0}K", n as f64 / 1_000.0)
    } else {
        // Print as-is for smaller numbers
        n.to_string()
    }
}
