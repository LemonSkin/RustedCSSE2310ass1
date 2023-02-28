#[derive(Debug, PartialEq)]
pub enum Options {
    None,
    Alpha,
    Len,
    Longest,
}

#[derive(Debug, PartialEq)]
pub struct ErrorHandler {
    pub error_string: String,
    pub error_code: i32,
}
