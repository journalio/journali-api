-- Your SQL goes here
CREATE TABLE tags
(
    id       uuid NOT NULL PRIMARY KEY DEFAULT uuid_generate_v4(),
    name     text NOT NULL,
    color    text NOT NULL,
    owner_id uuid NOT NULL,

    FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE tags_items
(
    tag_id    uuid     NOT NULL,
    item_id   uuid     NOT NULL,
    item_type smallint NOT NULL,

    PRIMARY KEY (tag_id, item_id, item_type),
    FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE,
    FOREIGN KEY (item_id, item_type) REFERENCES items (id, item_type) ON DELETE CASCADE
);
