-- Create a table for teams
-- The Team ID of `users` as well
-- as solves will be added in a 
-- different migration

CREATE TABLE teams (
  id SERIAL PRIMARY KEY NOT NULL,
  team_name CITEXT UNIQUE NOT NULL,
  invite_code_hash VARCHAR(128) NOT NULL,
  points INTEGER NOT NULL DEFAULT 0
);