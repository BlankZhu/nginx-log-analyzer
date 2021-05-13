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
