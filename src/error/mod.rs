use crate::item;

// LogConfigError tells if log format doesn't fit log types in config files
#[derive(Debug)]
pub struct LogConfigError {
    pub detail: String,
}

// InvalidLogTypeError tells if log types meets invalid string
#[derive(Debug)]
pub struct InvalidLogTypeError {
    pub raw_type: String,
}

// InvalidLogTypeError tells if log line doesn't fit the registered log format
#[derive(Debug)]
pub struct InvalidLogLineError {
    pub line: String,
}

// LoadAccessLogError tells if any error while opening nginx's access log file content
#[derive(Debug)]
pub struct LoadAccessLogError {
    pub filename: String,
    pub detail: String,
}

// InvalidStatusDataError covers InvalidItemTypeError
// InvalidStatusDataError tells possible errors while adding access logs'
// elems to statistics
#[derive(Debug)]
pub struct InvalidStatusDataError {
    pub detail: String,
    // pub backtrace: String,
}

#[derive(Debug)]
pub struct InvalidItemTypeError {
    pub item_title: String,
    pub typ: item::Type,
    pub data: String,
}
