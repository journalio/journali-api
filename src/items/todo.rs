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
    pub page_id: Uuid,
    pub title: String,
}

#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub page_id: Uuid,
}

impl ItemLike for Todo {
    fn id(&self) -> Uuid {
        self.id
    }

    fn item_type(&self) -> ItemType {
        ItemTypeNames::Todo as i16
    }
}

impl Todo {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_todo);
    }

    pub(crate) fn create(
        new_todo: &NewTodo,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let todo = Self {
            id: Uuid::new_v4(),
            item_type: ItemTypeNames::Todo as i16,
            page_id: new_todo.page_id,
            title: new_todo.title.clone(),
        };

        todo.as_item().create(conn)?;
        diesel::insert_into(todos::table).values(&todo).get_result(conn)
    }
}

mod routes {
    use actix_web::{post, web, Error, HttpResponse};

    use crate::{database::exec_on_pool, DbPool};

    use super::{NewTodo, Todo};

    #[post("/todos")]
    pub async fn create_todo(
        pool: web::Data<DbPool>,
        form: web::Json<NewTodo>,
    ) -> Result<HttpResponse, Error> {
        let todo: Todo =
            exec_on_pool(pool, move |conn| Todo::create(&form, &conn))
                .await
                .map_err(|_| HttpResponse::InternalServerError().finish())?;

        Ok(HttpResponse::Ok().json(todo))
    }
}
