use std::collections::HashMap;
use super::Item;

pub struct StrItem {
    title: String,
    count: usize,
    map: HashMap<String, usize>,
}

impl Item for StrItem {
    fn add(&mut self, elem: &str) {
        if let Some(value) = self.map.get_mut(elem) {
            *value += 1;
        } else {
            self.map.insert(elem.to_string(), 1);
        }
        self.count += 1;
    }

    fn count(&self) -> usize {
        self.count
    }

    fn title(&self) -> &str {
        self.title.as_str()
    }

    fn get_result(&self) -> Box<dyn super::result::ItemResult> {
        todo!()
    }
}
