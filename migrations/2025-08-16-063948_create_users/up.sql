-- Your SQL goes here
create type user_status as ENUM ('ACTIVE', 'BANNED', 'INACTIVE');

create table users (
    id uuid PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password varchar(255) NOT NULL,
    status user_status NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    country_id INT REFERENCES country(id) ON DELETE SET NULL ON UPDATE CASCADE
);