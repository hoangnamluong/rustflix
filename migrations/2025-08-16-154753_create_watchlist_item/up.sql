-- Your SQL goes here
create table watchlist_item (
    user_id uuid references users(id) on delete cascade,
    title_id uuid references title(id) on delete cascade,
    added_at timestamp default now(),
    primary key (user_id, title_id)
);