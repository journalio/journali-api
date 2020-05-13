create table users (
    id uuid not null primary key default uuid_generate_v4(),
    username text not null,
    password text not null
);