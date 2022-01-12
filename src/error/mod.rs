use crate::item;
use std::{error, fmt};

// LogConfigError tells if log format doesn't fit log types in config files
#[derive(Debug)]
pub struct LogConfigError {
    pub detail: String,
}

impl fmt::Display for LogConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LogConfigError: {}", self.detail)
    }
}

impl error::Error for LogConfigError {}

// InvalidLogTypeError tells if log_types meets invalid string
#[derive(Debug)]
pub struct InvalidLogTypeError {
    pub raw_type: String,
}

impl fmt::Display for InvalidLogTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidLogTypeError: `{}`", self.raw_type)
    }
}

impl error::Error for InvalidLogTypeError {}

#[derive(Debug)]
pub struct ExtractRegexError {}

impl fmt::Display for ExtractRegexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExtractRegexError: cannot extract regex from log_format")
    }
}

impl error::Error for ExtractRegexError {}

// InvalidLogTypeError tells if log line doesn't fit the registered log format
#[derive(Debug)]
pub struct InvalidLogLineError {
    pub line: String,
}

impl fmt::Display for InvalidLogLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InvalidLogLineError: line `{}` doesn't fit log_type",
            self.line
        )
    }
}

impl error::Error for InvalidLogLineError {}

// LoadAccessLogError tells if any error while opening nginx's access log file content
#[derive(Debug)]
pub struct LoadAccessLogError {
    pub filename: String,
    pub detail: String,
}

impl fmt::Display for LoadAccessLogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LoadAccesssLogError: cannot load `{}`, detail: {}",
            self.filename, self.detail
        )
    }
}

impl error::Error for LoadAccessLogError {}

// InvalidStatusDataError covers InvalidItemDataError
// InvalidStatusDataError tells possible errors while adding access logs'
// elems to statistics
#[derive(Debug)]
pub struct InvalidStatusDataError {
    pub detail: String,
}

impl fmt::Display for InvalidStatusDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InvalidStatusDataError: {}", self.detail)
    }
}

impl error::Error for InvalidStatusDataError {}

#[derive(Debug)]
pub struct InvalidItemDataError {
    pub item_title: String,
    pub typ: item::Type,
    pub data: String,
}

impl fmt::Display for InvalidItemDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InvalidItemDataError: {}({:?}) got invalid data {}",
            self.item_title, self.typ, self.data
        )
    }
}

impl error::Error for InvalidItemDataError {}
