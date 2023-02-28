pub use crate::cmd_processing::Config;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn run(config: Config) {
    // Count number of each letters in input string
    let mut input_letter_count: HashMap<char, u8> = HashMap::new();
    for c in config.letters.to_lowercase().chars() {
        let count = input_letter_count.entry(c).or_insert(0);
        *count += 1;
    }

    let mut out: Vec<String> = Vec::new();
    // Prepare file to read line by line
    let file = File::open(config.dictionary).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut longest = 0;

    // Process each line of file
    for line in lines {
        // Extract the word
        let word = line.unwrap();
        // Convert to lowercase for testing
        let word_test = word.to_lowercase();

        if config.include && !word_test.contains(config.letter.to_ascii_lowercase()) {
            continue;
        }

        // Count number of each letter in word
        let mut word_letter_count: HashMap<char, u8> = HashMap::new();
        for c in word_test.chars() {
            let count = word_letter_count.entry(c).or_insert(0);
            *count += 1;
        }

        let mut valid_match = true;
        for key in word_letter_count.keys() {
            if !input_letter_count.contains_key(key)
                || input_letter_count.get(key) < word_letter_count.get(key)
            {
                valid_match = false;
            }
        }

        if valid_match {
            out.push(word.clone());
            if word.len() > longest {
                longest = word.len();
            }
        }
    }

    if config.options == ass1::Options::Alpha {
        out.sort();
        for word in &out {
            println! {"{word}"}
        }
    } else if config.options == ass1::Options::Len {
        out.sort();
        while longest > 0 {
            for word in &out {
                if word.len() == longest {
                    println!("{word}");
                }
            }
            longest -= 1;
        }
    } else if config.options == ass1::Options::Longest {
        for word in &out {
            if word.len() >= longest {
                println!("{word}")
            }
        }
    }
}
