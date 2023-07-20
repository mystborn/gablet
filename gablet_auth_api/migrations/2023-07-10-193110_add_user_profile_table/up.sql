DROP TABLE IF EXISTS user_profiles;
CREATE TABLE user_profiles(
    id SERIAL PRIMARY KEY,
    user_id SERIAL UNIQUE NOT NULL,
    about TEXT,
    avatar VARCHAR(255),
    banner VARCHAR(255),
    gender VARCHAR(30),
    country VARCHAR(50),
    lang VARCHAR(10),

    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);

INSERT INTO user_profiles (user_id)
SELECT id FROM users;