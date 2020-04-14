#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]
//! This is a library containing functionality for the journali
//! backend.
//!
//! This library exists for documentation purposes.

use actix_web::{get, web, HttpResponse, Responder};

#[get("/hello/{name}")]
pub async fn hello(data: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello sailor {}!", data.into_inner()))
}
