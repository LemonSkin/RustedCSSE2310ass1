use std::fs::File;

use ass1::ErrorHandler;
use ass1::Options;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub options: Options,
    pub include: bool,
    pub letter: char,
    pub letters: String,
    pub dictionary: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, ErrorHandler> {
        let mut error_code: i32 = 0;

        let mut config = Config {
            options: Options::None,
            include: false,
            letter: '\0',
            letters: String::from(""),
            dictionary: String::from(""),
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
                error_code = 1;
                break;
            }

            if config.include && config.letter == '\0' {
                if i.len() < 2 && first_char.is_alphabetic() {
                    config.letter = first_char;
                    continue;
                } else {
                    error_code = 1;
                    break;
                }
            }

            if config.letters.is_empty() {
                config.letters = i.clone();
                if i.len() < 3 {
                    error_code = 3;
                    break;
                } else if !i.chars().all(char::is_alphabetic) {
                    error_code = 4;
                    break;
                }
                continue;
            }

            if config.dictionary.is_empty() {
                config.dictionary = i.clone();
                let _file = match File::open(&config.dictionary) {
                    Err(_) => {
                        error_code = 2;
                        break;
                    }
                    Ok(_) => {
                        continue;
                    }
                };
            }
            error_code = 1;
            break;
        }

        if config.letters.is_empty() {
            error_code = 1;
        }

        match error_code {
            1 => {
                return Err(ErrorHandler {
                    error_string: String::from(
                        "unjumble [-alpha|-len|-longest] [-include letter] letters [dictionary]",
                    ),
                    error_code: (error_code),
                })
            }
            2 => {
                return Err(ErrorHandler {
                    error_string: format!(
                        "unjumble: file \"{}\" can not be opened",
                        config.dictionary
                    ),
                    error_code: (error_code),
                })
            }
            3 => {
                return Err(ErrorHandler {
                    error_string: String::from("unjumble: must supply at least 3 letters"),
                    error_code: (error_code),
                })
            }
            4 => {
                return Err(ErrorHandler {
                    error_string: String::from("unjumble: can only unjumble alphabetic characters"),
                    error_code: (error_code),
                })
            }
            _ => return Ok(config),
        }
        // Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_valid_input() {
        // letters only
        let test_string = "target/debug/ass1 letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }

        // no option, no include, dict
        let test_string = "target/debug/ass1 letters valid_dict.txt";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }

        // option, no include, no dict
        let test_string = "target/debug/ass1 -len letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }

        // no option, include, no dict
        let test_string = "target/debug/ass1 -include a letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }

        // option, include, no dict
        let test_string = "target/debug/ass1 -longest -include a letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }

        // include, option, no dict
        let test_string = "target/debug/ass1 -include a -alpha letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }

        // no option, include, dict
        let test_string = "target/debug/ass1 -include a letters valid_dict.txt";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }

        // option, include, dict
        let test_string = "target/debug/ass1 -alpha -include a letters valid_dict.txt";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }

        // include, option, dict
        let test_string = "target/debug/ass1 -include a -alpha letters valid_dict.txt";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false, "{:?}", result),
        }
    }

    #[test]
    fn test_2_invalid_input() {
        // invalid option
        let test_string = "target/debug/ass1 -invalid letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // double option
        let test_string = "target/debug/ass1 -alpha -alpha letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // multiple option
        let test_string = "target/debug/ass1 -alpha -len letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // include, multiple option
        let test_string = "target/debug/ass1 -include a -alpha -len letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // multiple option, include
        let test_string = "target/debug/ass1 -alpha -len -include a letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // include, double option
        let test_string = "target/debug/ass1 -include a -alpha -alpha letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // double option, include
        let test_string = "target/debug/ass1 -alpha -alpha -include a letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // include multiple letters
        let test_string = "target/debug/ass1 -include ab letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // include, no letters
        let test_string = "target/debug/ass1 -include a";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // args after dict
        let test_string = "target/debug/ass1 letters valid_dict.txt /another_dict.txt";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }

        // include contains numeric
        let test_string = "target/debug/ass1 -include 1 letters";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 1),
            _ => assert!(false, "{:?}", result),
        }
    }

    #[test]
    fn test_3_invalid_dict() {
        let test_string = "target/debug/ass1 letters invalid_dict.txt";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 2),
            _ => assert!(false, "{:?}", result),
        }
    }

    #[test]
    fn test_4_not_enough_letters() {
        let test_string = "target/debug/ass1 le";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 3),
            _ => assert!(false, "{:?}", result),
        }
    }

    #[test]
    fn test_5_letters_contains_numerics() {
        let test_string = "target/debug/ass1 13773r5";
        let test_input: Vec<String> = test_string.split_whitespace().map(str::to_string).collect();
        let result = Config::build(&test_input);
        match result {
            Err(err) => assert_eq!(err.error_code, 4),
            _ => assert!(false, "{:?}", result),
        }
    }
}
