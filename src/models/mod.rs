use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::pages;

#[derive(Deserialize, Insertable)]
#[table_name = "pages"]
pub struct NewPage {
    pub title: String,
}

#[derive(Queryable, Serialize)]
pub struct Page {
    pub id: Uuid,
    pub item_type: i16,
    pub title: String,
}
