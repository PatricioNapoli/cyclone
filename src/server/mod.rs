use std::sync::{Arc};
extern crate dashmap;
use dashmap::DashMap;

pub trait MessageHandler : Fn(usize, &[u8], &Arc<DashMap<String, Vec<char>>>) -> String + Send + Sync + Copy + 'static {}

pub mod tcpserver;
pub use tcpserver::*;

pub mod unixserver;
pub use unixserver::*;
