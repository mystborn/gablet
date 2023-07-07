-- Your SQL goes here
ALTER TABLE refresh_tokens
ADD COLUMN base_uri VARCHAR(255) NOT NULL DEFAULT '';