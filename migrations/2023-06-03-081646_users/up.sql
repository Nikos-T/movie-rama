-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    -- bcrypt hashed password
    -- There are more advanced ways for this for extra security
    -- TODO: use OAUTH
    password_hash CHAR(60) NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL
);

