-- Your SQL goes here
create table playback_progress (
    user_id int references users(id) on delete cascade,
    asset_id int references asset(id) on delete cascade,
    position_ms int not null,
    updated_at timestamp default now(),
    primary key (user_id, asset_id)
);