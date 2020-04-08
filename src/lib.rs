#![feature(proc_macro_hygiene, decl_macro)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]
//! This is a library containing functionality for the journali
//! backend.
//!
//! This library exists for documentation purposes.

#[macro_use]
extern crate rocket;

#[get("/hello/<name>/<age>")]
pub fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}
