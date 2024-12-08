use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use indicatif::ProgressBar;

fn main() {
    let file_path = "input.txt"; 
    let top_n = 10; // Number of most frequent words to display

    let file = File::open(file_path).expect("Could not open the file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not read a line"))
        .collect();

    // Set up a shared word frequency map with Mutex for thread-safe access
    let word_counts = Arc::new(Mutex::new(HashMap::new()));

    // Create a progress bar
    let progress_bar = ProgressBar::new(lines.len() as u64);

    // Process lines in parallel
    lines.par_iter().for_each(|line| {
        let words = preprocess_line(line);

        // Update the shared word frequency map
        let mut map = word_counts.lock().unwrap();
        for word in words {
            *map.entry(word).or_insert(0) += 1;
        }

        progress_bar.inc(1);
    });

    progress_bar.finish();

    // Get the top N words
    let counts = word_counts.lock().unwrap();
    let mut word_vec: Vec<_> = counts.iter().collect();
    word_vec.sort_by(|&(_, a), &(_, b)| b.cmp(a)); // Sort by frequency, descending

    // Print the top N words
    println!("Top {} most frequent words:", top_n);
    for (word, count) in word_vec.into_iter().take(top_n) {
        println!("{}: {}", word, count);
    }
}

// Preprocess a line by splitting into words and normalizing
fn preprocess_line(line: &str) -> Vec<String> {
    line.to_lowercase()
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphanumeric())) // Strip punctuation
        .filter(|word| !word.is_empty())
        .map(String::from)
        .collect()
}
