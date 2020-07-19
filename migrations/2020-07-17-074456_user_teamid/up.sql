-- Add team_id to `users` with a foreign key constraint
-- to `teams`

ALTER TABLE users
ADD team_id INTEGER;

ALTER TABLE users
ADD CONSTRAINT users_team_id_fkey
FOREIGN KEY (team_id) REFERENCES teams(id);