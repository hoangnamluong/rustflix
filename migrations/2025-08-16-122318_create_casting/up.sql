-- Your SQL goes here
create table casting(
    title_id uuid references title(id) on delete cascade,
    filmmaker_id uuid references filmmaker(id) on delete cascade,
    role_id uuid references casting_role(id) on delete set null,
    character_name varchar(255),
    primary key (title_id, filmmaker_id, role_id)
);