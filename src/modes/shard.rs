use std::sync::{Arc};
use std::iter::FromIterator;

extern crate dashmap;
use dashmap::DashMap;

use super::super::server;

impl<U> server::MessageHandler for U where U: Fn(usize, &[u8], &Arc<DashMap<String, Vec<char>>>) -> String + Send + Sync + Copy + 'static {}

pub fn handle_message_shard(size: usize, data: &[u8], map: &Arc<DashMap<String, Vec<char>>>) -> String {
    let s = std::str::from_utf8(data).unwrap();

    if s.starts_with("set") {
        let split = s.split("\n").collect::<Vec<_>>()[0].split(" ").collect::<Vec<_>>();
        let k = split[1];
        let v = split[2];
        map.insert(String::from(k), String::from(v).chars().collect());
        return format!("Set {} to {}\n", k, v);
    }

    if s.starts_with("get") {
        let split = s.split("\n").collect::<Vec<_>>()[0].split(" ").collect::<Vec<_>>();
        let k = String::from(split[1]);
        let v = &*map.get(&k).unwrap();
        return format!("{}\n", String::from_iter(v));
    }

    return String::from_utf8_lossy(&data[0..size]).to_string();
}
