//! This module contains the `impl_create`, `impl_update`, `impl_find` and
//! `impl_delete` macros. These macros should be used when defining a new table
//! in the database.

#[macro_export]
macro_rules! impl_ops {
    (for $item:ty {
        const table = $table:ident;
    }) => {
        impl crate::items::crud::Create for $item {

            fn create(
                model: Self,
                conn: &diesel::PgConnection
            ) -> QueryResult<Self> {
                diesel::insert_into($table::table).values(mode).execute(conn) 
            }
        }

        impl crate::items::crud::Update for $item {

            fn update(id: Uuid, 
    }
}
#[macro_export]
macro_rules! impl_update {
    (for $item:ty {type Update = $create:ident; table = $table:ident}) => {
        impl crate::items::crud::Update for $item {
            type Update = crate::items::item::OwnedItem<$create>;

            fn update(
                id: Uuid,
                update: &Self::Update,
                conn: &PgConnection,
            ) -> QueryResult<Self> {
                let update_item = update.as_ref();
                if crate::items::item::Item::has_owner::<Self>(
                    id,
                    update.user.id,
                    conn,
                ) {
                    diesel::update(
                        $table::table
                            .filter($table::columns::id.eq(id))
                            .filter($table::item_type.eq(Self::TYPE as i16)),
                    )
                    .set(update_item)
                    .get_result(conn)
                } else {
                    Err(diesel::result::Error::NotFound)
                }
            }
        }
    };
}
