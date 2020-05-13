table! {
    items (id, item_type) {
        id -> Uuid,
        item_type -> Int2,
        parent_id -> Nullable<Uuid>,
        parent_type -> Nullable<Int2>,
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
    todo_items (id) {
        id -> Uuid,
        item_type -> Int2,
        title -> Text,
        is_checked -> Bool,
    }
}

table! {
    todos (id) {
        id -> Uuid,
        item_type -> Int2,
        title -> Text,
    }
}

allow_tables_to_appear_in_same_query!(items, pages, todo_items, todos,);
