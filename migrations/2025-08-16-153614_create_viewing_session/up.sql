-- Your SQL goes here
create table viewing_session (
    id uuid primary key DEFAULT gen_random_uuid (),
    user_id uuid references users(id) on delete set null,
    asset_id uuid references asset(id) on delete set null,
    started_at timestamp default now(), 
    ended_at timestamp
);