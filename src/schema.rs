table! {
    items (id, item_type) {
        id -> Uuid,
        item_type -> Itemtype,
    }
}

table! {
    pages (id, item_type) {
        id -> Uuid,
        item_type -> Itemtype,
        title -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    items,
    pages,
);
