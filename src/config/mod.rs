use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub access_log: String,
    pub log_format: String,
    pub log_types: Vec<String>,
}

impl Config {
    pub fn load_from_yaml_file(filename: &String) -> Result<Config, Box<dyn std::error::Error>> {
        let f = std::fs::File::open(filename)?;
        let d: Config = serde_yaml::from_reader(f)?;
        Ok(d)
    }
}
