CREATE TYPE itemtype AS enum ('page');

CREATE TABLE items
(
    id         uuid        NOT NULL,
    item_type  itemtype    NOT NULL,

    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),

    PRIMARY KEY (id, item_type)
);

CREATE TABLE pages
(
    id        uuid     NOT NULL,
    item_type itemtype NOT NULL DEFAULT 'page' CHECK (item_type = 'page'), -- item type is always 100

    title     text     NOT NULL,

    PRIMARY KEY (id, item_type),
    FOREIGN KEY (id, item_type) REFERENCES items (id, item_type)
)
