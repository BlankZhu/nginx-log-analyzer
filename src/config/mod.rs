use crate::error::LoadYamlConfigError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
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
