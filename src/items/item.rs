use core::convert::AsRef;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::crud::Find;
use super::reex_diesel::*;
use super::{ItemLike, ItemType};
use crate::items::page::Page;
use crate::items::text_field::TextField;
use crate::items::todo::Todo;
use crate::items::todo_item::TodoItem;
use crate::items::Items;
use crate::schema::items;
use crate::users::user::User;

#[derive(
    Identifiable, Associations, Insertable, Queryable, Copy, Clone, Serialize,
)]
#[belongs_to(User, foreign_key = "owner_id")]
pub struct Item {
    pub(crate) id: Uuid,
    pub(crate) item_type: ItemType,
    pub(crate) parent_id: Option<Uuid>,
    pub(crate) parent_type: Option<ItemType>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) owner_id: Uuid,
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
            owner_id: Uuid::default(),
        }
    }
}

impl Item {
    pub fn has_owner<T: super::TypeMarker>(
        id: Uuid,
        owner: Uuid,
        conn: &PgConnection,
    ) -> bool {
        use diesel::dsl::{exists, select};

        select(exists(
            items::table
                .filter(items::owner_id.eq(owner))
                .filter(items::id.eq(id))
                .filter(items::item_type.eq(T::TYPE as i16)),
        ))
        .get_result(conn)
        .unwrap_or(false)
    }

    pub(super) fn delete<T>(id: Uuid, conn: &PgConnection) -> QueryResult<()>
    where
        T: super::TypeMarker,
    {
        diesel::delete(
            items::table
                .filter(items::columns::id.eq(id))
                .filter(items::item_type.eq(T::TYPE as i16)),
        )
        .get_result::<Item>(conn)
        .map(drop)
    }

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

    pub(super) fn find_by_parent(
        pid: &Uuid,
        conn: &PgConnection,
    ) -> QueryResult<Vec<Items>> {
        items::table.filter(items::parent_id.eq(pid)).load::<Item>(conn).map(
            |items| {
                items
                    .into_iter()
                    .map(|item| match item.item_type {
                        200 => Items::Todo(
                            Todo::find(item.id, &conn)
                                .expect("Failed to load todo"),
                        ),
                        210 => Items::TodoItem(
                            TodoItem::find(item.id, &conn)
                                .expect("Failed to load todo item"),
                        ),

                        100 => Items::Page(
                            Page::find((item.id, unreachable!()), &conn)
                                .expect("Failed to load todo item"),
                        ),
                        300 => Items::TextField(
                            TextField::find((item.id, unreachable!()), &conn)
                                .expect("Failed to load todo item"),
                        ),
                        _ => panic!("wtf"),
                    })
                    .rev()
                    .collect()
            },
        )
    }
}

#[derive(Clone)]
pub struct OwnedItem<Item> {
    /// The user `Item` belongs to
    pub user: User,

    /// The item in question
    pub item: Item,
}

impl<Item> AsRef<Item> for OwnedItem<Item> {
    fn as_ref(&self) -> &Item {
        &self.item
    }
}

impl<T> OwnedItem<T> {
    pub const fn new(user: User, item: T) -> Self {
        Self { user, item }
    }

    pub fn into_item(&self) -> Item
    where
        T: ItemLike,
    {
        let Self { user, item } = self;
        let mut item = item.as_new_item();

        item.owner_id = user.id;
        item
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
        cfg.service(routes::update_item_parent)
            .service(routes::get_items_by_parent);
    }
}

mod routes {
    use actix_web::{get, patch, web, Error, HttpResponse};
    use serde::Deserialize;
    use uuid::Uuid;

    use crate::{
        database::exec_on_pool, items::item::UpdateParentRequest,
        utils::responsable::Responsable, DbPool,
    };

    use super::Item;

    #[derive(Deserialize)]
    pub struct ItemsByParentRequest {
        parent_id: Uuid,
    }

    #[get("/items")]
    pub async fn get_items_by_parent(
        pool: web::Data<DbPool>,
        query: web::Query<ItemsByParentRequest>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(&pool, move |conn| {
            Item::find_by_parent(&query.parent_id, &conn)
        })
        .await
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|_| HttpResponse::InternalServerError().finish().into())
    }

    #[patch("/items/{id}")]
    pub async fn update_item_parent(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        form: web::Json<UpdateParentRequest>,
    ) -> Result<HttpResponse, Error> {
        exec_on_pool(&pool, move |conn| {
            Item::update_parent(&id.into_inner(), &form, &conn)
        })
        .await
        .into_response()
    }
}
