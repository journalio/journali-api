use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::items::ItemTypeNames;
use crate::schema::pages;

use super::reex_diesel::*;
use super::{ItemLike, ItemType};

#[derive(Queryable, Serialize, Insertable)]
pub struct Page {
    pub id: Uuid,
    pub item_type: ItemType,
    pub title: String,
}

#[derive(Deserialize)]
pub struct NewPage {
    pub title: String,
}

impl ItemLike for NewPage {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        100
    }

    fn parent_id(&self) -> Option<Uuid> {
        None
    }

    fn parent_type(&self) -> Option<i16> {
        None
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
        let item = new_page.as_new_item();
        let page = Self {
            id: item.id,
            item_type: item.item_type,
            title: new_page.title.clone(),
        };

        item.create(conn)?;
        diesel::insert_into(pages::table).values(&page).get_result(conn)
    }

    pub(crate) fn get(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        pages::table
            .filter(pages::id.eq(id))
            .filter(pages::item_type.eq(ItemTypeNames::Page as i16))
            .get_result(conn)
    }
}

mod routes {
    use actix_web::{get, post, web, Error, HttpResponse};
    use uuid::Uuid;

    use crate::items::Responsable;
    use crate::{database::exec_on_pool, DbPool};

    use super::{NewPage, Page};

    #[post("/pages")]
    pub async fn create_page(
        pool: web::Data<DbPool>,
        form: web::Json<NewPage>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| Page::create(&form, &conn))
            .await
            .into_response()
    }

    #[get("/pages/{id}")]
    pub async fn find_page(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, |conn| Page::get(id.into_inner(), &conn))
            .await
            .into_response()
    }
}
