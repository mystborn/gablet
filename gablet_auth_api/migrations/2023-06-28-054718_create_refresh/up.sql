-- Your SQL goes here
CREATE TABLE refresh_tokens(
    id SERIAL PRIMARY KEY,
    refresh_token TEXT NOT NULL DEFAULT '',
    username VARCHAR(50) UNIQUE NOT NULL,
    CONSTRAINT user_id FOREIGN KEY(id) REFERENCES users(id) ON DELETE CASCADE
);