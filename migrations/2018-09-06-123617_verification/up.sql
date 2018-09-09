-- Your SQL goes here
create table verifications (
  id SERIAL PRIMARY KEY,
  verified_at TIMESTAMP,
  ip_address TEXT NOT NULL
);
