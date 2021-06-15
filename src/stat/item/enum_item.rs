use super::result::enum_result::StrResult;
use super::result::ItemResult;
use super::Item;
use std::collections::HashMap;

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

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_result(&self) -> Box<dyn ItemResult> {
        Box::new(StrResult::new(self.title.as_str(), self.count, &self.map))
    }
}

impl StrItem {
    pub fn new(title: &str) -> StrItem {
        StrItem {
            title: title.to_string(),
            count: 0,
            map: HashMap::new(),
        }
    }
}
