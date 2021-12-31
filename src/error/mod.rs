use crate::item;

// LogConfigError tells if log format doesn't fit log types in config files
#[derive(Debug)]
pub struct LogConfigError;

// LoadAccessLogError tells if any error while opening nginx's access log file content
#[derive(Debug)]
pub struct LoadAccessLogError {
    pub filename: String,
    pub detail: String,
}

// InvalidAccessLogError tells if any error while reading access log's line
#[derive(Debug)]
pub struct InvalidAccessLogError {
    pub line: String,
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
