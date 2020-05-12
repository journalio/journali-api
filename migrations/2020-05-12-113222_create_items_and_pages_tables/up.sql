CREATE TABLE items
(
    id         uuid        NOT NULL,
    item_type  smallint    NOT NULL,

    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),

    PRIMARY KEY (id, item_type)
);

CREATE TABLE pages
(
    id        uuid     NOT NULL,
    item_type smallint NOT NULL DEFAULT 100 CHECK (item_type = 100), -- item type is always 100(page)

    title     text     NOT NULL,

    PRIMARY KEY (id, item_type),
    FOREIGN KEY (id, item_type) REFERENCES items (id, item_type)
)
