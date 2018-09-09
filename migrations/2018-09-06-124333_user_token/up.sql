-- Your SQL goes here
create table user_tokens (
  id SERIAL PRIMARY KEY,
  client_id UUID NOT NULL REFERENCES clients (uuid),
  account_id UUID NOT NULL REFERENCES accounts (uuid),
  -- Refresh tokens are optional wrt a user token
  refresh_id UUID REFERENCES refresh_tokens (uuid)
);
