use super::result::time_result::TimeResult;
use super::result::ItemResult;
use super::Item;
use lazy_static::lazy_static;
use std::collections::HashMap;
use regex::Regex;

pub struct TimeHourItem {
    title: String,
    count: usize,
    data: HashMap<String, usize>,
}

impl Item for TimeHourItem {
    fn add(&mut self, elem: &str) {
        lazy_static! {
            static ref RE_HOUR: Regex = Regex::new(r"(\d{2}/\w{3}/\d{4}:\d{2})").unwrap();
        }
        let cap = RE_HOUR.captures(elem);
        match cap {
            Some(cap) => {
                let key: &str = &cap[1];
                if let Some(value) = self.data.get_mut(key) {
                    *value += 1;
                } else {
                    self.data.insert(key.to_string(), 1);
                }
                self.count += 1;
            },
            None => {
                eprintln!("cannot capture hour info from {}", elem);
                return;
            }
        }

    }

    fn count(&self) -> usize {
        self.count
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_result(&self) -> Box<dyn ItemResult> {
        Box::new(TimeResult::new(self.title.as_str(), self.count, &self.data))
    }
}

impl TimeHourItem {
    pub fn new(title: &str) -> TimeHourItem {
        TimeHourItem {
            title: title.to_string(),
            count: 0,
            data: HashMap::new(),
        }
    }
}

pub struct TimeMinuteItem {
    title: String,
    count: usize,
    data: HashMap<String, usize>,
}

impl Item for TimeMinuteItem {
    fn add(&mut self, elem: &str) {
        lazy_static! {
            static ref RE_MIN: Regex = Regex::new(r"(\d{2}/\w{3}/\d{4}:\d{2}:\d{2})").unwrap();
        }
        let cap = RE_MIN.captures(elem);
        match cap {
            Some(cap) => {
                let key: &str = &cap[1];
                if let Some(value) = self.data.get_mut(key) {
                    *value += 1;
                } else {
                    self.data.insert(key.to_string(), 1);
                }
                self.count += 1;
            },
            None => {
                eprintln!("cannot capture minute info from {}", elem);
                return;
            }
        }

    }

    fn count(&self) -> usize {
        self.count
    }

    fn title(&self) -> String {
        self.title.clone()
    }

    fn get_result(&self) -> Box<dyn ItemResult> {
        Box::new(TimeResult::new(self.title.as_str(), self.count, &self.data))
    }
}

impl TimeMinuteItem {
    pub fn new(title: &str) -> TimeMinuteItem {
        TimeMinuteItem {
            title: title.to_string(),
            count: 0,
            data: HashMap::new(),
        }
    }
}
