#![feature(proc_macro_hygiene, decl_macro)]
#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use]
extern crate rocket;

#[catch(404)]
fn not_found() -> String {
    "{\"status\": \"Not Found\", \"Message\": \"Page not found\"}".to_string()
}

fn main() {
    rocket::ignite()
        .mount("/api", routes![journali_api::hello])
        .register(catchers![not_found])
        .launch();
}
