use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::items::Items;
use crate::{
    items::{ItemTypeNames, TypeMarker},
    schema::text_fields,
};

use super::{
    crud2::{raw_crud, ModelFromPartial},
    reex_diesel::*,
    ItemLike, ItemType,
};

#[derive(Queryable, Serialize, Insertable)]
pub struct TextField {
    pub id: Uuid,
    pub item_type: i16,
    pub text: String,
    pub coord_x: i32,
    pub coord_y: i32,
}

#[derive(Deserialize)]
pub struct NewTextField {
    pub text: String,
    pub page_id: Uuid,
    pub coord_x: i32,
    pub coord_y: i32,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "text_fields"]
pub struct UpdateTextField {
    pub text: String,
    pub coord_x: i32,
    pub coord_y: i32,
}

impl TypeMarker for TextField {
    const TYPE: ItemTypeNames = ItemTypeNames::TextField;
}

impl ItemLike for NewTextField {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        TextField::TYPE as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        Some(self.page_id)
    }

    fn parent_type(&self) -> Option<i16> {
        Some(ItemTypeNames::Page as i16)
    }
}

impl From<TextField> for Items {
    fn from(text_field: TextField) -> Self {
        Self::TextField(text_field)
    }
}

impl raw_crud::Create for TextField {
    fn create(self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(text_fields::table).values(&self).get_result(conn)
    }
}

impl ModelFromPartial<NewTextField> for TextField {
    fn from_partial(
        partial: NewTextField,
        item: &crate::items::item::Item,
    ) -> Self {
        Self {
            id: item.id,
            item_type: item.item_type,
            text: partial.text,
            coord_x: partial.coord_x,
            coord_y: partial.coord_y,
        }
    }
}

impl raw_crud::Update<UpdateTextField> for TextField {
    fn update(
        id: Uuid,
        update_text_field: UpdateTextField,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(
            text_fields::table
                .filter(text_fields::columns::id.eq(id))
                .filter(text_fields::item_type.eq(Self::TYPE as i16)),
        )
        .set(update_text_field)
        .get_result(conn)
    }
}

impl raw_crud::Find for TextField {
    fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        text_fields::table
            .filter(text_fields::columns::id.eq(id))
            .filter(text_fields::item_type.eq(Self::TYPE as i16))
            .get_result(conn)
    }
}

impl raw_crud::Delete for TextField {
    fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<()> {
        super::Item::delete::<Self>(id, conn)
    }
}

impl TextField {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_text_field);
        cfg.service(routes::find_text_field);
        cfg.service(routes::update_text_field);
        cfg.service(routes::delete_text_field);
    }
}

mod routes {
    use actix_web::{
        delete, get, patch, post, web, Error, HttpRequest, HttpResponse,
    };
    use uuid::Uuid;

    use crate::{items::crud2::crud2http, DbPool};

    use super::{NewTextField, TextField, UpdateTextField};

    #[post("/text_fields")]
    pub async fn create_text_field(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        form: web::Json<NewTextField>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::create::<TextField, _>(form.into_inner(), user, &pool).await
    }

    #[get("/text_fields/{id}")]
    pub async fn find_text_field(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::find::<TextField>(id.into_inner(), user, &pool).await
    }

    #[patch("/text_fields/{id}")]
    pub async fn update_text_field(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
        form: web::Json<UpdateTextField>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        crud2http::update::<TextField, _>(
            id.into_inner(),
            form.into_inner(),
            user,
            &pool,
        )
        .await
    }

    #[delete("/text_fields/{id}")]
    pub async fn delete_text_field(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::delete::<TextField>(id.into_inner(), user, &pool).await
    }
}
