use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::items;

use super::reex_diesel::*;
use super::{ItemLike, ItemType};

#[derive(Insertable, Queryable, Copy, Clone, Serialize)]
pub struct Item {
    pub(crate) id: Uuid,
    pub(crate) item_type: ItemType,
    pub(crate) parent_id: Option<Uuid>,
    pub(crate) parent_type: Option<ItemType>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl ItemLike for Item {
    fn id(&self) -> Uuid {
        self.id
    }

    fn item_type(&self) -> ItemType {
        self.item_type
    }

    fn parent_id(&self) -> Option<Uuid> {
        self.parent_id
    }

    fn parent_type(&self) -> Option<ItemType> {
        self.parent_type
    }

    fn as_item(&self) -> Item {
        *self
    }
}

impl Default for Item {
    fn default() -> Self {
        Item {
            id: Uuid::default(),
            item_type: 0,
            parent_id: None,
            parent_type: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Item {
    pub(super) fn create(&self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(items::table).values(self).get_result(conn)
    }

    pub(super) fn update_parent(
        id: &Uuid,
        form: &UpdateParentRequest,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(items::table.filter(items::columns::id.eq(id)))
            .set(form)
            .get_result(conn)
    }
}

#[derive(AsChangeset, Deserialize)]
#[table_name = "items"]
pub struct UpdateParentRequest {
    pub(crate) parent_id: Uuid,
    pub(crate) parent_type: ItemType,
}

impl Item {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::update_item_parent);
    }
}

mod routes {
    use actix_web::{patch, web, Error, HttpResponse};
    use uuid::Uuid;

    use crate::items::item::UpdateParentRequest;
    use crate::{database::exec_on_pool, DbPool};

    use super::Item;

    #[patch("/items/{id}")]
    pub async fn update_item_parent(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        form: web::Json<UpdateParentRequest>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| {
            Item::update_parent(&id.into_inner(), &form, &conn)
        })
        .await
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|_| HttpResponse::InternalServerError().finish().into())
    }
}
