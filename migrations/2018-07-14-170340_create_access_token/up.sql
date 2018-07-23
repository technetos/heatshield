-- Your SQL goes here
CREATE TABLE access_tokens (
  id SERIAL PRIMARY KEY,
  client_id INTEGER NOT NULL REFERENCES clients (id),
  enabled BOOLEAN NOT NULL DEFAULT 'f'
)
