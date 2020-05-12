use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{items, pages};

pub trait ItemLike {
    fn id(&self) -> Uuid;
    fn item_type(&self) -> i16;
}

#[derive(Insertable, Queryable)]
pub struct Item {
    pub id: Uuid,
    pub item_type: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewPage {
    pub title: String,
}

#[derive(Queryable, Serialize, Insertable)]
pub struct Page {
    pub id: Uuid,
    pub item_type: i16,
    pub title: String,
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

impl ItemLike for Page {
    fn id(&self) -> Uuid {
        self.id
    }
    fn item_type(&self) -> i16 {
        100
    }
}
