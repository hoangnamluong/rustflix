-- Your SQL goes here
create table genre(
    id uuid primary key DEFAULT gen_random_uuid (),
    name varchar(255) unique not null
);