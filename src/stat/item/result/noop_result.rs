use super::ItemResult;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct NoopResult {
    title: String
}

impl ItemResult for NoopResult {
    fn get_readable_result(&self) -> String {
        format!("{}\nno available data", self.title)
    }

    fn get_json_result(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

impl NoopResult {
    pub fn new(title: &str) -> NoopResult {
        NoopResult {
            title: title.to_string(),
        }
    }
}