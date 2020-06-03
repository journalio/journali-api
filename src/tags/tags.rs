use crate::schema::tags;
use uuid::Uuid;
use serde::Serialize;

#[derive(Queryable, Serialize, Insertable)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub owner_id: Uuid,
}
