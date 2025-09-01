-- Your SQL goes here
create table title (
    id SERIAL primary key,
    name varchar(255) not null,
    synopsis text not null,
    release_year smallint not null,
    runtime_min smallint not null,
    age_rating maturity_rating not null,
    poster_url varchar(255) not null,
    hero_image_url varchar(255) not null,
    orginal_language_id int references languages(id) on delete set null on update cascade,
    is_active boolean not null default true
);