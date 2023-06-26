-- Your SQL goes here

CREATE TYPE user_level AS ENUM ('user', 'superuser', 'mod', 'admin');

DROP TABLE IF EXISTS users;
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(128) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    name VARCHAR(128) NOT NULL DEFAULT '',
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    level user_level NOT NULL DEFAULT 'user'
);