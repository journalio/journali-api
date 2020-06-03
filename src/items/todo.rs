use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::items::Items;
use crate::{
    items::{ItemTypeNames, TypeMarker},
    schema::todos,
};

use super::{
    crud2::{raw_crud, ModelFromPartial},
    reex_diesel::*,
    ItemLike, ItemType,
};

#[derive(Queryable, Serialize, Insertable)]
pub struct Todo {
    pub id: Uuid,
    pub item_type: i16,
    pub title: String,
    pub coord_x: i32,
    pub coord_y: i32,
}

#[derive(Deserialize)]
pub struct NewTodo {
    pub title: String,
    pub todo_id: Uuid,
    pub coord_x: i32,
    pub coord_y: i32,
}

#[derive(Deserialize, AsChangeset)]
#[table_name = "todos"]
pub struct UpdateTodo {
    title: String,
    pub coord_x: i32,
    pub coord_y: i32,
}

impl TypeMarker for Todo {
    const TYPE: ItemTypeNames = ItemTypeNames::Todo;
}

impl ItemLike for NewTodo {
    fn id(&self) -> Uuid {
        Uuid::new_v4()
    }

    fn item_type(&self) -> ItemType {
        Todo::TYPE as i16
    }

    fn parent_id(&self) -> Option<Uuid> {
        None
    }

    fn parent_type(&self) -> Option<i16> {
        None
    }
}

impl From<Todo> for Items {
    fn from(todo: Todo) -> Self {
        Self::Todo(todo)
    }
}

impl raw_crud::Create for Todo {
    fn create(self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(todos::table).values(&self).get_result(conn)
    }
}

impl ModelFromPartial<NewTodo> for Todo {
    fn from_partial(partial: NewTodo, item: &crate::items::item::Item) -> Self {
        Self {
            id: item.id,
            item_type: item.item_type,
            title: partial.title,
            coord_x: partial.coord_x,
            coord_y: partial.coord_y,
        }
    }
}

impl raw_crud::Update<UpdateTodo> for Todo {
    fn update(
        id: Uuid,
        update_todo: UpdateTodo,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(
            todos::table
                .filter(todos::columns::id.eq(id))
                .filter(todos::item_type.eq(Self::TYPE as i16)),
        )
        .set(update_todo)
        .get_result(conn)
    }
}

impl raw_crud::Find for Todo {
    fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        todos::table
            .filter(todos::columns::id.eq(id))
            .filter(todos::item_type.eq(Self::TYPE as i16))
            .get_result(conn)
    }
}

impl raw_crud::Delete for Todo {
    fn delete(id: Uuid, conn: &PgConnection) -> QueryResult<()> {
        super::Item::delete::<Self>(id, conn)
    }
}

impl Todo {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_todo);
        cfg.service(routes::find_todo);
        cfg.service(routes::update_todo);
        cfg.service(routes::delete_todo);
    }
}

mod routes {
    use actix_web::{
        delete, get, patch, post, web, Error, HttpRequest, HttpResponse,
    };
    use uuid::Uuid;

    use crate::{items::crud2::crud2http, DbPool};

    use super::{NewTodo, Todo, UpdateTodo};

    #[post("/todos")]
    pub async fn create_todo(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        form: web::Json<NewTodo>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::create::<Todo, _>(form.into_inner(), user, &pool).await
    }

    #[get("/todos/{id}")]
    pub async fn find_todo(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::find::<Todo>(id.into_inner(), user, &pool).await
    }

    #[patch("/todos/{id}")]
    pub async fn update_todo(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
        form: web::Json<UpdateTodo>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();

        crud2http::update::<Todo, _>(
            id.into_inner(),
            form.into_inner(),
            user,
            &pool,
        )
        .await
    }

    #[delete("/todos/{id}")]
    pub async fn delete_todo(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        let user = req.extensions().get().cloned().unwrap();
        crud2http::delete::<Todo>(id.into_inner(), user, &pool).await
    }
}
