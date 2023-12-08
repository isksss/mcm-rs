use serde::{Deserialize, Serialize, de};
use std::fs::{self};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub project: String,
    pub version: Option<String>,
    pub build: Option<String>,
}

// read configfile and parse
pub fn load_config() -> Result<Config, String> {
    let config_file_name = "config.toml";
    let config_file :String = match fs::read_to_string(config_file_name) {
        Ok(config_file) => config_file,
        Err(_) => return Err(format!("Could not read config file: {}", config_file_name)),
    };

    let config : Result<Config, toml::de::Error> = toml::from_str(&config_file);
    match config {
        Ok(config) => Ok(config),
        Err(_) => Err(String::from("Could not parse config file")),
    }
}

// create default configfile
pub fn create_default_config() -> Result<(), String> {
    let config_file_name = "config.toml";
    let defaultvonifg : Config = Config {
        server: Server {
            project: "paper".to_string(),
            version: None,
            build: None,
        },
    };

    let default_config = toml::to_string(&defaultvonifg).unwrap();
    match fs::write(config_file_name, default_config) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Could not write config file: {}", config_file_name)),
    }
}