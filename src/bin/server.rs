#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use diesel::{
    pg,
    r2d2::{self, ConnectionManager},
};
use env_logger::Env;

const NOT_FOUND: &str =
    "{\"status\": \"Not Found\", \"Message\": \"Page not found\"}";

mod migrations {
    use super::pg;
    use diesel::prelude::Connection;

    // Embeds the migrations
    diesel_migrations::embed_migrations!();

    pub(super) fn run(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let connection = pg::PgConnection::establish(&db_url)?;
        // This will run the necessary migrations.
        embedded_migrations::run(&connection)?;
        Ok(())
    }
}

#[actix_rt::main]
#[cfg_attr(tarpaulin, skip)]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    // Run migrations.
    migrations::run(&connspec).expect("Failed to run migrations.");

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
