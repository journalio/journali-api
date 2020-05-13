//! This module contains all item definitions used by the application
//!
//! The several items used are listed below:
//! - [`Item`](item/struct.Item.html)
//! - [`Page`](page/struct.Page.html)

use uuid::Uuid;

use item::Item;

/// Reexport commonly used diesel
/// namespaces
mod reex_diesel {
    pub use diesel::{pg::PgConnection, prelude::*, QueryResult};
}

pub mod item;
pub mod page;
pub mod todo;
pub mod todo_item;

pub type ItemType = i16;

pub trait ItemLike {
    fn id(&self) -> Uuid;
    fn item_type(&self) -> ItemType;

    fn as_item(&self) -> Item {
        Item {
            id: self.id(),
            item_type: self.item_type(),
            ..Default::default()
        }
    }
}

#[repr(i16)]
pub enum ItemTypeNames {
    Page = 100,
    Todo = 200,
    TodoItem = 210,
}
