use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use crate::Options;

pub use crate::configuration::Config;

pub fn run(config: Config) -> Vec<String> {
    // Generate count of each letter in the input word
    let input_letter_count = count_letters(config.letters);

    // Initialise empty vector to hold output words
    let mut out: Vec<String> = Vec::new();

    // Prepare file to read line by line
    let file = File::open(config.dictionary).unwrap();
    let lines = io::BufReader::new(file).lines();

    // Process each line of file
    for line in lines {
        // Extract the word
        let word = line.unwrap();
        // Convert to lowercase for testing
        let word_test = word.to_lowercase();

        // If include is set and the word does not contain the letter, skip to next word
        if config.include && !word_test.contains(config.letter.to_ascii_lowercase()) {
            continue;
        }

        // Generate count of each letter in the test word
        let word_letter_count = count_letters(word_test);

        // Assume word is valid and invalidate if violation is found
        let mut valid_match = true;
        for key in word_letter_count.keys() {
            if !input_letter_count.contains_key(key)
                || input_letter_count.get(key) < word_letter_count.get(key)
            {
                valid_match = false;
            }
        }

        // If word is a valid match, add it to the vector of words and update longest word size
        if valid_match {
            out.push(word);
        }
    }

    if config.options == Options::Alpha {
        out.sort();
    } else if config.options == Options::Len {
        out = sort_by_length(out);
    } else if config.options == Options::Longest {
        out = filter_by_longest(out);
    }

    return out;
}

// Count number of letters in input string
fn count_letters(str: String) -> HashMap<char, u8> {
    let mut input_letter_count: HashMap<char, u8> = HashMap::new();
    for c in str.to_lowercase().chars() {
        let count = input_letter_count.entry(c).or_insert(0);
        *count += 1;
    }

    return input_letter_count;
}

fn sort_by_length(mut words: Vec<String>) -> Vec<String> {
    words.sort();
    let mut sorted_words: Vec<String> = Vec::new();
    let mut longest_word = get_longest_word_length(&words);

    while longest_word > 0 {
        for word in &words {
            if word.len() == longest_word {
                sorted_words.push(word.to_string());
            }
        }
        longest_word -= 1;
    }

    return sorted_words;
}

fn filter_by_longest(words: Vec<String>) -> Vec<String> {
    let mut longest_words: Vec<String> = Vec::new();
    let longest_word = get_longest_word_length(&words);

    for word in words {
        if word.len() == longest_word {
            longest_words.push(word.to_string());
        }
    }

    return longest_words;
}

// Returns the length of the longest word
fn get_longest_word_length(words: &Vec<String>) -> usize {
    let mut longest_word = 0;
    for word in words {
        if word.len() > longest_word {
            longest_word = word.len();
        }
    }
    return longest_word;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1_get_longest_word() {
        let words: Vec<String> = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "abcd".to_string(),
        ];
        let length = get_longest_word_length(&words);

        let expected_length: usize = 4;

        assert_eq!(length, expected_length);
    }

    #[test]
    fn test_2_sort_by_length() {
        let mut words: Vec<String> = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "abcd".to_string(),
        ];
        words = sort_by_length(words);

        let expected_words: Vec<String> = vec![
            "abcd".to_string(),
            "abc".to_string(),
            "ab".to_string(),
            "a".to_string(),
        ];

        assert_eq!(words, expected_words);
    }

    #[test]
    fn test_3_filter_by_longest() {
        let mut words: Vec<String> = vec![
            "a".to_string(),
            "ab".to_string(),
            "abc".to_string(),
            "abcd".to_string(),
        ];
        words = filter_by_longest(words);

        let expected_words: Vec<String> = vec!["abcd".to_string()];

        assert_eq!(words, expected_words);
    }
}
