use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{items::ItemTypeNames, schema::pages};

use super::{
    crud::{Create, Find, Update},
    reex_diesel::*,
    ItemLike, ItemType,
};

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

#[derive(AsChangeset, Deserialize)]
#[table_name = "pages"]
pub struct UpdatePage {
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

impl Create for Page {
    type Create = NewPage;

    fn create(new_page: &NewPage, conn: &PgConnection) -> QueryResult<Self> {
        let item = new_page.as_new_item();
        let page = Self {
            id: item.id,
            item_type: item.item_type,
            title: new_page.title.clone(),
        };

        item.create(conn)?;
        diesel::insert_into(pages::table).values(&page).get_result(conn)
    }
}

impl Find for Page {
    fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        pages::table
            .filter(pages::id.eq(id))
            .filter(pages::item_type.eq(ItemTypeNames::Page as i16))
            .get_result(conn)
    }
}

impl Update for Page {
    type Update = UpdatePage;

    fn update(
        id: Uuid,
        form: &UpdatePage,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(
            pages::table
                .filter(pages::columns::id.eq(id))
                .filter(pages::item_type.eq(ItemTypeNames::Page as i16)),
        )
        .set(form)
        .get_result(conn)
    }
}

impl Page {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_page);
        cfg.service(routes::find_page);
        cfg.service(routes::update_pages);
    }
}

mod routes {
    use actix_web::{get, patch, post, web, Error, HttpResponse};
    use uuid::Uuid;

    use crate::{items::crud::Crudder, DbPool};

    use super::{NewPage, Page, UpdatePage};

    #[post("/pages")]
    pub async fn create_page(
        pool: web::Data<DbPool>,
        form: web::Json<NewPage>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Page>::create(form.into_inner(), &pool).await
    }

    #[get("/pages/{id}")]
    pub async fn find_page(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Page>::find(id.into_inner(), &pool).await
    }

    #[patch("/pages/{id}")]
    pub async fn update_pages(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        form: web::Json<UpdatePage>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Page>::update(id.into_inner(), form.into_inner(), &pool).await
    }
}
