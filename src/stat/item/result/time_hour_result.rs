use std::collections::HashMap;

use super::ItemResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeHourResult {
    title: String,
    count: usize,
    data: HashMap<String, usize>,
}

impl ItemResult for TimeHourResult {
    fn get_readable_result(&self) -> String {
        let mut ret = format!("{}\ncount:\t{}\ndetail:\n", self.title, self.count);
        let mut tmp: Vec<String> = Vec::new();

        for (k, v) in &self.data {
            let line = format!("- {}\t{}", k, v);
            tmp.push(line);
        }
        ret.push_str(tmp.join("\n").as_str());
        ret
    }

    fn get_json_result(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl TimeHourResult {
    pub fn new(title: &str, count: usize, data: &HashMap<String, usize>) -> TimeHourResult {
        TimeHourResult {
            title: title.to_string(),
            count,
            data: data.clone(),
        }
    }
}
