use std::{env, process};
use log::{info, error};
use std::sync::{Arc};

extern crate dashmap;
use dashmap::DashMap;

#[macro_use]
mod utils;
mod logging;
mod config;

mod server;
mod modes;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

fn run_mode(mode: &str, config_file: &str) {
    let mut file = String::from(config_file);
    if file == "".to_string() {
        file = "config.yml".to_string();
    }

    let config = config::Configuration::new(&file);
    let data: DashMap<String, Vec<char>> = DashMap::new();
    let data = Arc::new(data);

    match mode {
        // index to shards, set to shards and broadcast to slaves
        "coordinator" => {
            server::TcpServer::new("127.0.0.1", "1717", config).listen(modes::handle_message_shard, data);
        },
        // standalone, set, mvcc and disk storage dumps
        "shard" => {
            server::TcpServer::new("127.0.0.1", "1717", config).listen(modes::handle_message_shard, data);
        },
        // cache, measure real total size, track when setting values, mvcc rejection on sets (local or remote)
        "slave" => {
            server::UnixServer::new(&config.socket, &config).listen(modes::handle_message_shard, data);
        },
        _ => {
            error!("Cyclone mode {} does not exist, exiting!", mode);
            process::exit(1);
        }
    }
}

fn main() {
    match logging::init_logging() {
        Ok(_) => {
            info!("/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/");
            info!("/\\/\\/\\/\\/ Cyclone \\/\\/\\/\\/\\");
            info!("/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/\\/");
            println!("", )
        }
        Err(_) => {}
    }

    info!("Author: {}", AUTHOR);
    info!("Version: {}", VERSION);
    println!("", );

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            info!("No mode or config file specified, falling back to Master Single Shard");
            run_mode("shard", "");
        },
        2 => {
            run_mode(&args[1], "");
        },
        3 => {
            run_mode(&args[1], &args[2]);
        },
        _ => {
            error!("Too many program arguments received.. exiting now");
            process::exit(1);
        }
    }
}
