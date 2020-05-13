use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::items::ItemTypeNames;
use crate::schema::todo_items;

use super::reex_diesel::*;
use super::{ItemLike, ItemType};

#[derive(Queryable, Serialize, Insertable)]
pub struct TodoItem {
    pub id: Uuid,
    pub item_type: i16,
    pub todo_id: Uuid,
    pub title: String,
    pub is_checked: bool,
}

#[derive(Deserialize)]
pub struct NewTodoItem {
    pub title: String,
    pub todo_id: Uuid,
}

impl ItemLike for TodoItem {
    fn id(&self) -> Uuid {
        self.id
    }

    fn item_type(&self) -> ItemType {
        ItemTypeNames::TodoItem as i16
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
        let todo = Self {
            id: Uuid::new_v4(),
            item_type: ItemTypeNames::TodoItem as i16,
            todo_id: new_todo.todo_id,
            title: new_todo.title.clone(),
            is_checked: false,
        };

        todo.as_item().create(conn)?;
        diesel::insert_into(todo_items::table).values(&todo).get_result(conn)
    }
}

mod routes {
    use actix_web::{post, web, Error, HttpResponse};

    use crate::{database::exec_on_pool, DbPool};

    use super::{NewTodoItem, TodoItem};

    #[post("/todo-items")]
    pub async fn create_todo_item(
        pool: web::Data<DbPool>,
        form: web::Json<NewTodoItem>,
    ) -> Result<HttpResponse, Error> {
        let todo: TodoItem =
            exec_on_pool(pool, move |conn| TodoItem::create(&form, &conn))
                .await
                .map_err(|_| HttpResponse::InternalServerError().finish())?;

        Ok(HttpResponse::Ok().json(todo))
    }
}
