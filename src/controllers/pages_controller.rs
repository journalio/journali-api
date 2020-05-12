use std::future::Future;

use actix_web::dev::Service;
use actix_web::{get, post, web, Error, HttpResponse};
use diesel::{PgConnection, QueryResult};
use uuid::Uuid;

use crate::models::Page;
use crate::{models, repositories, DbPool};

async fn exec_on_pool<T: Send + Sync>(
    pool: web::Data<DbPool>,
    f: impl FnOnce(&PgConnection) -> QueryResult<T>,
) -> impl Future<Output = QueryResult<T>> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || f(&conn))
}

#[post("/pages")]
pub async fn create_page(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewPage>,
) -> Result<HttpResponse, Error> {
    let page: Page = exec_on_pool(pool, move |conn| -> QueryResult<Page> {
        repositories::pages_repository::create(&form, &conn)
    })
    .await
    .map_err(|e| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(page))
}

#[get("/pages/{id}")]
pub async fn find_page(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    let page = web::block(move || -> QueryResult<Page> {
        repositories::pages_repository::get(id.into_inner(), &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(page))
}

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(create_page).service(find_page);
}
