#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rocket;
extern crate serde;
extern crate serde_json;

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

    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
