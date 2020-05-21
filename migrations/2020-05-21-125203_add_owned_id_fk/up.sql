-- Your SQL goes here
ALTER TABLE items
    ADD COLUMN owner_id uuid NOT NULL,
    ADD CONSTRAINT users_fk FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE;