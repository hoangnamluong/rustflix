-- Your SQL goes here
create table title_genre(
    title_id int references title(id) on delete cascade on update cascade,
    genre_id int references genre(id) on delete cascade on update cascade,

    primary key (title_id, genre_id) 
);