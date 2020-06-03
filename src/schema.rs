table! {
    items (id, item_type) {
        id -> Uuid,
        item_type -> Int2,
        parent_id -> Nullable<Uuid>,
        parent_type -> Nullable<Int2>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        owner_id -> Uuid,
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
    tags (id) {
        id -> Uuid,
        name -> Text,
        color -> Text,
        owner_id -> Uuid,
    }
}

table! {
    tags_items (tag_id, item_id, item_type) {
        tag_id -> Uuid,
        item_id -> Uuid,
        item_type -> Int2,
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

joinable!(items -> users (owner_id));
joinable!(tags -> users (owner_id));
joinable!(tags_items -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    items,
    pages,
    tags,
    tags_items,
    text_fields,
    todo_items,
    todos,
    users,
);
