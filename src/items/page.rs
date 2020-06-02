use core::convert::AsRef;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    items::{ItemTypeNames, TypeMarker},
    schema::pages,
};

use super::{
    crud::{Create, Delete, Find, Update},
    crud2::{crud2http, raw_crud, ModelFromPartial},
    item::OwnedItem,
    reex_diesel::*,
    ItemLike, ItemType,
};

#[derive(Queryable, Serialize, Insertable)]
pub struct Page {
    pub id: Uuid,
    pub item_type: ItemType,
    pub title: String,
}

#[derive(Clone, Deserialize)]
pub struct NewPage {
    pub title: String,
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "pages"]
pub struct UpdatePage {
    pub title: String,
}

impl TypeMarker for Page {
    const TYPE: ItemTypeNames = ItemTypeNames::Page;
}

impl ItemLike for NewPage {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        Page::TYPE as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        None
    }

    fn parent_type(&self) -> Option<i16> {
        None
    }
}

impl raw_crud::Create for Page {
    fn create(self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(pages::table).values(&self).get_result(conn)
    }
}

impl ModelFromPartial<NewPage> for Page {
    fn from_partial(partial: NewPage, item: &crate::items::item::Item) -> Self {
        Self { id: item.id, item_type: item.item_type, title: partial.title }
    }
}

impl raw_crud::Update<UpdatePage> for Page {
    fn update(
        id: Uuid,
        update_page: UpdatePage,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(
            pages::table
                .filter(pages::columns::id.eq(id))
                .filter(pages::item_type.eq(Self::TYPE as i16)),
        )
        .set(update_page)
        .get_result(conn)
    }
}

use diesel::BelongingToDsl;

impl Find<(Uuid, crate::users::user::User)> for Page {
    fn find(
        (id, user): (Uuid, crate::users::user::User),
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        use crate::schema;
        super::item::Item::belonging_to(&user)
            .inner_join(
                schema::pages::table
                    .on(schema::pages::id.eq(schema::items::id)),
            )
            .get_result::<(super::item::Item, Page)>(conn)
            .map(|(_, page)| page)
    }
}

impl Delete for Page {
    fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<()> {
        super::Item::delete::<Self>(id, conn)
    }
}

impl Page {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_page);
        cfg.service(routes::find_page);
        cfg.service(routes::update_page);
        cfg.service(routes::delete_page);
    }
}

mod routes {
    use actix_web::{
        delete, get, patch, post, web, Error, HttpRequest, HttpResponse,
    };
    use uuid::Uuid;

    use crate::{
        items::{crud::Crudder, crud2::crud2http, item::OwnedItem},
        DbPool,
    };

    use super::{NewPage, Page, UpdatePage};

    #[post("/pages")]
    pub async fn create_page(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        form: web::Json<NewPage>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::create::<Page, _>(form.into_inner(), user, &pool).await
    }

    #[get("/pages/{id}")]
    pub async fn find_page(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        Crudder::<Page>::find((id.into_inner(), user), &pool).await
    }

    #[patch("/pages/{id}")]
    pub async fn update_page(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
        form: web::Json<UpdatePage>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        crud2hhtp::create::<Page, _>(
            id.into_inner(),
            form.into_inner(),
            user,
            &pool,
        )
        .await
    }

    #[delete("/pages/{id}")]
    pub async fn delete_page(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Page>::delete(id.into_inner(), &pool).await
    }
}
