use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};

use crate::models::{Item, ItemLike};
use crate::schema::items;

pub fn create<T: ItemLike>(
    item_like: &T,
    conn: &PgConnection,
) -> QueryResult<Item> {
    let item = Item {
        id: item_like.id(),
        item_type: item_like.item_type(),
        ..Default::default()
    };
    diesel::insert_into(items::table).values(&item).get_result(conn)
}
