-- Your SQL goes here
CREATE TABLE user_profiles(
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    about TEXT,
    avatar VARCHAR(255),
    banner VARCHAR(255),
    gender VARCHAR(30),
    country VARCHAR(50),
    lang VARCHAR(10)
);

INSERT INTO user_profiles (user_id)
SELECT id FROM users;