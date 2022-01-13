use std::fmt::format;

use serde::{Deserialize, Serialize};

use crate::error::LoadYamlConfigError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // pub access_log: String,
    pub log_format: String,
    pub log_types: Vec<String>,
}

impl Config {
    pub fn load_from_yaml_file(filename: &String) -> Result<Config, LoadYamlConfigError> {
        std::fs::File::open(filename)
            .map_err(|err| LoadYamlConfigError {
                filename: filename.clone(),
                detail: format!("{}", err),
            })
            .and_then(|file| {
                serde_yaml::from_reader(file).map_err(|err| LoadYamlConfigError {
                    filename: filename.clone(),
                    detail: format!("{}", err),
                })
            })
    }
}
