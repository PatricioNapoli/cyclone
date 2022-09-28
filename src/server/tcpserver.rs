use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, Shutdown};
use log::{info, warn};
use std::sync::{Arc};

extern crate threadpool;
use threadpool::ThreadPool;

extern crate dashmap;
use dashmap::DashMap;

use super::super::config::Configuration;

fn handle_connection<F: super::MessageHandler>(mut stream: TcpStream, handler: F, map: &Arc<DashMap<String, Vec<char>>>) {
    let mut data = [0 as u8; 4096];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                info!("Client connection closed: {}", stream.peer_addr().unwrap());
                match stream.shutdown(Shutdown::Both) {Ok(_) => {}, Err(_) => {}};
                return;
            }
            let handle_out = &handler(size, &data, &map);
            stream.write(handle_out.as_bytes()).unwrap();
            true
        },
        Err(_) => {
            warn!("Error in TCP connection with {}, terminating", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub struct TcpServer {
    pub address: String,
    pub port: String,
    config: Configuration,
    listener: TcpListener,
    worker_pool: ThreadPool,
}

impl TcpServer {
    pub fn new(address: &str, port: &str, config: Configuration) -> Self {
        TcpServer {
            address: String::from(address), 
            port: String::from(port), 
            config,
            worker_pool: ThreadPool::with_name("workers".to_string(), 16),
            listener: TcpListener::bind(format!("{}:{}", address, port)).unwrap(),
        }
    }

    pub fn listen<F: super::MessageHandler> (&self, handler: F, map: Arc<DashMap<String, Vec<char>>>) {
        info!("Listening on TCP {}:{}", self.address, self.port);

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            let m_ref = map.clone();

            self.worker_pool.execute(move || {
                info!("Client connection from: {}", stream.peer_addr().unwrap());
                handle_connection(stream, handler, &m_ref);
            });
        }
    }
}
