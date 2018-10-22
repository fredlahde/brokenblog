#![feature(custom_attribute)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate rocket;
extern crate serde;
extern crate serde_json;

extern crate chrono;

#[macro_use]
extern crate lazy_static;

mod cache;
mod renderer;
mod post;

use cache::Cache;
use std::sync::Mutex;

const FILE: &'static str = "posts.json";

lazy_static! {
    static ref CHOLDER: Mutex<Cache> = Mutex::new(Cache::new());
}

fn main() {
    CHOLDER
        .lock()
        .expect("not able to lock cache")
        .init(FILE)
        .expect("not able to init cache");

    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> String {
    let mut cache = match CHOLDER.lock() {
        Ok(c) => c,
        Err(_e) => {
            panic!("failed to acuqire Mutex");
        }
    };

    cache.add(123, "fred").unwrap();
    cache.add(125, "fred").unwrap();

    let concat = cache
        .get_cached_content()
        .iter()
        .fold(String::new(), |mut acc, x| {
            acc.push_str(&x.content_parsed);
            acc
        });

    concat
}
