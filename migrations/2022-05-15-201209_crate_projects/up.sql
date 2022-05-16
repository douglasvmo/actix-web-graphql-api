-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS projects (
    id uuid DEFAULT uuid_generate_v4(),
    owner_id uuid NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS users_to_projects (
    user_id uuid,
    projects_id uuid,
    PRIMARY KEY (user_id, projects_id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (projects_id) REFERENCES projects (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS tasks (
    id uuid DEFAULT uuid_generate_v4(),
    projects_id uuid,
    name VARCHAR(80) NOT NULL,
    description VARCHAR(255) NOT NULL,
    done BOOLEAN NOT NULL DEFAULT 'f',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY (projects_id) REFERENCES projects(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS goals (
    id uuid DEFAULT uuid_generate_v4(),
    projects_id uuid,
    name VARCHAR(80) NOT NULL,
    description VARCHAR(255) NOT NULL,
    deadline TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id),
    FOREIGN KEY(projects_id) REFERENCES projects(id) ON DELETE CASCADE
);