-- Your SQL goes here
create table user_tokens (
  id SERIAL PRIMARY KEY,
  client_id UUID NOT NULL REFERENCES clients (uuid),
  account_id UUID NOT NULL REFERENCES accounts (uuid),
  refresh_id UUID NOT NULL
);
  
