-- Your SQL goes here
DROP TABLE IF EXISTS books;
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    description Text,
    approved BOOLEAN NOT NULL DEFAULT FALSE,
    small_thumbnail VARCHAR(255),
    big_thumbnail VARCHAR(255),

    CONSTRAINT author_id FOREIGN KEY(id) REFERENCES users(id)
);