use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidAccessLogError {
    msg: String,
}

impl InvalidAccessLogError {
    pub fn new(msg: String) -> InvalidAccessLogError {
        InvalidAccessLogError { msg }
    }
}

impl fmt::Display for InvalidAccessLogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid access log, detail: {}", self.msg)
    }
}