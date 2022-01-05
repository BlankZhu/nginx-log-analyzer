use std::collections::HashMap;

use super::{Item, Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Str {
    title: String,
    count: usize,
    map: HashMap<String, usize>,

    #[serde(skip_serializing)]
    typ: Type,
}

impl Item for Str {
    fn add(&mut self, datum: &String) -> Result<(), crate::error::InvalidItemDataError> {
        if let Some(value) = self.map.get_mut(datum) {
            *value += 1;
        } else {
            self.map.insert(datum.to_string(), 1);
        }
        self.count += 1;

        Ok(())
    }

    fn get_count(&self) -> usize {
        self.count.clone()
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_result(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn get_type(&self) -> String {
        format!("{:?}", self.typ)
    }
}

impl Str {
    pub fn new(title: String) -> Str {
        Str {
            title,
            count: 0,
            map: HashMap::new(),
            typ: Type::Str,
        }
    }
}
