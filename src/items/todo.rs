use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    items::{ItemTypeNames, TypeMarker},
    schema::todos,
};

use super::{
    crud::{Create, Delete, Find, Update},
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
    pub page_id: Uuid,
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
        Some(self.page_id)
    }

    fn parent_type(&self) -> Option<i16> {
        Some(ItemTypeNames::Page as i16)
    }
}

impl Create for Todo {
    type Create = NewTodo;

    fn create(new_todo: &NewTodo, conn: &PgConnection) -> QueryResult<Self> {
        let item = new_todo.as_new_item();
        let todo = Self {
            id: item.id,
            item_type: item.item_type,
            title: new_todo.title.clone(),
            coord_x: new_todo.coord_x,
            coord_y: new_todo.coord_y,
        };

        item.create(conn)?;
        diesel::insert_into(todos::table).values(&todo).get_result(conn)
    }
}

impl Find for Todo {
    fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        todos::table
            .filter(todos::columns::id.eq(id))
            .filter(todos::item_type.eq(Self::TYPE as i16))
            .get_result(conn)
    }
}

impl Update for Todo {
    type Update = UpdateTodo;

    fn update(
        id: Uuid,
        update_todo: &UpdateTodo,
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

impl Delete for Todo {
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
    use actix_web::{delete, get, patch, post, web, Error, HttpResponse};
    use uuid::Uuid;

    use crate::{items::crud::Crudder, DbPool};

    use super::{NewTodo, Todo, UpdateTodo};

    #[post("/todos")]
    pub async fn create_todo(
        pool: web::Data<DbPool>,
        form: web::Json<NewTodo>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Todo>::create(form.into_inner(), &pool).await
    }

    #[get("/todos/{id}")]
    pub async fn find_todo(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Todo>::find(id.into_inner(), &pool).await
    }

    #[patch("/todos/{id}")]
    pub async fn update_todo(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        form: web::Json<UpdateTodo>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Todo>::update(id.into_inner(), form.into_inner(), &pool).await
    }

    #[delete("/todos/{id}")]
    pub async fn delete_todo(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
    ) -> Result<HttpResponse, Error> {
        Crudder::<Todo>::delete(id.into_inner(), &pool).await
    }
}
