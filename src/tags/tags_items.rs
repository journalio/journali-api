use crate::schema::tags_items;
use crate::users::user::User;
use diesel::prelude::*;
use diesel::QueryResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Serialize, Insertable)]
pub struct TagsItem {
    pub tag_id: Uuid,
    pub item_id: Uuid,
    pub item_type: i16,
}

#[derive(Deserialize, Serialize)]
pub struct TagsItemRequest {
    id: Uuid,
    item_type: i16,
}

impl TagsItem {
    pub fn find_all(
        tag_id: Uuid,
        conn: &PgConnection,
    ) -> QueryResult<Vec<Self>> {
        tags_items::table
            .filter(tags_items::columns::tag_id.eq(tag_id))
            .load(conn)
    }

    pub fn add_items(
        tag_id: Uuid,
        item_ids: Vec<TagsItemRequest>,
        user: User,
        connection: &PgConnection,
    ) -> QueryResult<usize> {
        // VALIDATION!
        let _ = user;
        let insert_data = item_ids
            .into_iter()
            .map(|tagsitem_request| TagsItem {
                tag_id,
                item_id: tagsitem_request.id,
                item_type: tagsitem_request.item_type,
            })
            .collect::<Vec<_>>();

        diesel::insert_into(tags_items::table)
            .values(&insert_data)
            .execute(connection)
    }
}
