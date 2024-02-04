use planety_core::System;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};

use crate::error::ApplicationError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub timestep: f64,
    pub steps: i64,
    pub export_step: i64,
    pub export_directory: String,
    pub export_file_name_prefix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigSystem {
    pub config: Config,
    pub system: System,
}

impl ConfigSystem {
    pub fn parse(filename: String) -> Result<Self, ApplicationError> {

        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config_result: Result<ConfigSystem, toml::de::Error> = toml::from_str(&contents);
        match config_result {
            Ok(config) => Ok(config),
            Err(e) => {
                println!("Error when parsing config from file {e}");
                Err(ApplicationError::from(e))
            }
        }
    }
}
