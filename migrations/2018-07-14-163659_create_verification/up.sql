-- Your SQL goes here
CREATE TABLE verifications (
  id SERIAL PRIMARY KEY,
  verified_at TIMESTAMP,
  ip_address TEXT NOT NULL,
  confirmation_id INTEGER NOT NULL REFERENCES verifications (id)
)
  
