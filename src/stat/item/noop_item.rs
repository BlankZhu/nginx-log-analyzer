use super::Item;
use super::result::ItemResult;
use super::result::noop_result::NoopResult;

pub struct NoopItem {
    title: String
}

impl Item for NoopItem {
    fn add(&mut self, _: &str) {
        
    }

    fn count(&self) -> usize {
        0
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn get_result(&self) -> Box<dyn ItemResult> {
        Box::new(NoopResult::new(self.title.as_str()))
    }
}