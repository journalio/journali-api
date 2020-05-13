table! {
    items (id, item_type) {
        id -> Uuid,
        item_type -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    pages (id) {
        id -> Uuid,
        item_type -> Int2,
        title -> Text,
    }
}

table! {
    todo_items (id) {
        id -> Uuid,
        item_type -> Int2,
        todo_id -> Uuid,
        title -> Text,
        is_checked -> Bool,
    }
}

table! {
    todos (id) {
        id -> Uuid,
        item_type -> Int2,
        page_id -> Uuid,
        title -> Text,
    }
}

joinable!(todo_items -> todos (todo_id));
joinable!(todos -> pages (page_id));

allow_tables_to_appear_in_same_query!(items, pages, todo_items, todos,);
