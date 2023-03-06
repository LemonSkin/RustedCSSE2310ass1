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
    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }
    mod test_1_valid_input {
        use super::*;
        #[test]
        fn _1_letters_only() {
            // let test_string = "target/debug/ass1 letters";
            // let test_input: Vec<String> =
            //     test_string.split_whitespace().map(str::to_string).collect();
            let test_input = vec_of_strings!("target/debug/ass1", "letters");
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
        #[test]
        fn _2_no_option_no_include_dictionary() {
            let test_string = "target/debug/ass1 letters valid_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
        #[test]
        fn _3_option_no_include_no_dict() {
            let test_string = "target/debug/ass1 -len letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _4_no_option_include_no_dict() {
            let test_string = "target/debug/ass1 -include a letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _5_option_include_no_dict() {
            let test_string = "target/debug/ass1 -longest -include a letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _6_include_option_no_dict() {
            let test_string = "target/debug/ass1 -include a -alpha letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _7_no_option_include_dict() {
            let test_string = "target/debug/ass1 -include a letters valid_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _8_option_include_dict() {
            let test_string = "target/debug/ass1 -alpha -include a letters valid_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Ok(_) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _9_include_option_dict() {
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

    mod test_2_invalid_input {
        use super::*;

        #[test]
        fn _1_invalid_option() {
            let test_string = "target/debug/ass1 -invalid letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _2_double_option() {
            let test_string = "target/debug/ass1 -alpha -alpha letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _3_multiple_option() {
            let test_string = "target/debug/ass1 -alpha -len letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _4_include_multiple_option() {
            let test_string = "target/debug/ass1 -include a -alpha -len letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _5_multiple_option_include() {
            let test_string = "target/debug/ass1 -alpha -len -include a letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _6_include_double_option() {
            let test_string = "target/debug/ass1 -include a -alpha -alpha letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }
        #[test]
        fn _7_double_option_include() {
            let test_string = "target/debug/ass1 -alpha -alpha -include a letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _8_include_multiple_letters() {
            let test_string = "target/debug/ass1 -include ab letters";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _9_include_no_letters() {
            let test_string = "target/debug/ass1 -include a";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _10_args_after_dict() {
            let test_string = "target/debug/ass1 letters valid_dict.txt /another_dict.txt";
            let test_input: Vec<String> =
                test_string.split_whitespace().map(str::to_string).collect();
            let result = Config::build(&test_input);
            match result {
                Err(Error::Usage) => assert!(true),
                _ => assert!(false, "{:?}", result),
            }
        }

        #[test]
        fn _11_include_contains_numeric() {
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

    #[test]
    fn test_3_invalid_dict() {
        let test_string = "target/debug/ass1 letters invalid_dict.txt";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(Error::FileIO(_)) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }
    }

    #[test]
    fn test_4_not_enough_letters() {
        let test_string = "target/debug/ass1 le";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(Error::LettersLength) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }
    }

    #[test]
    fn test_5_letters_contains_numerics() {
        let test_string = "target/debug/ass1 13773r5";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(Error::LettersContainsNumerics) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }
    }
}
