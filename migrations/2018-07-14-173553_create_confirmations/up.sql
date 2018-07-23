-- Your SQL goes here
CREATE TABLE confirmations (
  id SERIAL PRIMARY KEY,
  code TEXT NOT NULL UNIQUE
)
