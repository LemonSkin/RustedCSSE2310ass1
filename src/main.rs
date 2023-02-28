use std::env;
use std::process;

mod cmd_processing;
pub use crate::cmd_processing::Config;

mod runner;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| match err.error_code {
        _ => {
            eprintln!("{}", err.error_string);
            process::exit(err.error_code);
        }
    });

    runner::run(config);
}
