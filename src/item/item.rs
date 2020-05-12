use super::{ItemLike, ItemType};
use crate::schema::items;

use uuid::Uuid;

use super::reex_diesel::*;
use chrono::{DateTime, Utc};

#[derive(Insertable, Queryable, Copy, Clone)]
pub struct Item {
    pub(crate) id: Uuid,
    pub(crate) item_type: i16,
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

    fn as_item(&self) -> Item {
        *self
    }
}

impl Default for Item {
    fn default() -> Self {
        Item {
            id: Uuid::default(),
            item_type: 0,
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
