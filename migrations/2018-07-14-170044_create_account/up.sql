-- Your SQL goes here
CREATE TABLE accounts (
  id SERIAL PRIMARY KEY NOT NULL,
  uuid UUID UNIQUE,
  username TEXT,
  password TEXT,
  email TEXT UNIQUE,
  verification_id INTEGER REFERENCES verifications (id)
)
