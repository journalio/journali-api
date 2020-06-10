#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]
//! This is a library containing functionality for the journali
//! backend.
//!
//! This library exists for documentation purposes.

#[macro_use]
extern crate diesel;

use actix_web::{get, HttpResponse, Responder};

pub use database::{create_pool, DbPool};

pub mod utils;

//#[allow(clippy::single_component_path_imports)]
pub mod schema;

mod database;
pub mod items;
pub mod tags;
pub mod users;
/// The sole purpose of this module is to be
/// able to reference the current commit hash.
pub(crate) mod app_version {
    // We need to do this, orelse the environment
    // file will *NOT* be loaded during compilation.
    // This can't be used in an expression, due to
    // the way procedural macro's work.
    load_dotenv::try_load_dotenv!();
    pub const VERSION: &str = env!("RUST_APP_VERSION");
}

#[get("/version")]
pub async fn version() -> impl Responder {
    HttpResponse::Ok().body(app_version::VERSION)
}
