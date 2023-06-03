-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    -- hashed password using Hmac Sha256
    -- The length should be exactly equal to the hashing output
    password_hash CHAR(119) NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL
);

