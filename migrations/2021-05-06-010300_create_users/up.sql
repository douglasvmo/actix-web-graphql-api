-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
  id uuid DEFAULT uuid_generate_v4(),
  name VARCHAR(255) NOT NULL,
  email VARCHAR(80) UNIQUE NOT NULL,
  cpf_cnpj VARCHAR(14) UNIQUE NOT NULL,
  password VARCHAR(128) NOT NULL,
  role_id INTEGER,
  active BOOLEAN NOT NULL DEFAULT 'f',
  verification_payload VARCHAR(8),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  PRIMARY KEY (id)
);