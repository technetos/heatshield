-- Your SQL goes here
CREATE TABLE client_tokens (
  id SERIAL PRIMARY KEY,
  client_id UUID NOT NULL REFERENCES clients (uuid)
);
