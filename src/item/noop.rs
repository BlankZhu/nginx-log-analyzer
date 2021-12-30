use crate::error;
use crate::result;

use super::Item;

pub struct F64 {
    title: String,
}

impl Item for F64 {
    fn add(&mut self, elem: &String) -> Result<(), error::InvalidItemTypeError> {
        todo!()
    }

    fn get_count(&self) -> usize {
        todo!()
    }

    fn get_title(&self) -> String {
        todo!()
    }

    fn get_result(&self) -> Box<dyn result::Result> {
        todo!()
    }
}
