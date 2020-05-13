CREATE TABLE text_fields
(
    id        uuid     NOT NULL,
    item_type smallint NOT NULL DEFAULT 300 CHECK (item_type = 300),

    text      text     NOT NULL,

    PRIMARY KEY (id, item_type),
    FOREIGN KEY (id, item_type) REFERENCES items (id, item_type) ON DELETE CASCADE
)
