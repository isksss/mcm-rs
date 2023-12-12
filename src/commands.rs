use crate::config;
use std::process::Command;

pub fn init_command(config_file: &str) -> Result<(), String> {
    // check if config file exists
    match config::load_config(config_file) {
        Ok(_) => {
            println!("Config file already exists");
            return Err(String::from("Config file already exists"));
        }
        Err(_) => {
            println!("Config file does not exist");
            config::create_default_config(config_file)?;
            println!("Config file created");
            return Ok(());
        }
    }
}

pub fn start_command(config_file: &str) -> Result<(), String> {
    // check if config file exists
    let config_data: config::Config = match config::load_config(config_file) {
        Ok(config_data) => {
            println!("Config file exists");
            config_data
        }
        Err(_) => {
            println!("Config file does not exist");
            return Err(String::from("Config file does not exist"));
        }
    };

    // check if java exists
    let java_path = match check_command_exists("java") {
        Ok(path) => {
            println!("Java exists at {}", path);
            path
        }
        Err(err) => {
            return Err(err);
        }
    };


    // check if server jar exists
    let server_jar = format!("{}.jar", config_data.server.project);
    println!("Server jar: {}", server_jar);

    // download server jar
    match download_server_jar(&config_data) {
        Ok(_) => {
            println!("Server jar downloaded");
        }
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}


// private
fn check_command_exists(command: &str) -> Result<String, String> {
    let output = Command::new("which")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let path = String::from_utf8(output.stdout).unwrap();
        return Ok(path.trim().to_string());
    } else {
        return Err(format!("Command {} does not exist", command));
    }
}

fn download_server_jar(config: &config::Config) -> Result<(), String> {
    let server_jar = format!("{}.jar", config.server.project);

    Ok(())
}

fn check_project(config_data: &config::Config) -> Result<(), String> {
    let url = format!("{}", config_data.api.url);
    // get projects

    // 
    Ok(())
}