-- Your SQL goes here
create table genre(
    id SERIAL primary key,
    name varchar(255) unique not null
);