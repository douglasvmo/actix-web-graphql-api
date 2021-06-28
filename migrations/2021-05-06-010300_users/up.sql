-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id uuid DEFAULT uuid_generate_v4(),
  name VARCHAR NOT NULL,
  email TEXT NOT NULL,
  password VARCHAR NOT NULL,
  access_type VARCHAR NOT NULL DEFAULT 'none',
  active BOOLEAN NOT NULL DEFAULT 'f',
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);