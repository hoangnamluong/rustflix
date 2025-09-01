-- Your SQL goes here
create type streaming_protocol as ENUM ('HLS', 'DASH');
create table asset (
    id serial primary key, 
    title_id int references title(id) on delete cascade,
    manifest_url streaming_protocol not null default 'HLS',
    subtitle_locales int references languages(id) on delete set null,
    audio_locales int references languages(id) on delete set null
);