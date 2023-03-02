pub mod configuration;
pub mod runner;

#[derive(Debug, PartialEq)]
pub enum Options {
    None,
    Alpha,
    Len,
    Longest,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Usage,
    FileIO(String),
    LettersLength,
    LettersContainsNumerics,
}
