-- Your SQL goes here
create table rating(
    id uuid primary key DEFAULT gen_random_uuid (),
    user_id uuid references users(id) on delete set null,
    title_id uuid references title(id) on delete cascade,
    score smallint not null default 5 check (score between 1 and 5),
    rated_at timestamp default now(),
    unique (user_id, title_id)
);