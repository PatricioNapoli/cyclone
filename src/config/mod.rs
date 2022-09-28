use std::fs;

use log::{error};

extern crate yaml_rust;
use yaml_rust::{YamlLoader};

pub struct Configuration {
    pub socket: String,
}

impl Configuration {
    pub fn new(config_file: &str) -> Self {
        match fs::read_to_string(config_file) {
            Ok(conf) => {
                let file = YamlLoader::load_from_str(&conf).unwrap();
                let root = &file[0];

                Configuration {
                    socket: root["unix_socket"].as_str().unwrap().to_string(),
                }
            },
            Err(_) => {
                error!("Config file not found: {}", config_file);
                panic!("Configuration read");
            }
        }
    }
}
