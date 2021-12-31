use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub access_log: String,
    pub log_format: String,
    pub log_types: Vec<String>,
}
