use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::pages;

use super::reex_diesel::*;
use super::{ItemLike, ItemType};

#[derive(Queryable, Serialize, Insertable)]
pub struct Page {
    pub id: Uuid,
    pub item_type: i16,
    pub title: String,
}

#[derive(Deserialize)]
pub struct NewPage {
    pub title: String,
}

impl ItemLike for Page {
    fn id(&self) -> Uuid {
        self.id
    }

    fn item_type(&self) -> ItemType {
        100
    }
}

impl Page {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_page);
        cfg.service(routes::find_page);
    }
}

impl Page {
    pub(crate) fn create(
        new_page: &NewPage,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let page = Self {
            id: Uuid::new_v4(),
            item_type: 100,
            title: new_page.title.clone(),
        };

        page.as_item().create(conn)?;
        diesel::insert_into(pages::table).values(&page).get_result(conn)
    }

    pub(crate) fn get(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        pages::table.filter(pages::id.eq(id)).get_result(conn)
    }
}

mod routes {
    use actix_web::{get, post, web, Error, HttpResponse};
    use uuid::Uuid;

    use crate::{database::exec_on_pool, DbPool};

    use super::{NewPage, Page};

    #[post("/pages")]
    pub async fn create_page(
        pool: web::Data<DbPool>,
        form: web::Json<NewPage>,
    ) -> Result<HttpResponse, Error> {
        let page: Page =
            exec_on_pool(pool, move |conn| Page::create(&form, &conn))
                .await
                .map_err(|_| HttpResponse::InternalServerError().finish())?;

        Ok(HttpResponse::Ok().json(page))
    }

    #[get("/pages/{id}")]
    pub async fn find_page(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let page = exec_on_pool(pool, |conn| Page::get(id.into_inner(), &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

        Ok(HttpResponse::Ok().json(page))
    }
}
