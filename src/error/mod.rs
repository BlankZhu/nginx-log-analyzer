use crate::item;
use std::fmt;

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

#[derive(Debug)]
pub struct ExtractRegexError {}

impl fmt::Display for ExtractRegexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExtractRegexError: cannot extract regex from log_format")
    }
}

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
