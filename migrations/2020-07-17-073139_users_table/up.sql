-- Create a table for users
-- Team ID will be added later once the teams table
-- is created.

CREATE TABLE users (
  id SERIAL PRIMARY KEY NOT NULL,
  username CITEXT UNIQUE NOT NULL,
  password_hash VARCHAR(128) NOT NULL,
  email CITEXT UNIQUE NOT NULL,
  email_confirmed BOOLEAN NOT NULL DEFAULT 'f',
  admin BOOLEAN NOT NULL DEFAULT 'f'
);