#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use diesel::{
    pg,
    r2d2::{self, ConnectionManager},
};
use env_logger::Env;

const NOT_FOUND: &str =
    "{\"status\": \"Not Found\", \"Message\": \"Page not found\"}";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<pg::PgConnection>::new(connspec);
    let pool =
        r2d2::Pool::builder().build(manager).expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
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
