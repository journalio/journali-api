use diesel::{self, prelude::*};
use uuid::Uuid;

use crate::models::{NewPage, Page};
use crate::repositories::items_repository;
use crate::schema::pages;

pub fn create(new_page: &NewPage, conn: &PgConnection) -> QueryResult<Page> {
    let page = Page {
        id: Uuid::new_v4(),
        item_type: 100,
        title: new_page.title.clone(),
    };
    items_repository::create(&page, conn)?;
    diesel::insert_into(pages::table).values(&page).get_result(conn)
}

pub fn get(id: Uuid, conn: &PgConnection) -> QueryResult<Page> {
    pages::table.filter(pages::id.eq(id)).get_result(conn)
}
