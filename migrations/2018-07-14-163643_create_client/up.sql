-- Your SQL goes here
CREATE TABLE clients (
  id SERIAL PRIMARY KEY,
  uuid UUID UNIQUE,
  name TEXT,
  email TEXT
);
