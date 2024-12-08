# Word Frequency Counter

A high-performance, parallel word frequency counter built with Rust. This program reads a large text file, calculates the frequency of each word, and displays the top N most frequent words.

---

## Features

- **Parallel Processing**: Uses multiple threads for fast processing of large text files.
- **Concurrency Safety**: Ensures safe shared state management with Rust's ownership model.
- **Customization**: Easily modify the program to analyze different files or display more frequent words.
- **Progress Bar**: Includes a progress bar to track the file processing progress.

---

## Requirements

1. **Rust and Cargo**: Install Rust from [https://rustup.rs](https://rustup.rs).
2. **Dependencies**:
   - `rayon`: For parallel iterators.
   - `indicatif`: For the progress bar.

These are automatically installed via Cargo.

---

## Installation

1. Clone the repository or create the project:
   ```bash
   cargo new word_frequency_counter
   cd word_frequency_counter
2. Build the project: 
    ```bash  
    cargo build --release
    ```
3. Run the project:
    ```bash
    cargo run --release
