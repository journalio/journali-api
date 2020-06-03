use crate::schema::tags_items;
use uuid::Uuid;
use serde::Serialize;

#[derive(Queryable, Serialize, Insertable)]
pub struct TagsItem {
    pub tag_id: Uuid,
    pub item_id: Uuid,
    pub item_type: i16,
}
