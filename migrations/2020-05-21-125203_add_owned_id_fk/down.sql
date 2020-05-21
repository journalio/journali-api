-- This file should undo anything in `up.sql`
ALTER TABLE items
    DROP CONSTRAINT users_fk,
    DROP COLUMN owner_id;