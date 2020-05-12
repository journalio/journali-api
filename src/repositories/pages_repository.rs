use diesel::{self, prelude::*};
use uuid::Uuid;

use crate::models::{NewPage, Page};
use crate::schema::pages;

pub fn create(page: &NewPage, conn: &PgConnection) -> QueryResult<Page> {
    diesel::insert_into(pages::table).values(page).get_result(conn)
}

pub fn get(id: Uuid, conn: &PgConnection) -> QueryResult<Page> {
    pages::table.find(id).get_result::<Page>(&conn)
}
