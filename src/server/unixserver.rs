use std::io::prelude::*;
use std::net::{Shutdown};
use std::os::unix::net::{UnixStream, UnixListener};
use std::fs;
use std::os::unix::fs::FileTypeExt;
use log::{info, warn, error};
use std::sync::{Arc};

extern crate threadpool;
use threadpool::ThreadPool;

extern crate dashmap;
use dashmap::DashMap;

use super::super::config::Configuration;

fn handle_connection<F: super::MessageHandler>(mut stream: UnixStream, handler: F, map: &Arc<DashMap<String, Vec<char>>>) {
    let mut data = [0 as u8; 4096];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                info!("Client connection closed.");
                match stream.shutdown(Shutdown::Both) {Ok(_) => {}, Err(_) => {}};
                return;
            }
            let handle_out = &handler(size, &data, &map);
            stream.write(handle_out.as_bytes()).unwrap();
            true
        },
        Err(_) => {
            warn!("Error in Unix Socket connection, terminating it");
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub struct UnixServer {
    pub address: String,
    listener: UnixListener,
    pool: ThreadPool,
}

impl UnixServer {
    pub fn new(address: &str, config: &Configuration) -> Self {
        match fs::metadata(address) {
            Ok(meta) => {
                if meta.file_type().is_socket() {
                    fs::remove_file(address).unwrap();
                } else {
                    error!("Socket address provided exists and it's not a socket: {}", address);
                    panic!("Incorrect unix socket provided, unable to continue");
                }
            },
            Err(err) => {
                error!("{}", err)
            },
        }

        UnixServer {
            address: String::from(address), 
            pool: ThreadPool::new(32), 
            listener: UnixListener::bind(address).unwrap(),
        }
    }

    pub fn listen<F: super::MessageHandler> (&self, handler: F, map: Arc<DashMap<String, Vec<char>>>) {
        info!("Listening on socket {}", self.address);

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let m_ref = map.clone();

            self.pool.execute(move || {
                info!("Client connection");
                handle_connection(stream, handler, &m_ref);
            });
        }
    }
}
