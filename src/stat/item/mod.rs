mod enum_item;
pub mod item_factory;
mod noop_item;
mod num_item;
mod result;

use result::ItemResult;

pub trait Item {
    fn add(&mut self, elem: &str);
    fn count(&self) -> usize;
    fn title(&self) -> &str;
    fn get_result(&self) -> Box<dyn ItemResult>;
}
