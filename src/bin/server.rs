#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

use actix_web::{web, App, HttpResponse, HttpServer};

const NOT_FOUND: &str =
    "{\"status\": \"Not Found\", \"Message\": \"Page not found\"}";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").service(journali_api::hello))
            .default_service(web::to(|| {
                HttpResponse::NotFound().body(NOT_FOUND)
            }))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
