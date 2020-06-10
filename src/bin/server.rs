#![forbid(unsafe_code)]
#![cfg_attr(feature = "strict", deny(warnings))]

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use env_logger::Env;
use serde::Serialize;

use journali_api::{
    create_pool,
    items::{
        item::Item, page::Page, text_field::TextField, todo::Todo,
        todo_item::TodoItem,
    },
    tags::tags::Tag,
    users::User,
    utils::validator,
    version,
};

#[derive(Serialize)]
struct ErrMsg {
    status: String,
    message: String,
}

#[actix_rt::main]
#[cfg_attr(tarpaulin, skip)]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "full");

    dotenv::dotenv().ok();

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
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
                    .configure(User::routes)
                    .service(version)
                    .service(
                        web::scope("")
                            .wrap(auth)
                            .configure(Item::routes)
                            .configure(Page::routes)
                            .configure(Todo::routes)
                            .configure(TodoItem::routes)
                            .configure(TextField::routes)
                            .configure(Tag::routes)
                            .configure(User::route_me),
                    ),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
