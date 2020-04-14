#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use env_logger::Env;

const NOT_FOUND: &str =
    "{\"status\": \"Not Found\", \"Message\": \"Page not found\"}";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::scope("/api").service(journali_api::hello))
            .default_service(web::to(|| {
                HttpResponse::NotFound().body(NOT_FOUND)
            }))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
