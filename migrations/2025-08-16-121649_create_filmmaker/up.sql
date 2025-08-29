-- Your SQL goes here
create table filmmaker(
    id uuid primary key DEFAULT gen_random_uuid (),
    name varchar(255) not null,
    bio text,
    birth_date date
);