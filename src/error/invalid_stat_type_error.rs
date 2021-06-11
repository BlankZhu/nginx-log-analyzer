use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidStatTypeError {
    msg: String,
}

impl InvalidStatTypeError {
    pub fn new(msg: String) -> InvalidStatTypeError {
        InvalidStatTypeError { msg }
    }
}

impl fmt::Display for InvalidStatTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid stat type, detail: {}", self.msg)
    }
}