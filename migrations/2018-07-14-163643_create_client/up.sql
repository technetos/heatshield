-- Your SQL goes here
CREATE TYPE clientkind AS ENUM ('heatsheild_wasm');

CREATE TABLE clients (
  id SERIAL PRIMARY KEY,
  kind clientkind NOT NULL,
  name TEXT NOT NULL,
  email TEXT NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT 't'
)
