use std::collections::HashMap;

use crate::error;

use super::{Item, Type};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref RE_HOUR: Regex = Regex::new(r"(\d{2}/\w{3}/\d{4}:\d{2})").unwrap();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Hour {
    title: String,
    count: usize,
    map: HashMap<String, usize>,

    #[serde(skip_serializing)]
    typ: Type,
}

impl Item for Hour {
    fn add(&mut self, datum: &String) -> Result<(), crate::error::InvalidItemDataError> {
        let cap = RE_HOUR.captures(datum);
        match cap {
            Some(cap) => {
                let key = &cap[1];
                if let Some(value) = self.map.get_mut(key) {
                    *value += 1;
                } else {
                    self.map.insert(key.to_string(), 1);
                }
                self.count += 1;
                Ok(())
            }
            None => Err(error::InvalidItemDataError {
                item_title: self.title.clone(),
                typ: self.typ.clone(),
                data: datum.clone(),
            }),
        }
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

impl Hour {
    pub fn new(title: String) -> Hour {
        Hour {
            title,
            count: 0,
            map: HashMap::new(),
            typ: Type::Hour,
        }
    }
}
