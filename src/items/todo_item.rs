use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::items::ItemTypeNames;
use crate::schema::todo_items;

use super::reex_diesel::*;
use super::{ItemLike, ItemType};

#[derive(Queryable, Serialize, Insertable)]
pub struct TodoItem {
    pub id: Uuid,
    pub item_type: ItemType,
    pub title: String,
    pub is_checked: bool,
}

#[derive(Deserialize)]
pub struct NewTodoItem {
    pub title: String,
    pub todo_id: Uuid,
}

impl ItemLike for NewTodoItem {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        ItemTypeNames::TodoItem as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        Some(self.todo_id)
    }

    fn parent_type(&self) -> Option<i16> {
        Some(ItemTypeNames::Todo as i16)
    }
}

impl TodoItem {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_todo_item);
    }

    pub(crate) fn create(
        new_todo: &NewTodoItem,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let item = new_todo.as_new_item();
        let todo = Self {
            id: item.id,
            item_type: item.item_type,
            title: new_todo.title.clone(),
            is_checked: false,
        };

        item.create(conn)?;
        diesel::insert_into(todo_items::table).values(&todo).get_result(conn)
    }
}

mod routes {
    use actix_web::{post, web, Error, HttpResponse};

    use crate::utils::responsable::Responsable;
    use crate::{database::exec_on_pool, DbPool};

    use super::{NewTodoItem, TodoItem};

    #[post("/todo-items")]
    pub async fn create_todo_item(
        pool: web::Data<DbPool>,
        form: web::Json<NewTodoItem>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(pool, move |conn| TodoItem::create(&form, &conn))
            .await
            .into_response()
    }
}
