-- Your SQL goes here
create table title_genre(
    title_id uuid references title(id) on delete cascade on update cascade,
    genre_id uuid references genre(id) on delete cascade on update cascade,

    primary key (title_id, genre_id) 
);