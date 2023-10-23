-- This file should undo anything in `up.sql`
ALTER TABLE users
DROP COLUMN enabled,
DROP COLUMN created,
DROP COLUMN last_login;