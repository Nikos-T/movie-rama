-- Your SQL goes here

CREATE TABLE movies (
    id SERIAL PRIMARY KEY,
    posted_by INTEGER REFERENCES users(id) NOT NULL,
    posted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    title VARCHAR(255) NOT NULL UNIQUE,
    description TEXT NOT NULL
);

