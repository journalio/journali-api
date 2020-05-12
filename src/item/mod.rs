use uuid::Uuid;

/// Reexport commonly used diesel
/// namespaces
mod reex_diesel {
    pub use diesel::{pg::PgConnection, prelude::*, QueryResult};
}

pub mod item;
pub mod page;

use item::Item;

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
