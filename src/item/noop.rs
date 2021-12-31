use super::{Item, Type};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Noop {
    title: String,
    count: usize,

    #[serde(skip_serializing)]
    _typ: Type,
}

impl Item for Noop {
    fn add(&mut self, _: &String) -> Result<(), crate::error::InvalidItemTypeError> {
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
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl Noop {
    pub fn new(title: String) -> Noop {
        Noop {
            title,
            count: 0,
            _typ: Type::Noop,
        }
    }
}
