-- Your SQL goes here
create table genre(
    id uuid primary key,
    name varchar(255) unique not null
);