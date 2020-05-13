#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use diesel::{
    pg,
    r2d2::{self, ConnectionManager},
};
use env_logger::Env;
use serde::Serialize;

use journali_api::{
    items::{
        page::Page, text_field::TextField, todo::Todo, todo_item::TodoItem,
    },
    DbPool,
};

#[derive(Serialize)]
struct ErrMsg {
    status: String,
    message: String,
}

fn create_pool() -> DbPool {
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<pg::PgConnection>::new(conn_spec);

    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}

#[actix_rt::main]
#[cfg_attr(tarpaulin, skip)]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    dotenv::dotenv().ok();

    HttpServer::new(move || {
        App::new()
            .data(create_pool())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .default_service(web::to(|| {
                HttpResponse::NotFound().json(ErrMsg {
                    status: "404".to_string(),
                    message: "Page not found.".to_string(),
                })
            }))
            .service(
                web::scope("/api")
                    .configure(Page::routes)
                    .configure(Todo::routes)
                    .configure(TodoItem::routes)
                    .configure(TextField::routes),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
