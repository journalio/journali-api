use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    items::{ItemTypeNames, TypeMarker},
    schema::todo_items,
};

use super::{
    crud2::{raw_crud, ModelFromPartial},
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
    pub is_checked: bool,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "todo_items"]
pub struct UpdateTodoItem {
    pub title: String,
}

impl TypeMarker for TodoItem {
    const TYPE: ItemTypeNames = ItemTypeNames::TodoItem;
}

impl ItemLike for NewTodoItem {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        TodoItem::TYPE as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        None
    }

    fn parent_type(&self) -> Option<i16> {
        None
    }
}

impl raw_crud::Create for TodoItem {
    fn create(self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(todo_items::table).values(&self).get_result(conn)
    }
}

impl ModelFromPartial<NewTodoItem> for TodoItem {
    fn from_partial(
        partial: NewTodoItem,
        item: &crate::items::item::Item,
    ) -> Self {
        Self {
            id: item.id,
            item_type: item.item_type,
            title: partial.title,
            is_checked: partial.is_checked,
        }
    }
}

impl raw_crud::Update<UpdateTodoItem> for TodoItem {
    fn update(
        id: Uuid,
        update_todo_item: UpdateTodoItem,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(
            todo_items::table
                .filter(todo_items::columns::id.eq(id))
                .filter(todo_items::item_type.eq(Self::TYPE as i16)),
        )
        .set(update_todo_item)
        .get_result(conn)
    }
}

impl raw_crud::Find for TodoItem {
    fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        todo_items::table
            .filter(todo_items::columns::id.eq(id))
            .filter(todo_items::item_type.eq(Self::TYPE as i16))
            .get_result(conn)
    }
}

impl raw_crud::Delete for TodoItem {
    fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<()> {
        super::Item::delete::<Self>(id, conn)
    }
}

impl TodoItem {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_todo_item);
        cfg.service(routes::find_todo_item);
        cfg.service(routes::update_todo_item);
        cfg.service(routes::delete_todo_item);
    }
}

mod routes {
    use actix_web::{
        delete, get, patch, post, web, Error, HttpRequest, HttpResponse,
    };
    use uuid::Uuid;

    use crate::{items::crud2::crud2http, DbPool};

    use super::{NewTodoItem, TodoItem, UpdateTodoItem};

    #[post("/todo_items")]
    pub async fn create_todo_item(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        form: web::Json<NewTodoItem>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::create::<TodoItem, _>(form.into_inner(), user, &pool).await
    }

    #[get("/todo_items/{id}")]
    pub async fn find_todo_item(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::find::<TodoItem>(id.into_inner(), user, &pool).await
    }

    #[patch("/todo_items/{id}")]
    pub async fn update_todo_item(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
        form: web::Json<UpdateTodoItem>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        crud2http::update::<TodoItem, _>(
            id.into_inner(),
            form.into_inner(),
            user,
            &pool,
        )
        .await
    }

    #[delete("/todo_items/{id}")]
    pub async fn delete_todo_item(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::delete::<TodoItem>(id.into_inner(), user, &pool).await
    }
}
