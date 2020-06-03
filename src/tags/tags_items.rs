use crate::schema::tags_items;
use diesel::prelude::*;
use diesel::QueryResult;
use serde::Serialize;
use uuid::Uuid;

#[derive(Queryable, Serialize, Insertable)]
pub struct TagsItem {
    pub tag_id: Uuid,
    pub item_id: Uuid,
    pub item_type: i16,
}

impl TagsItem {
    pub(super) fn create(&self, conn: &PgConnection) -> QueryResult<Self> {
        diesel::insert_into(tags_items::table).values(self).get_result(conn)
    }
}
