-- Your SQL goes here
create table casting(
    title_id int references title(id) on delete cascade,
    filmmaker_id int references filmmaker(id) on delete cascade,
    role_id int references casting_role(id) on delete set null,
    character_name varchar(255),
    primary key (title_id, filmmaker_id, role_id)
);