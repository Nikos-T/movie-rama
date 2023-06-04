-- Your SQL goes here

CREATE TYPE VOTE AS ENUM ('positive', 'negative');

CREATE TABLE votes (
    user_id INTEGER NOT NULL,
    movie_id INTEGER NOT NULL,
    vote VOTE NOT NULL,
    PRIMARY KEY (user_id, movie_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (movie_id) REFERENCES movies(id)
);

