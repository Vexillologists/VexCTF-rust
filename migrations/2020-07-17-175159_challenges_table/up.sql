-- Create a table for challenges
-- The solves of `teams` will be
-- added in a different migration
-- as well as the categories

CREATE TABLE challenges (
  id SERIAL PRIMARY KEY NOT NULL,
  challenge_name CITEXT UNIQUE NOT NULL,
  challenge_description TEXT NOT NULL,
  hints TEXT ARRAY NOT NULL DEFAULT '{}',
  flag_case_sensitive BOOLEAN NOT NULL DEFAULT 'f',
  flag VARCHAR(128) NOT NULL,
  points INTEGER NOT NULL DEFAULT 0
);