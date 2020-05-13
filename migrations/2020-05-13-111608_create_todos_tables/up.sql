CREATE TABLE todos
(
    id        uuid     NOT NULL,
    item_type smallint NOT NULL DEFAULT 200 CHECK (item_type = 200), -- item type is always 200(to do)

    title     text     NOT NULL,

    PRIMARY KEY (id, item_type),
    FOREIGN KEY (id, item_type) REFERENCES items (id, item_type) ON DELETE CASCADE
);

CREATE TABLE todo_items
(
    id         uuid     NOT NULL,
    item_type  smallint NOT NULL DEFAULT 210 CHECK (item_type = 210), -- item type is always 210(todo_item)

    title      text     NOT NULL,
    is_checked bool     NOT NULL DEFAULT FALSE,

    PRIMARY KEY (id, item_type),
    FOREIGN KEY (id, item_type) REFERENCES items (id, item_type) ON DELETE CASCADE
);
