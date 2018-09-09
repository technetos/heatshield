-- Your SQL goes here
create table salts (
  id SERIAL PRIMARY KEY NOT NULL,
  salt TEXT UNIQUE NOT NULL
);
