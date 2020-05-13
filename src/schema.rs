table! {
    items (id, item_type) {
        id -> Uuid,
        item_type -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    pages (id, item_type) {
        id -> Uuid,
        item_type -> Int2,
        title -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        full_name -> Text,
        password_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(items, pages, users,);
