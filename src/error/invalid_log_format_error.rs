use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidLogFormatError {
    msg: String,
}

impl InvalidLogFormatError {
    pub fn new(msg: String) -> InvalidLogFormatError {
        InvalidLogFormatError { msg }
    }
}

impl fmt::Display for InvalidLogFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid log format, detail: {}", self.msg)
    }
}