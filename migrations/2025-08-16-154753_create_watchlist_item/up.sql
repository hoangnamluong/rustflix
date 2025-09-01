-- Your SQL goes here
create table watchlist_item (
    user_id int references users(id) on delete cascade,
    title_id int references title(id) on delete cascade,
    added_at timestamp default now(),
    primary key (user_id, title_id)
);