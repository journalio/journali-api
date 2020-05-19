use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{items::ItemTypeNames, schema::todos};

use super::{
    crud::{Create, Find, Update},
    reex_diesel::*,
    ItemLike, ItemType,
};

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

#[derive(Deserialize, AsChangeset)]
#[table_name = "todos"]
pub struct UpdateTodo {
    title: String,
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

impl Create for Todo {
    type Create = NewTodo;

    fn create(new_todo: &NewTodo, conn: &PgConnection) -> QueryResult<Self> {
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

impl Find for Todo {
    fn find(id: Uuid, conn: &PgConnection) -> QueryResult<Self> {
        todos::table.filter(todos::columns::id.eq(id)).get_result(conn)
    }
}

impl Update for Todo {
    type Update = UpdateTodo;

    fn update(
        id: Uuid,
        update_todo: &UpdateTodo,
        conn: &PgConnection,
    ) -> QueryResult<Self> {
        diesel::update(todos::table.filter(todos::columns::id.eq(id)))
            .set(update_todo)
            .get_result(conn)
    }
}

impl Todo {
    pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(routes::create_todo);
        cfg.service(routes::find_todo);
        cfg.service(routes::update_todo);
    }
}

mod routes {
    use actix_web::{get, patch, post, web, Error, HttpResponse};
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
}
