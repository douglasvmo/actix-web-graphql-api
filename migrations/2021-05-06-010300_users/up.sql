-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  email TEXT NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 'f'
)