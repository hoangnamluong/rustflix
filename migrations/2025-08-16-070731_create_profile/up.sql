-- Your SQL goes here
create type maturity_rating as ENUM ('G', 'PG', 'PG-13/12', 'R/15', 'NC-17/18', 'TV-MA');

create table profile (
    id SERIAL PRIMARY KEY,
    user_id int NOT NULL REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    name VARCHAR(255) NOT NULL DEFAULT 'NAME',
    maturity_rating_max maturity_rating NOT NULL DEFAULT 'NC-17/18',
    avatar_url VARCHAR(255),
    language INT NOT NULL REFERENCES languages(id)
);

create or replace function set_default_language()
returns trigger as $$
BEGIN
    if NEW.language is null then
        select id into NEW.language from languages where code = 'en';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

create trigger set_profile_default_language
before insert on profile
for each row 
execute function set_default_language();