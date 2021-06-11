use std::fmt;

#[derive(Debug, Clone)]
pub struct LoadFileError {
    msg: String,
}

impl LoadFileError {
    pub fn new(msg: String) -> LoadFileError {
        LoadFileError { msg }
    }
}

impl fmt::Display for LoadFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid log format, detail: {}", self.msg)
    }
}