-- Your SQL goes here
create table casting_role (
    id uuid primary key DEFAULT gen_random_uuid (),
    name varchar(255) not null
);