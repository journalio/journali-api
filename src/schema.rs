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
    text_fields (id, item_type) {
        id -> Uuid,
        item_type -> Int2,
        text -> Text,
        coord_x -> Int4,
        coord_y -> Int4,
    }
}

table! {
    todo_items (id, item_type) {
        id -> Uuid,
        item_type -> Int2,
        title -> Text,
        is_checked -> Bool,
    }
}

table! {
    todos (id, item_type) {
        id -> Uuid,
        item_type -> Int2,
        title -> Text,
        coord_x -> Int4,
        coord_y -> Int4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    items,
    pages,
    text_fields,
    todo_items,
    todos,
    users,
);
