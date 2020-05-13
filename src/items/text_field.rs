use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::items::ItemTypeNames;
use crate::schema::text_fields;

use super::reex_diesel::*;
use super::{ItemLike, ItemType};

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

impl TextField {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_text_field);
    }

    pub(crate) fn create(
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

mod routes {
    use actix_web::{post, web, Error, HttpResponse};

    use crate::{database::exec_on_pool, DbPool};

    use super::{NewTextField, TextField};

    #[post("/text_fields")]
    pub async fn create_text_field(
        pool: web::Data<DbPool>,
        form: web::Json<NewTextField>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| TextField::create(&form, &conn))
            .await
            .map(|text_field| HttpResponse::Ok().json(text_field))
            .map_err(|_| HttpResponse::InternalServerError().finish().into())
    }
}
