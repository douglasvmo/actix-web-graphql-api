-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS auths (
    id uuid DEFAULT uuid_generate_v4(),
    user_id uuid NOT NULL,
    payload VARCHAR(8) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    CONSTRAINT fk_user_auths
      FOREIGN KEY(user_id) 
	  REFERENCES users(id)
);