use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{items::ItemTypeNames, schema::todo_items};

use super::{
    crud::{Create, Find, Update},
    reex_diesel::*,
    ItemLike, ItemType,
};

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

#[derive(Deserialize, AsChangeset)]
#[table_name = "todo_items"]
pub struct UpdateTodoItem {
    pub title: String,
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

impl Create for TodoItem {
    type Create = NewTodoItem;

    fn create(
        new_todo_item: &NewTodoItem,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        let item = new_todo_item.as_new_item();
        let todo = Self {
            id: item.id,
            item_type: item.item_type,
            title: new_todo_item.title.clone(),
            is_checked: false,
        };

        item.create(conn)?;
        diesel::insert_into(todo_items::table).values(&todo).get_result(conn)
    }
}

impl Find for TodoItem {
    fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        todo_items::table
            .filter(todo_items::columns::id.eq(id))
            .filter(todo_items::item_type.eq(ItemTypeNames::TodoItem as i16))
            .get_result(conn)
    }
}

impl Update for TodoItem {
    type Update = UpdateTodoItem;

    fn update(
        id: Uuid,
        update_todo_item: &UpdateTodoItem,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(
            todo_items::table.filter(todo_items::columns::id.eq(id)).filter(
                todo_items::item_type.eq(ItemTypeNames::TodoItem as i16),
            ),
        )
        .set(update_todo_item)
        .get_result(conn)
    }
}

impl TodoItem {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_todo_item);
        cfg.service(routes::find_todo_item);
        cfg.service(routes::update_todo_item);
    }
}

mod routes {
    use actix_web::{get, patch, post, web, Error, HttpResponse};
    use uuid::Uuid;

    use crate::{items::crud::Crudder, DbPool};

    use super::{NewTodoItem, TodoItem, UpdateTodoItem};

    #[post("/todo-items")]
    pub async fn create_todo_item(
        pool: web::Data<DbPool>,
        form: web::Json<NewTodoItem>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<TodoItem>::create(form.into_inner(), &pool).await
    }

    #[get("/todo-items/{id}")]
    pub async fn find_todo_item(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<TodoItem>::find(id.into_inner(), &pool).await
    }

    #[patch("/todo-items/{id}")]
    pub async fn update_todo_item(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        form: web::Json<UpdateTodoItem>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<TodoItem>::update(id.into_inner(), form.into_inner(), &pool)
            .await
    }
}
