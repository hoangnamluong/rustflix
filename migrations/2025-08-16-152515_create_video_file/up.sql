-- Your SQL goes here
create table video_file (
    id serial primary key, 
    asset_id int references asset(id) on delete cascade,
    codec TEXT NOT NULL,
    container TEXT NOT NULL,
    width INT NOT NULL,
    height INT NOT NULL,
    bitrate_kbps INT NOT NULL
);