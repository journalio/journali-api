use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::items::ItemTypeNames;
use crate::schema::todos;

use super::reex_diesel::*;
use super::{ItemLike, ItemType};

#[derive(Queryable, Serialize, Insertable)]
pub struct Todo {
    pub id: Uuid,
    pub item_type: i16,
    pub title: String,
}

#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub page_id: Uuid,
}

impl ItemLike for NewTodo {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        ItemTypeNames::Todo as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        Some(self.page_id)
    }

    fn parent_type(&self) -> Option<i16> {
        Some(ItemTypeNames::Page as i16)
    }
}

impl Todo {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_todo);
    }

    pub fn find(id: &Uuid, conn: &PgConnection) -> QueryResult<Self> {
        todos::table.filter(todos::columns::id.eq(id)).get_result(conn)
    }
    pub(crate) fn create(
        new_todo: &NewTodo,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let item = new_todo.as_new_item();
        let todo = Self {
            id: item.id,
            item_type: item.item_type,
            title: new_todo.title.clone(),
        };

        item.create(conn)?;
        diesel::insert_into(todos::table).values(&todo).get_result(conn)
    }
}

mod routes {
    use actix_web::{post, web, Error, HttpResponse};

    use crate::utils::responsable::Responsable;
    use crate::{database::exec_on_pool, DbPool};

    use super::{NewTodo, Todo};

    #[post("/todos")]
    pub async fn create_todo(
        pool: web::Data<DbPool>,
        form: web::Json<NewTodo>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| Todo::create(&form, &conn))
            .await
            .into_response()
    }
}
