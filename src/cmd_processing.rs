use std::fs::File;

use crate::Error;
use crate::Options;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub options: Options,
    pub include: bool,
    pub letter: char,
    pub letters: String,
    pub dictionary: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, Error> {
        let mut config = Config {
            options: Options::None,
            include: false,
            letter: '\0',
            letters: String::new(),
            dictionary: String::new(),
        };

        for i in &args[1..] {
            let first_char = i.chars().next().unwrap();
            if first_char == '-' && config.letters.is_empty() {
                match i.as_str() {
                    "-alpha" => {
                        if config.options == Options::None {
                            config.options = Options::Alpha;
                            continue;
                        }
                    }
                    "-len" => {
                        if config.options == Options::None {
                            config.options = Options::Len;
                            continue;
                        }
                    }
                    "-longest" => {
                        if config.options == Options::None {
                            config.options = Options::Longest;
                            continue;
                        }
                    }
                    "-include" => {
                        config.include = true;
                        continue;
                    }
                    _ => {}
                }
                return Err(Error::Usage);
            }

            if config.include && config.letter == '\0' {
                if i.len() < 2 && first_char.is_alphabetic() {
                    config.letter = first_char;
                    continue;
                } else {
                    return Err(Error::Usage);
                }
            }

            if config.letters.is_empty() {
                config.letters = i.clone();
                if i.len() < 3 {
                    return Err(Error::LettersLength);
                } else if !i.chars().all(char::is_alphabetic) {
                    return Err(Error::LettersContainsNumerics);
                }
                continue;
            }

            if config.dictionary.is_empty() {
                config.dictionary = i.clone();
                let _file = match File::open(&config.dictionary) {
                    Err(_) => {
                        return Err(Error::FileIO(config.dictionary));
                    }
                    Ok(_) => {
                        continue;
                    }
                };
            }
            return Err(Error::Usage);
        }

        if config.letters.is_empty() {
            return Err(Error::Usage);
        }
        Ok(config)
    }
}

#[cfg(test)]
mod test_build {
    use super::*;
    mod test_1 {
        use super::*;
        #[test]
        fn test_1_1_valid_input_letters_only() {
            // letters only
            let test_string = "target/debug/ass1 letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
        #[test]
        fn test_1_2_valid_input() {
            // no option, no include, dict
            let test_string = "target/debug/ass1 letters valid_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // option, no include, no dict
            let test_string = "target/debug/ass1 -len letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // no option, include, no dict
            let test_string = "target/debug/ass1 -include a letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // option, include, no dict
            let test_string = "target/debug/ass1 -longest -include a letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // include, option, no dict
            let test_string = "target/debug/ass1 -include a -alpha letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // no option, include, dict
            let test_string = "target/debug/ass1 -include a letters valid_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // option, include, dict
            let test_string = "target/debug/ass1 -alpha -include a letters valid_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // include, option, dict
            let test_string = "target/debug/ass1 -include a -alpha letters valid_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
    }

    mod invalid_inputs {
        use super::*;
        #[test]
        fn test_2_invalid_input() {
            // invalid option
            let test_string = "target/debug/ass1 -invalid letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // double option
            let test_string = "target/debug/ass1 -alpha -alpha letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // multiple option
            let test_string = "target/debug/ass1 -alpha -len letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // include, multiple option
            let test_string = "target/debug/ass1 -include a -alpha -len letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // multiple option, include
            let test_string = "target/debug/ass1 -alpha -len -include a letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // include, double option
            let test_string = "target/debug/ass1 -include a -alpha -alpha letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // double option, include
            let test_string = "target/debug/ass1 -alpha -alpha -include a letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // include multiple letters
            let test_string = "target/debug/ass1 -include ab letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // include, no letters
            let test_string = "target/debug/ass1 -include a";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // args after dict
            let test_string = "target/debug/ass1 letters valid_dict.txt /another_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }

            // include contains numeric
            let test_string = "target/debug/ass1 -include 1 letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
    }

    mod invalid_dict {
        use super::*;
        #[test]
        fn test_3_invalid_dict() {
            let test_string = "target/debug/ass1 letters invalid_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::FileIO(_)) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
    }

    mod letter_length {
        use super::*;
        #[test]
        fn test_4_not_enough_letters() {
            let test_string = "target/debug/ass1 le";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::LettersLength) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
    }

    mod letter_contains_numerics {
        use super::*;
        #[test]
        fn test_5_letters_contains_numerics() {
            let test_string = "target/debug/ass1 13773r5";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::LettersContainsNumerics) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
    }
}
