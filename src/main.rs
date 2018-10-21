#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;

mod cache;
use cache::Cache;
use std::process;

const FILE: &'static str = "posts.json";

fn main() {
    let mut cache = Cache::new();
    match cache.init(FILE) {
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
        _ => {}
    }
    cache.add(123, "test").expect("item already in cache");
}
