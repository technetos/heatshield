-- Your SQL goes here
create table refresh_tokens (
  id SERIAL PRIMARY KEY,
  uuid UUID NOT NULL UNIQUE
);
