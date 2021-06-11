use std::collections::HashMap;

use super::ItemResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StrResult {
    title: String,
    count: usize,
    percentage: HashMap<String, f64>,
}

impl ItemResult for StrResult {
    fn get_readable_result(&self) -> String {
        let mut ret = format!("{}\ncount:\t{}\ndetail:\n", self.title, self.count);
        let mut tmp: Vec<String> = Vec::new();
        for (k, v) in &self.percentage {
            let line = format!("- {}\t{:.4}%", k, v * 100.0);
            tmp.push(line);
        }
        ret.push_str(tmp.join("\n").as_str());
        ret
    }

    fn get_json_result(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl StrResult {
    pub fn new(title: &str, count: usize, data: &HashMap<String, usize>) -> StrResult {
        let p = data.iter()
            .map(|pair| (pair.0.clone(), (*pair.1 as f64) / (count as f64)))
            .collect();

        StrResult {
            title: title.to_string(),
            count,
            percentage: p,
        }
    }
}
