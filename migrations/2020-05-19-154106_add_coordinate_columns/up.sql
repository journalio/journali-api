ALTER TABLE todos
    ADD COLUMN coord_x int NOT NULL CHECK ( coord_x >= 0 ) DEFAULT 0,
    ADD COLUMN coord_y int NOT NULL CHECK (coord_y >= 0)   DEFAULT 0;

ALTER TABLE text_fields
    ADD COLUMN coord_x int NOT NULL CHECK ( coord_x >= 0 ) DEFAULT 0,
    ADD COLUMN coord_y int NOT NULL CHECK (coord_y >= 0)   DEFAULT 0;
