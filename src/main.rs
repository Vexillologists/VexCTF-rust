#![warn(clippy::pedantic)]
#![feature(panic_info_message, decl_macro, proc_macro_hygiene)]
// Some things need to be passed by value in Rocket
#![allow(clippy::needless_pass_by_value)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_json;

mod schema;
mod systems;

fn main() {
    // We are a go!
    systems::init();
}
