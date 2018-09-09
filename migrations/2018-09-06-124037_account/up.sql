-- Your SQL goes here
create table accounts (
  id SERIAL PRIMARY KEY NOT NULL,
  uuid UUID UNIQUE,
  username TEXT UNIQUE,
  password TEXT,
  email TEXT UNIQUE,
  verification_id INTEGER REFERENCES verifications (id)
);
