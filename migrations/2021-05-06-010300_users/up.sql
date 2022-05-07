-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
  id uuid DEFAULT uuid_generate_v4(),
  name VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  cpf_cnpj VARCHAR(14) NOT NULL,
  password VARCHAR NOT NULL,
  role_id INTEGER,
  active BOOLEAN NOT NULL DEFAULT 'f',
  verification_code VARCHAR(8),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);