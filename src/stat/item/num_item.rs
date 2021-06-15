use super::result::num_result::{F64Result, IsizeResult};
use super::result::ItemResult;
use super::Item;

pub struct IsizeItem {
    title: String,
    data: Vec<isize>,
}

impl Item for IsizeItem {
    fn add(&mut self, elem: &str) {
        let parse_isize = elem.parse::<isize>();
        match parse_isize {
            Ok(value) => {
                self.data.push(value);
            }
            Err(err) => {
                eprintln!(
                    "failed parse IsizeItem[{}] from: [{}] , detail: [{}]",
                    &self.title, &elem, &err
                );
            }
        }
    }

    fn count(&self) -> usize {
        self.data.len()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_result(&self) -> Box<dyn ItemResult> {
        Box::new(IsizeResult::new(self.title.as_str(), &self.data))
    }
}

impl IsizeItem {
    pub fn new(title: &str) -> IsizeItem {
        IsizeItem {
            title: title.to_string(),
            data: Vec::new(),
        }
    }
}

pub struct F64Item {
    title: String,
    data: Vec<f64>,
}

impl Item for F64Item {
    fn add(&mut self, elem: &str) {
        let parse_isize = elem.parse::<f64>();
        match parse_isize {
            Ok(value) => {
                self.data.push(value);
            }
            Err(err) => {
                eprintln!(
                    "failed parse F64Item[{}] from: [{}] , detail: [{}]",
                    &self.title, &elem, &err
                );
            }
        }
    }

    fn count(&self) -> usize {
        self.data.len()
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_result(&self) -> Box<dyn ItemResult> {
        Box::new(F64Result::new(self.title.as_str(), &self.data))
    }
}

impl F64Item {
    pub fn new(title: &str) -> F64Item {
        F64Item {
            title: title.to_string(),
            data: Vec::new(),
        }
    }
}
