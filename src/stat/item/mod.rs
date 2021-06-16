mod enum_item;
pub mod item_factory;
mod noop_item;
mod num_item;
pub mod result;
mod time_hour_item;

use result::ItemResult;

pub trait Item {
    fn add(&mut self, elem: &str);
    fn count(&self) -> usize;
    fn title(&self) -> String;
    fn get_result(&self) -> Box<dyn ItemResult>;
}
