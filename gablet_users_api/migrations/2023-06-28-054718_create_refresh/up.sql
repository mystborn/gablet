-- Your SQL goes here
CREATE TABLE refresh_tokens(
    id SERIAL PRIMARY KEY,
    refresh_token TEXT NOT NULL DEFAULT '',
    username VARCHAR(50) NOT NULL
);