-- Your SQL goes here
CREATE TABLE clients (
  id SERIAL PRIMARY KEY,
  uuid UUID UNIQUE NOT NULL,
  name TEXT,
  email TEXT
);
