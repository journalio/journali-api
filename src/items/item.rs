use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::schema::items;

use super::reex_diesel::*;
use super::{ItemLike, ItemType};

#[derive(Insertable, Queryable, Copy, Clone)]
pub struct Item {
    pub(crate) id: Uuid,
    pub(crate) item_type: ItemType,
    pub(crate) parent_id: Option<Uuid>,
    pub(crate) parent_type: Option<ItemType>,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
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
        }
    }
}

impl Item {
    pub(super) fn create(&self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(items::table).values(self).get_result(conn)
    }
}
