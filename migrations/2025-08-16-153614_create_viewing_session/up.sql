-- Your SQL goes here
create table viewing_session (
    id serial primary key,
    user_id int references users(id) on delete set null,
    asset_id int references asset(id) on delete set null,
    started_at timestamp default now(), 
    ended_at timestamp
);