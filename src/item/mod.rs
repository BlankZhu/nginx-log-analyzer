mod f64;
mod hour;
mod isize;
mod minute;
mod noop;
mod str;

use crate::error;
use crate::result;

pub enum Type {
    F64,
    Hour,
    Isize,
    Minute,
    Noop,
    Str,
}

pub trait Item {
    fn add(&mut self, elem: &String) -> Result<(), error::InvalidItemTypeError>;
    fn get_count(&self) -> usize;
    fn get_title(&self) -> String;
    fn get_result(&self) -> Box<dyn result::Result>;
}

pub struct Factory;

impl Factory {
    pub fn create_item(typ: Type, title: String) -> Box<dyn Item> {
        todo!()
    }
}
