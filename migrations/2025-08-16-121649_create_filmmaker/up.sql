-- Your SQL goes here
create table filmmaker(
    id serial primary key,
    name varchar(255) not null,
    bio text,
    birth_date date
);