use core::fmt;

pub trait ErrorStack {}

#[derive(Debug)]
pub struct LogFormatError;

#[derive(Debug)]
pub struct LoadAccessLogError {
    pub filename: String,
}

#[derive(Debug)]
pub struct InvalidAccessLogError {
    pub line: String,
}

// todo: cover InvalidItemTypeError
#[derive(Debug)]
pub struct InvalidStatusDataError {}

#[derive(Debug)]
pub struct InvalidLogFormatError {
    // pub detail: String,
}

#[derive(Debug)]
pub struct InvalidItemTypeError {
    pub item_title: String,
    // pub typ: Type,
}
