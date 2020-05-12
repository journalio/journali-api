use actix_web::{post, web, Error, HttpResponse};
use diesel::QueryResult;

use crate::models::Page;
use crate::{models, repositories, DbPool};

#[post("/pages")]
pub async fn create_page(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewPage>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    // use web::block to offload blocking Diesel code without blocking server thread
    let page = web::block(move || -> QueryResult<Page> {
        repositories::pages_repository::create(&form, &conn)
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().json(page))
}
