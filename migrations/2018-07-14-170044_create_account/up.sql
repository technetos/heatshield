-- Your SQL goes here
CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  username TEXT,
  password TEXT,
  email TEXT,
  enabled BOOLEAN DEFAULT 'f',
  verification_id INTEGER REFERENCES verifications (id)
)
