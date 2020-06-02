//! This module contains all item definitions used by the application
//!
//! The several items used are listed below:
//! - [`Item`](item/struct.Item.html)
//! - [`Page`](page/struct.Page.html)

use serde::Serialize;
use uuid::Uuid;

use item::Item;

use crate::items::page::Page;
use crate::items::text_field::TextField;
use crate::items::todo::Todo;
use crate::items::todo_item::TodoItem;

/// Reexport commonly used diesel
/// namespaces
mod reex_diesel {
    pub use diesel::{pg::PgConnection, prelude::*, QueryResult};
}

pub mod crud;
pub mod crud2;
pub mod item;
pub mod page;
pub mod text_field;
pub mod todo;
pub mod todo_item;

pub type ItemType = i16;

pub trait ItemLike {
    fn id(&self) -> Uuid;
    fn item_type(&self) -> ItemType;
    fn parent_id(&self) -> Option<Uuid>;
    fn parent_type(&self) -> Option<ItemType>;

    fn as_item(&self) -> Item {
        Item {
            id: self.id(),
            item_type: self.item_type(),
            parent_id: self.parent_id(),
            parent_type: self.parent_type(),
            ..Default::default()
        }
    }

    fn as_new_item(&self) -> Item {
        Item { id: Uuid::new_v4(), ..self.as_item() }
    }
}

pub trait TypeMarker {
    const TYPE: ItemTypeNames;
}

#[repr(i16)]
pub enum ItemTypeNames {
    Page = 100,
    Todo = 200,
    TodoItem = 210,
    TextField = 300,
}

#[derive(Serialize)]
pub enum Items {
    Page(Page),
    Todo(Todo),
    TodoItem(TodoItem),
    TextField(TextField),
}

pub trait FromPartial<T> {
    fn from_partial(item: &Item, incomplete: T) -> Self;
}

use crate::items::item::OwnedItem;

pub trait Create
where
    <<Self as Create>::Table as diesel::QuerySource>::FromClause:
        diesel::query_builder::QueryFragment<diesel::pg::Pg>,
    Self: diesel::Insertable<<Self as Create>::Table>,
    Self::Values: diesel::insertable::CanInsertInSingleQuery<diesel::pg::Pg>
        + diesel::query_builder::QueryFragment<diesel::pg::Pg>,
    Self: FromPartial<<Self as Create>::Create>,
{
    type Table: diesel::associations::HasTable;
    type Create: Clone + ItemLike;
    fn create<C>(
        model: OwnedItem<Self::Create>,
        conn: &diesel::PgConnection,
    ) -> diesel::QueryResult<Self> {
        let new_create_item = model.as_ref().clone();
        let new_item = model.into_item();

        let this = Self::from_partial(&new_item, new_create_item);

        new_item.create(conn)?;
        diesel::insert_into(Self::Table::table()).values(this).execute(conn)
    }
}
#[macro_export]
macro_rules! impl_update {
    (for $item:ty {type Update = $create:ident; table = $table:ident}) => {
        impl crate::items::crud::Update for $item {
            type Update = crate::items::item::OwnedItem<$create>;

            fn update(
                id: Uuid,
                update: &Self::Update,
                conn: &PgConnection,
            ) -> QueryResult<Self> {
                let update_item = update.as_ref();
                if crate::items::item::Item::has_owner::<Self>(
                    id,
                    update.user.id,
                    conn,
                ) {
                    diesel::update(
                        $table::table
                            .filter($table::columns::id.eq(id))
                            .filter($table::item_type.eq(Self::TYPE as i16)),
                    )
                    .set(update_item)
                    .get_result(conn)
                } else {
                    Err(diesel::result::Error::NotFound)
                }
            }
        }
    };
}
