use std::env;
use std::process;

// mod cmd_processing;
use ass1::cmd_processing;
use ass1::runner;
use ass1::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: cmd_processing::Config =
        cmd_processing::Config::build(&args).unwrap_or_else(|err| match err {
            Error::Usage => {
                eprintln!(
                    "Usage: unjumble [-alpha|-len|-longest] [-include letter] letters [dictionary]"
                );
                process::exit(1);
            }
            Error::FileIO(string) => {
                eprintln!("unjumble: file \"{string}\" can not be opened");
                process::exit(2);
            }
            Error::LettersLength => {
                eprintln!("unjumble: must supply at least three letters");
                process::exit(3);
            }
            Error::LettersContainsNumerics => {
                eprintln!("unjumble: can only unjumble alphabetic characters");
                process::exit(4);
            }
        });

    runner::run(config);
}
