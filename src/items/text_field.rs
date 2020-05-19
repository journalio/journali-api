use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{items::ItemTypeNames, schema::text_fields};

use super::{
    crud::{Create, Find, Update},
    reex_diesel::*,
    ItemLike, ItemType,
};

#[derive(Queryable, Serialize, Insertable)]
pub struct TextField {
    pub id: Uuid,
    pub item_type: i16,
    pub text: String,
}

#[derive(Deserialize)]
pub struct NewTextField {
    pub text: String,
    pub page_id: Uuid,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "text_fields"]
pub struct UpdateTextField {
    pub text: String,
}

impl ItemLike for NewTextField {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        ItemTypeNames::TextField as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        Some(self.page_id)
    }

    fn parent_type(&self) -> Option<i16> {
        Some(ItemTypeNames::Page as i16)
    }
}

impl Create for TextField {
    type Create = NewTextField;

    fn create(
        new_text_field: &NewTextField,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let item = new_text_field.as_new_item();
        let text_field = Self {
            id: item.id,
            item_type: item.item_type,
            text: new_text_field.text.clone(),
        };

        item.create(conn)?;
        diesel::insert_into(text_fields::table)
            .values(&text_field)
            .get_result(conn)
    }
}

impl Find for TextField {
    fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        text_fields::table
            .filter(text_fields::columns::id.eq(id))
            .filter(text_fields::item_type.eq(ItemTypeNames::TextField as i16))
            .get_result(conn)
    }
}

impl Update for TextField {
    type Update = UpdateTextField;

    fn update(
        id: Uuid,
        update_text_field: &UpdateTextField,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(
            text_fields::table.filter(text_fields::columns::id.eq(id)).filter(
                text_fields::item_type.eq(ItemTypeNames::TextField as i16),
            ),
        )
        .set(update_text_field)
        .get_result(conn)
    }
}

impl TextField {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_text_field);
        cfg.service(routes::find_text_field);
        cfg.service(routes::update_text_field);
    }
}

mod routes {
    use actix_web::{get, patch, post, web, Error, HttpResponse};
    use uuid::Uuid;

    use crate::{items::crud::Crudder, DbPool};

    use super::{NewTextField, TextField, UpdateTextField};

    #[post("/text_fields")]
    pub async fn create_text_field(
        pool: web::Data<DbPool>,
        form: web::Json<NewTextField>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<TextField>::create(form.into_inner(), &pool).await
    }

    #[get("/text_fields/{id}")]
    pub async fn find_text_field(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<TextField>::find(id.into_inner(), &pool).await
    }

    #[patch("/text_fields/{id}")]
    pub async fn update_text_field(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        form: web::Json<UpdateTextField>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<TextField>::update(id.into_inner(), form.into_inner(), &pool)
            .await
    }
}
