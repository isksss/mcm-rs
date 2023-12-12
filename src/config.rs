use serde::{Deserialize, Serialize};
use std::fs::{self};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub api: Api,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub project: String,
    pub version: Option<String>,
    pub build: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Api{
    pub url: String,
}

// read configfile and parse
pub fn load_config(config_file_name: &str) -> Result<Config, String> {
    let config_file: String = match fs::read_to_string(config_file_name) {
        Ok(config_file) => config_file,
        Err(_) => return Err(format!("Could not read config file: {}", config_file_name)),
    };

    let config: Result<Config, toml::de::Error> = toml::from_str(&config_file);
    match config {
        Ok(config) => Ok(config),
        Err(_) => Err(String::from("Could not parse config file")),
    }
}

// create default configfile
pub fn create_default_config(config_file_name: &str) -> Result<(), String> {
    let defaultvonifg: Config = Config {
        server: Server {
            project: "paper".to_string(),
            version: Some("1.16.5".to_string()),
            build: Some("".to_string()),
        },
        api: Api {
            url: "https://api.papermc.io/v2/projects".to_string(),
        },
    };

    let default_config = toml::to_string(&defaultvonifg).unwrap();
    match fs::write(config_file_name, default_config) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Could not write config file: {}", config_file_name)),
    }
}
