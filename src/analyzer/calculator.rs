use std::collections::HashMap;

use crate::error::InvalidStatusDataError;
use crate::item::Item;

pub struct Calculator {
    items: Vec<Box<dyn Item>>,
}

impl Calculator {
    pub fn new() -> Calculator {
        Calculator { items: Vec::new() }
    }

    pub fn register_item(&mut self, item: Box<dyn Item>) {
        self.items.push(item);
    }

    pub fn get_titles(&self) -> String {
        let titles: Vec<String> = self.items.iter().map(|item| item.get_title()).collect();
        titles.join(",")
    }

    pub fn get_types(&self) -> String {
        let types: Vec<String> = self.items.iter().map(|item| item.get_type()).collect();
        types.join(",")
    }

    pub fn add_data(&mut self, data: Vec<String>) -> Result<(), InvalidStatusDataError> {
        if data.len() != self.items.len() {
            let detail = format!(
                "given data's length({}) doesn't fit items length({})",
                data.len(),
                self.items.len()
            );
            return Err(InvalidStatusDataError { detail });
        }

        let mut index: usize = 0;
        for datum in data.iter() {
            if let Err(err) = self.items[index].add(datum) {
                eprintln!(
                    "given data({}) doesn't fit the item {}({:?})",
                    err.data, err.item_title, err.typ
                );
                continue;
            }

            index += 1;
        }

        Ok(())
    }

    pub fn get_results(&self) -> HashMap<String, &Box<dyn Item>> {
        let mut ret = HashMap::new();
        for item in self.items.iter() {
            ret.insert(item.get_title(), item);
        }
        ret
    }
}
