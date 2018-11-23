-- Your SQL goes here
create table access_tokens (
  id SERIAL PRIMARY KEY,
  jwt TEXT NOT NULL, 
  expires_in INTEGER NOT NULL,
  user_id INTEGER NOT NULL
);
