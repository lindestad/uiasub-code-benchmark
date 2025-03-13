use rand::Rng;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn wordlist(n: usize) -> Result<(), Box<dyn Error>> {
    // Read words from "words_alpha.txt" into a vector
    let file = File::open("words_alpha.txt")?;
    let reader = BufReader::new(file);
    let mut word_list = Vec::new();
    for line in reader.lines() {
        let line = line?;
        // trim the line and push into the vector
        word_list.push(line.trim().to_string());
    }

    // Prepare to generate random indices and build the output string.
    let mut rng = rand::rng();
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

    println!("Success.");
    Ok(())
}

fn format_usize(n: usize) -> String {
    if n >= 1_000_000 {
        // Format as millions (Mega)
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        // Format as thousands (Kilo)
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        // Print as-is for smaller numbers
        n.to_string()
    }
}
